use std::{error::Error, fs::File, io::Write};

mod vec3;
mod color;
use vec3::Vec3;
use color::write_color;

fn main() -> Result<(), Box<dyn Error>> {
    let image_width = 256;
    let image_height = 256;

    // Create file
    let mut file = File::create("./image.ppm")?;
    
    // Header
    writeln!(file, "P3\n{} {}\n255", image_width, image_height)?;

    // Render
    for j in (0..image_height) {
        eprint!("\rScanlines remaining: {} ", image_height - j);
        for i in 0..image_width {
            let r = i as f64 / (image_width - 1) as f64;
            let g = j as f64 / (image_height - 1) as f64;
            let b = 0.0;

            let pixel_color = Vec3::new(r, g, b);
            write_color(&mut file, pixel_color)?;
        }
    }

    eprintln!("\rDone.                ");

    Ok(())
}