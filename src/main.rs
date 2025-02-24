use dotenv::dotenv;
use rayon::prelude::*;
use vec3::{Point3,Vec3};
use std::fs::File;
use std::io::BufWriter;
use indicatif::ProgressBar;
use std::sync::Arc;
use std::io::Write;
use crate::color::Color;
use crate::ray::Ray;



// Imports from files
mod vec3;
mod color;
mod ray;

fn main() -> std::io::Result<()> {
    dotenv().ok();

    //Camera
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let mut image_height = (image_width as f64 / aspect_ratio) as i32;
    image_height = if image_height < 1 { 1 } else { image_height };

    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * aspect_ratio;
    let camera_center = Point3::new(0.0, 0.0, 0.0);
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);
    let pixel_du = viewport_u / image_width as f64;
    let pixel_dv = viewport_v / image_height as f64;
    let viewport_upper_left = camera_center
                                    - viewport_u / 2.0
                                    - viewport_v / 2.0
                                    - Vec3::new(0.0, 0.0, focal_length);
    let pixel00_loc = viewport_upper_left
                            + 0.5 * (pixel_du + pixel_dv);



    // Output file path
    let image_output_path = std::env::var("IMAGE_OUTPUT")
            .expect("IMAGE_OUTPUT must be set");
    let file = File::create(image_output_path)?;
    let mut writer = BufWriter::new(file);

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
                let pixel_center = pixel00_loc
                                    + i as f64 * pixel_du
                                    + j as f64 * pixel_dv;
                let ray_direction = pixel_center - camera_center;
                let r = Ray::new(camera_center, ray_direction);
                let pixel_color = ray_color(&r);


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


fn ray_color(r: &Ray) -> Color {
    if hit_sphere(
        &Point3::new(0.0, 0.0, -1.0), 
        0.5, 
        r
    ) {
        return Color::new(1.0, 0.0, 0.0);
    }

    let unit_direction = r.direction().unit_vector();
    let a = 0.5 * (unit_direction.y() + 1.0);

    (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
}

fn hit_sphere(center: &Point3, radius: f64, r: &Ray) -> bool {
    let oc = r.origin() - center;
    let a = r.direction().dot(r.direction());
    let b = -2.0 * r.direction().dot(oc);
    let c = oc.dot(oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    
    discriminant >= 0.0
}

