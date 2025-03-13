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


    fn to_ansi(pixels: &mut Vec<(u8, u8, u8)>, width: usize, mut height: usize) -> String {
        // Renders the pixels to the terminal using half blocks
        // If the height is odd, add an extra row of black pixels
        if height % 2 == 1 {
            pixels.append(&mut vec![(0, 0, 0); width]);
            height += 1;
        }
    
        // Create a big string with everything to print
        let mut output = String::with_capacity(height * width * 25);
        for y in 0..height/2 {
            for x in 0..width {
                // Background color : pixels[(y*2) * width + x]
                // Foreground color : pixels[(y*2+1) * width + x]
                let bg = pixels[(y*2) * width + x];
                let fg = pixels[(y*2+1) * width + x];
    
                write!(output, 
                       "\x1b[48;2;{};{};{}m\x1b[38;2;{};{};{}m▄", 
                       bg.0, bg.1, bg.2, fg.0, fg.1, fg.2)
                       .unwrap();
            }
            write!(output, "\x1b[0m\n").unwrap();
        }
        return output;
    }

    fn overwrite(&mut self) {
        print!("\x1b[1;1H{}", NeonTerm::to_ansi(&mut self.buffer, self.size.0, self.size.1));
    }

    pub fn get_term_size() -> (usize, usize) {
        // Get the size of the terminal
        let size = crossterm::terminal::size().unwrap();
        return (size.0 as usize, size.1 as usize);
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
}