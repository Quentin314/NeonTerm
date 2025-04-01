use crossterm;

use std::fmt::Write;

#[derive(Debug, Clone)]
pub struct NeonTerm {
    pub buffer: Vec<(u8, u8, u8)>,
    size: (usize, usize),
    offset: (usize, usize)
}

impl NeonTerm {
    pub fn new(size: (usize, usize), offset: (usize, usize)) -> Self {
        let buffer = vec![(0, 0, 0); size.0 * size.1];
        crossterm::execute!(std::io::stdout(), crossterm::cursor::Hide).unwrap();
        NeonTerm::clear();
        return NeonTerm { buffer, size, offset };
    }

    pub fn render(&mut self) {
        self.overwrite();
    }


    fn to_ansi(pixels: &mut Vec<(u8, u8, u8)>, width: usize, height: usize, x_offset: usize, y_offset: usize) -> String {
        // Renders the pixels to the terminal using half blocks
    
        // Create a big string with everything to print
        let mut output = String::new();
        // Y offset
        for _ in 0..(y_offset/2) {
            write!(output, "\n").unwrap();
        }
        let y_offset_odd = y_offset % 2;
        write!(output, "{}", " ".repeat(x_offset)).unwrap();
        for y in 0..(height/2 + y_offset_odd) {
            for x in 0..width {
                // If the y offset is odd, top half of the first row is the background color
                if y == 0 && y_offset_odd == 1 {
                    let fg = pixels[x];
                    write!(output, "\x1b[0m\x1b[38;2;{};{};{}m▄", fg.0, fg.1, fg.2).unwrap();
                    continue;
                }
                // If the height is odd, the last row is the background color
                if ((height % 2 == 1) ^ (y_offset_odd == 1)) && y == height/2 - 1 + y_offset_odd {
                    let bg = pixels[(y*2-y_offset_odd) * width + x];
                    write!(output, "\x1b[0m\x1b[38;2;{};{};{}m▀", bg.0, bg.1, bg.2).unwrap();
                    continue;
                }

                // Background color : pixels[(y*2) * width + x]
                // Foreground color : pixels[(y*2+1) * width + x]
                let bg = pixels[(y*2-y_offset_odd) * width + x];
                let fg = pixels[(y*2+1-y_offset_odd) * width + x];
    
                write!(output, 
                       "\x1b[48;2;{};{};{}m\x1b[38;2;{};{};{}m▄", 
                       bg.0, bg.1, bg.2, fg.0, fg.1, fg.2)
                       .unwrap();
            }
            if y < height/2 - 1 + y_offset_odd {
                write!(output, "\x1b[0m\n{}", " ".repeat(x_offset)).unwrap();
            } else {
                write!(output, "\x1b[0m").unwrap();
            }
        }
        return output;
    }

    fn overwrite(&mut self) {
        print!("\x1b[0;0H{}", NeonTerm::to_ansi(&mut self.buffer, self.size.0, self.size.1, self.offset.0, self.offset.1));
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

    pub fn update_offset(&mut self, offset: (usize, usize)) {
        if self.offset == offset {
            return;
        }
        self.offset = offset;
        NeonTerm::clear();
    }
}