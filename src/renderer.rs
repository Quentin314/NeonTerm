use std::{fmt::Write, os::raw, thread};


#[derive(Debug, Clone)]
pub struct RawText {
    // WARNING: Do not use newlines or tabs in the text, it will break the rendering
    pub text: String,
    pub fg: (u8, u8, u8),
    pub bg: (u8, u8, u8),
    pub pos: (usize, usize) // Position relative to the window, for y position, 0 is the first row that entirely fits in the window
}
impl RawText {
    pub fn new(text: &str, fg: (u8, u8, u8), bg: (u8, u8, u8), pos: (usize, usize)) -> Self {
        RawText {
            text: text.to_string(),
            fg,
            bg,
            pos
        }
    }
}


#[derive(Debug, Clone)]
pub struct NeonTerm {
    pub buffer: Vec<(u8, u8, u8)>,
    size: (usize, usize),
    offset: (usize, usize),
    pub raw_texts: Vec<RawText>
}

impl NeonTerm {
    pub fn new(size: (usize, usize), offset: (usize, usize)) -> Self {
        let buffer = vec![(0, 0, 0); size.0 * size.1];
        // Set terminal to raw mode
        crossterm::terminal::enable_raw_mode().unwrap();
        crossterm::execute!(std::io::stdout(), crossterm::cursor::Hide).unwrap();
        NeonTerm::clear();
        return NeonTerm { buffer, size, offset, raw_texts: Vec::new() };
    }

    pub fn render(&mut self) {
        print!("\x1b[0;0H{}{}", self.to_ansi(), self.get_raw_texts_ansi());
        use std::io::{self, Write};
        io::stdout().flush().unwrap();
    }


    fn to_ansi(&self) -> String {
        // Renders the pixels to the terminal using half blocks
    
        // Create a big string with everything to print
        let mut output = String::new();
        // Y offset
        for _ in 0..(self.offset.1/2) {
            write!(output, "\n").unwrap();
        }
        let y_offset_odd = self.offset.1 % 2;
        write!(output, "{}", " ".repeat(self.offset.0)).unwrap();
        for y in 0..((self.size.1+1 + y_offset_odd)/2) {
            for x in 0..self.size.0 {
                // If the character is in a raw text, skip it
                if self.in_raw_text(x, y) {
                    write!(output, "\x1b[1C").unwrap(); // Move cursor right by 1
                    continue;
                }
                // If the y offset is odd, top half of the first row is the background color
                if y == 0 && y_offset_odd == 1 {
                    let fg = self.buffer[x];
                    write!(output, "\x1b[0m\x1b[38;2;{};{};{}m▄", fg.0, fg.1, fg.2).unwrap();
                    continue;
                }
                // If the height is odd, the last row is the background color
                if ((self.size.1 % 2 == 1) ^ (y_offset_odd == 1)) && y == (self.size.1+1 + y_offset_odd)/2 - 1 {
                    let bg = self.buffer[(y*2-y_offset_odd) * self.size.0 + x];
                    write!(output, "\x1b[0m\x1b[38;2;{};{};{}m▀", bg.0, bg.1, bg.2).unwrap();
                    continue;
                }

                // Background color : pixels[(y*2) * width + x]
                // Foreground color : pixels[(y*2+1) * width + x]
                let bg = self.buffer[(y*2-y_offset_odd) * self.size.0 + x];
                let fg = self.buffer[(y*2+1-y_offset_odd) * self.size.0 + x];
    
                write!(output, 
                       "\x1b[48;2;{};{};{}m\x1b[38;2;{};{};{}m▄", 
                       bg.0, bg.1, bg.2, fg.0, fg.1, fg.2)
                       .unwrap();
            }
            if y < (self.size.1+1 + y_offset_odd)/2 - 1 {
                write!(output, "\x1b[0m\r\n{}", " ".repeat(self.offset.0)).unwrap();
            } else {
                write!(output, "\x1b[0m").unwrap();
            }
        }
        return output;
    }

    fn get_raw_texts_ansi(&mut self) -> String {
        let mut output = String::new();
        for raw_text in &self.raw_texts {
            // Move cursor to position
            write!(output, "\x1b[{};{}H", raw_text.pos.1 + 1 + (self.offset.1+1)/2, raw_text.pos.0 + 1 + self.offset.0).unwrap();
            // Set foreground color
            write!(output, "\x1b[38;2;{};{};{}m", raw_text.fg.0, raw_text.fg.1, raw_text.fg.2).unwrap();
            // Set background color
            write!(output, "\x1b[48;2;{};{};{}m", raw_text.bg.0, raw_text.bg.1, raw_text.bg.2).unwrap();
            // Write text (filter out newlines and tabs)
            write!(output, "{}", raw_text.text).unwrap();
        }
        // Reset colors
        write!(output, "\x1b[0m").unwrap();
        return output;
    }

    fn in_raw_text(&self, x: usize, y: usize) -> bool {
        // Check if the position is in a raw text
        for raw_text in &self.raw_texts {
            if y == raw_text.pos.1 + (self.offset.1 + 1) / 2 && x >= raw_text.pos.0 && x < raw_text.pos.0 + raw_text.text.len() {
                return true;
            }
        }
        return false;
    }


    pub fn get_term_size() -> (usize, usize) {
        // Get the size of the terminal
        let size = crossterm::terminal::size().unwrap();
        return (size.0 as usize, (size.1*2) as usize);
    }

    pub fn get_size(&self) -> (usize, usize) {
        return self.size;
    }

    pub fn clear() {
        print!("\x1b[2J");
    }

    pub fn update_size(&mut self, width: usize, height: usize) {
        if self.size == (width, height) {
            return;
        }
        self.size = (width, height);
        self.buffer = vec![(0, 0, 0); width * height];
        NeonTerm::clear();
    }

    pub fn fullscreen(&mut self) {
        // Sets the NeonTerm size to the current terminal size

        // Get the terminal size
        let (width, height) = NeonTerm::get_term_size();
        self.update_size(width, height);
        self.offset = (0, 0);
    }

    pub fn update_offset(&mut self, offset: (usize, usize)) {
        if self.offset == offset {
            return;
        }
        self.offset = offset;
        NeonTerm::clear();
    }
}