use crate::commons::vec3::{Point3,Vec3};
use crate::commons::ray::Ray;
use crate::commons::color::Color;
use crate::hittable::{Hittable,HitRecord};
use crate::interval::Interval;
use crate::commons::INFINITY;
use indicatif::ProgressBar;
use crate::commons::color::write_color;
use rayon::prelude::*;
use std::sync::Arc;

pub struct Camera {
    pub aspect_ratio:f64,
    pub image_width:i32,

    image_height:i32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_du : Vec3,
    pixel_dv : Vec3

}

impl Default for Camera {
    fn default() -> Self {
        Self::new()
    }
}

impl Camera {
    pub fn new() -> Self {
        Self {
            aspect_ratio: 16.0 / 9.0,
            image_width: 400,
            image_height: 0,
            center: Point3::new(0.0, 0.0, 0.0),
            pixel00_loc: Point3::new(0.0, 0.0, 0.0),
            pixel_du: Vec3::new(0.0, 0.0, 0.0),
            pixel_dv: Vec3::new(0.0, 0.0, 0.0),
        }
    }
    
    pub fn render(&mut self, writer: &mut impl std::io::Write, world: &dyn Hittable) -> std::io::Result<()> {
        self.initialize();
        // Write PPM header
        writeln!(writer, "P3\n{} {}\n255", self.image_width, self.image_height)?;

        // Progress bar
        let progress = std::sync::Arc::new(
            ProgressBar::new((self.image_height * self.image_width) as u64)
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
        let pixels:Vec<(i32,i32,i32)> = (0..self.image_height)
            .into_par_iter() // Parallel iterator
            .flat_map(move |j| {
                let progress = progress.clone(); // Clone progress bar for thread
                let pixel00_loc = self.pixel00_loc;
                let pixel_du = self.pixel_du;
                let pixel_dv = self.pixel_dv;
                let center = self.center;
                let image_width = self.image_width;

                (0..image_width).into_par_iter().map(move |i| {
                    // Calculate pixel color
                    let pixel_center = pixel00_loc
                                        + (i as f64) * pixel_du
                                        + (j as f64) * pixel_dv;
                    let ray_direction = pixel_center - center;
                    let r = Ray::new(center, ray_direction);
                    let pixel_color = Self::ray_color(&r, world);

                    // Increment progress bar
                    progress.inc(1);

                    // Write pixel color to file

                    let (r, g, b) = write_color(&pixel_color);
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

    // Initialize camera
    fn initialize(&mut self) {
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as i32;
        self.image_height = if self.image_height < 1 { 1 } else {self.image_height };

        self.center = Point3::new(0.0, 0.0, 0.0);
        
        //Viewport Dimensions
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * self.aspect_ratio;

        // Viewport Vectors
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);
        
        // Delta vectors from pixel to pixel
        self.pixel_du = viewport_u / self.image_width as f64;
        self.pixel_dv = viewport_v / self.image_height as f64;

        let viewport_upper_left = self.center
                                        - viewport_u / 2.0
                                        - viewport_v / 2.0
                                        - Vec3::new(0.0, 0.0, focal_length);
        self.pixel00_loc = viewport_upper_left
                                + 0.5 * (self.pixel_du + self.pixel_dv); 

    }

    // Ray Color
    fn ray_color(r: &Ray, world: &dyn Hittable) -> Color {
        let mut rec = HitRecord::default();
        
        if world.hit(r, Interval::new(0.0, INFINITY), &mut rec) {
            return 0.5 * (rec.normal() + Color::new(1.0, 1.0, 1.0));
        }

        let unit_direction = r.direction().unit_vector();
        let a = 0.5 * (unit_direction.y() + 1.0);

        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }   
}