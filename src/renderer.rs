use crossterm;

use std::fmt::Write;

pub fn to_ansi(pixels: &mut Vec<(u8, u8, u8)>, width: usize, mut height: usize) -> String {
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

pub fn get_terminal_size() -> (usize, usize) {
    // Get the size of the terminal
    let size = crossterm::terminal::size().unwrap();
    return (size.0 as usize, size.1 as usize);
}


pub fn overwrite(ansi:String) {
    print!("\x1b[1;1H{}", ansi);
}


pub fn clear() {
    print!("\x1b[2J");
}