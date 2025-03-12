mod renderer;
use std::time::Instant;
use crossterm;
use noise::{Perlin, NoiseFn};

fn main() {
    let mut frame_count = 0;
    // Disable the cursor
    crossterm::execute!(std::io::stdout(), crossterm::cursor::Hide).unwrap();
    // Create a noise generator with a random seed
    let perlin = Perlin::new(42);
        
    // Animation parameters
    let scale = 0.03; // Controls how zoomed in/out the noise pattern is
    let speed = 2.0; // Controls animation speed
    
    let start_time = Instant::now();
    renderer::clear();
    loop {
        // Get the terminal size
        let (width, height) = renderer::get_terminal_size();
        let size = width.min(height*2-4);
        // Update time - we'll use this as our z-coordinate in the noise function
        let time = start_time.elapsed().as_secs_f64() * speed;
        let mut grid: Vec<(u8, u8, u8)> = vec![(0, 0, 0); size * size];
        // Generate new colors for the entire grid
        for y in 0..size {
            for x in 0..size {
                // Calculate noise coordinates - scaled to create nice patterns
                let nx = x as f64 * scale;
                let ny = y as f64 * scale;
                
                // Get noise values for each color component
                // Offset each channel in noise-space to create interesting color variations
                let r_value = perlin.get([nx, ny, time]);
                let g_value = perlin.get([nx, ny, time + 100.0]); // Offset to create different patterns
                let b_value = perlin.get([nx, ny, time + 200.0]); // Offset even more
                
                // Convert noise values (-1.0 to 1.0) to RGB (0 to 255)
                let r = ((r_value * 0.5 + 0.5) * 255.0) as u8;
                let g = ((g_value * 0.5 + 0.5) * 255.0) as u8;
                let b = ((b_value * 0.5 + 0.5) * 255.0) as u8;
                
                // Store the color in our grid
                grid[y * size + x] = (r, g, b);
            }
        }
        // Convert the pixels to an ANSI string
        let ansi = renderer::to_ansi(&mut grid, size, size);

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