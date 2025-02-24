use dotenv::dotenv;
use rayon::prelude::*;
use std::fs::File;
use std::io::BufWriter;
use indicatif::ProgressBar;
use std::sync::Arc;
use std::io::Write;
use crate::color::Color;


// Imports from files
mod vec3;
mod color;

fn main() -> std::io::Result<()> {
    dotenv().ok();

    // Output file path
    let image_output_path = std::env::var("IMAGE_OUTPUT")
            .expect("IMAGE_OUTPUT must be set");
    let file = File::create(image_output_path)?;
    let mut writer = BufWriter::new(file);

    // Image dimensions
    let image_width  = 256;
    let image_height = 256;

    // Write PPM header
    writeln!(writer, "P3\n{} {}\n255", image_width, image_height)?;


    // Progress bar
    let progress = std::sync::Arc::new(
        ProgressBar::new((image_height * image_width) as u64)
    );
    // Set progress bar style
    progress.set_style(
        indicatif::ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} ({eta})")
            .unwrap()
    );
    

    // Clone progress bar for parallel processing
    let progress_ref = Arc::clone(&progress);

    //Parallel Processing of pixels
    let pixels:Vec<(i32,i32,i32)> = (0..image_height)
        .into_par_iter() // Parallel iterator
        .flat_map(move |j| {
            let progress = progress.clone(); // Clone progress bar for thread
            (0..image_width).into_par_iter().map(move |i| {

                // Calculate pixel color
                let pixel_color = Color::new(
                    i as f64 / (image_width - 1) as f64,
                    j as f64 / (image_height - 1) as f64,
                    0.5,
                );

                // Increment progress bar
                progress.inc(1);

                // Write pixel color to file
                let (r, g, b) = color::write_color(&pixel_color);
                (r, g, b)
            })
        })
        .collect();
    
    // Finish progress bar
    progress_ref.finish_with_message("Render complete");

    // Write pixels to file
    for (r, g, b) in pixels {
        writeln!(writer, "{} {} {}", r, g, b)?;
    }

    // Flush buffer to file
    writer.flush()?;
    Ok(())
}