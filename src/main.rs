use dotenv::dotenv;
use rayon::prelude::*;
use std::fs::File;
use std::io::BufWriter;
use indicatif::ProgressBar;
use std::sync::Arc;
use std::io::Write;

fn main() -> std::io::Result<()> {
    dotenv().ok();


    let image_output_path = std::env::var("IMAGE_OUTPUT")
            .expect("IMAGE_OUTPUT must be set");
    let file = File::create(image_output_path)?;
    let mut writer = BufWriter::new(file);

    let image_width  = 256;
    let image_height = 256;

    // Write PPM header
 writeln!(writer, "P3\n{} {}\n255", image_width, image_height)?;

    let progress = std::sync::Arc::new(
        ProgressBar::new((image_height * image_width) as u64)
    );
    progress.set_style(
        indicatif::ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} ({eta})")
            .unwrap()
    );
    

    let progress_ref = Arc::clone(&progress);

    //Parallel Processing of pixels
    let pixels:Vec<(i32,i32,i32)> = (0..image_height)
        .into_par_iter()
        .flat_map(move |j| {
            let progress = progress.clone();
            (0..image_width).into_par_iter().map(move |i| {
                let r = i as f64 / (image_width-1) as f64;
                let g = j as f64/ (image_width-1) as f64;
                let b = 0.25;

                let ir = (255.999 * r) as i32;
                let ig = (255.999 * g) as i32;
                let ib = (255.999 * b) as i32;  

                // Increment progress bar
                progress.inc(1);

                (ir,ig,ib)
            })
        })
        .collect();
    
    progress_ref.finish_with_message("Render complete");
    // Write pixels to file
    for (r, g, b) in pixels {
        writeln!(writer, "{} {} {}", r, g, b)?;
    }

    Ok(())
}