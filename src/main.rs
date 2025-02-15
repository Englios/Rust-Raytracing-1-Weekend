use std::{error::Error, fs::File, io::Write};

fn main() -> Result<(), Box<dyn Error>> {
    let image_width = 256;
    let image_height = 256;

    // Create file
    let mut file = File::create("./image.ppm")?;
    
    // Header
    writeln!(file, "P3\n{} {}\n255", image_width, image_height)?;

    // Render
    for j in 0..image_height {
        eprint!("\rScanlines remaining: {} ", image_height - j);  // Using eprint! for stderr
        for i in 0..image_width {
            let r = i as f64 / (image_width - 1) as f64;
            let g = j as f64 / (image_height - 1) as f64;
            let b = 0.0;

            let ir = (255.999 * r) as i32; 
            let ig = (255.999 * g) as i32;
            let ib = (255.999 * b) as i32;

            writeln!(file, "{} {} {}", ir, ig, ib)?;
        }
    }

    eprintln!("\rDone.                ");  // Using eprintln! for stderr

    Ok(())
}