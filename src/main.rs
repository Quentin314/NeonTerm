mod renderer;
use std::time::Instant;
use noise::{Perlin, NoiseFn};
use renderer::NeonTerm;

fn main() {
    // Create the NeonTerm renderer
    let mut term = NeonTerm::new((5, 5), (1, 1));
    //term.raw_texts.push(renderer::RawText::new("Hello, NeonTerm!", (255, 0, 0), (0, 0, 0), (0, 0)));
    // Create a noise generator with a random seed
    let perlin = Perlin::new(42);
    
    let start_time = Instant::now();
    let mut frame_count = 0;
    loop {
        //term.fullscreen();
        // Get the terminal size
        let (mut width, mut height) = term.get_size();

        // Update time - we'll use this as our z-coordinate in the noise function
        //let time = start_time.elapsed().as_secs_f64();
        //term.buffer = noise(width, height, 0.005, 2.0, time, &perlin);

        term.buffer = checkerboard(width, height);
        
        // Render
        term.render();

        // Print the frame count
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

fn noise(width: usize, height: usize, scale: f64, speed: f64, time: f64, perlin: &Perlin) -> Vec<(u8, u8, u8)> {
    let mut pixels: Vec<(u8,u8,u8)> = Vec::with_capacity(width * height);
    // Generate new colors for the entire grid
    for y in 0..height {
        for x in 0..width {
            // Calculate noise coordinates - scaled to create nice patterns
            let nx = x as f64 * scale;
            let ny = y as f64 * scale;
            
            // Get noise values for each color component
            // Offset each channel in noise-space to create interesting color variations
            let r_value = perlin.get([nx, ny, time*speed]);
            let g_value = perlin.get([nx, ny, time*speed + 100.0]); // Offset to create different patterns
            let b_value = perlin.get([nx, ny, time*speed + 200.0]); // Offset even more
            
            // Convert noise values (-1.0 to 1.0) to RGB (0 to 255)
            let r = ((r_value * 0.5 + 0.5) * 255.0) as u8;
            let g = ((g_value * 0.5 + 0.5) * 255.0) as u8;
            let b = ((b_value * 0.5 + 0.5) * 255.0) as u8;
            
            // Store the color in the terminal buffer
            pixels.push((r, g, b));
        }
    }

    return pixels;
}

fn checkerboard(width: usize, height: usize) -> Vec<(u8, u8, u8)> {
    // Create a checkerboard pattern
    let mut pixels = Vec::with_capacity(width * height);
    for y in 0..height {
        for x in 0..width {
            let color = if (x + y) % 2 == 0 {
                (255, 255, 255)
            } else {
                (0, 0, 0)
            };
            pixels.push(color);
        }
    }
    return pixels;
}