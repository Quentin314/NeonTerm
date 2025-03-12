mod renderer;
use crossterm;
fn main() {
    let mut frame_count = 0;
    // Disable the cursor
    crossterm::execute!(std::io::stdout(), crossterm::cursor::Hide).unwrap();
    loop {
        // Get the terminal size
        let (width, height) = renderer::get_terminal_size();
        let size = width.min(height*2-2);
        // Convert the pixels to an ANSI string
        let ansi = renderer::to_ansi(&mut rainbow_square(size, frame_count), size, size);

        // Print the ANSI string
        renderer::overwrite(ansi);

        // Print the frame count
        println!("Frame: {}", frame_count);
        frame_count += 1;

        // Sleep for a while
        std::thread::sleep(std::time::Duration::from_millis(0));
    }
}



fn rainbow_square(size: usize, offset: usize) -> Vec<(u8, u8, u8)> {
    // Create a square with rainbow colors (red increasing in the x direction, green increasing in the y direction and blue increasing in both directions)
    let mut pixels = Vec::with_capacity(size * size);
    for y in 0..size {
        for x in 0..size {
            pixels.push((((((size - x) + offset) % size) as f32 / size as f32 * 256.0) as u8, (((x + offset) % size) as f32 / size as f32 * 256.0) as u8, ((((size - y) + offset) % size) as f32 / size as f32 * 256.0) as u8));
        }
    }
    return pixels;
}