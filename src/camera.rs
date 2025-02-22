use super::*;
use rayon::prelude::*;
use crate::color::Color;
use crate::commons::{degrees_to_radians, random_double};
use crate::interval::Interval;
use crate::vec3::Point3;
use std::io::{self,BufWriter,Write};
use std::fs::File;
use indicatif::ProgressBar;
use std::sync::Arc;

#[derive(Clone, Copy)]
pub struct Camera{
    pub aspect_ratio : f64,
    pub image_width : i32,
    pub samples_per_pixel : i32,
    pub max_depth: i32,

    pub vfov : f64,
    pub lookfrom : Point3,
    pub lookat: Point3,
    pub vup: Vec3,

    pub defocus_angle:f64,
    pub focus_dist:f64,

    image_height :i32,
    pixel_samples_scale: f64,
    center : Point3,
    pixel00_loc :Point3,
    pixel_du : Vec3,
    pixel_dv : Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3 
}


impl Default for Camera {
    fn default() -> Self {
        Self {
            aspect_ratio: 1.0,
            image_width: 100,
            samples_per_pixel: 100,
            max_depth:50,
            vfov:90.0,
            lookfrom: Point3::new(0.0, 0.0, 0.0),
            lookat: Point3::new(0.0, 0.0, 0.0),
            vup: Vec3::new(0.0, 1.0, 0.0),
            defocus_angle:0.0,
            focus_dist:10.0,
            image_height: 100,
            pixel_samples_scale: 1.0,
            center: Point3::new(0.0, 0.0, 0.0),
            pixel00_loc: Point3::new(0.0, 0.0, 0.0),
            pixel_du: Vec3::new(0.0, 0.0, 0.0),
            pixel_dv: Vec3::new(0.0, 0.0, 0.0),
            u: Vec3::default(),
            v: Vec3::default(),
            w: Vec3::default(),
            defocus_disk_u: Vec3::default(),
            defocus_disk_v: Vec3::default()
        }
    }
}

impl Camera {
    pub fn new(
        aspect_ratio:f64,
        image_width:i32,
        samples_per_pixel:i32,
        max_depth:i32,vfov:f64,
        lookfrom:Point3,
        lookat:Point3,
        vup:Vec3,
        defocus_angle:f64,
        focus_dist:f64
    ) -> Self {
        Self {
            aspect_ratio,
            image_width,
            samples_per_pixel,
            max_depth,
            vfov,
            lookfrom,
            lookat,
            vup,
            defocus_angle,
            focus_dist,
            image_height: 0, // Calculated based on aspect_ratio
            pixel_samples_scale:0.0,
            center: Point3::new(0.0, 0.0, 0.0),
            pixel00_loc: Point3::new(0.0, 0.0, 0.0),
            pixel_du: Vec3::new(0.0, 0.0, 0.0),
            pixel_dv: Vec3::new(0.0, 0.0, 0.0),
            u: Vec3::new(0.0, 0.0, 0.0),
            v: Vec3::new(0.0, 0.0, 0.0),
            w: Vec3::new(0.0, 0.0, 0.0),
            defocus_disk_u: Vec3::new(0.0, 0.0, 0.0),
            defocus_disk_v: Vec3::new(0.0, 0.0, 0.0),
        }
        
    }

    fn initialize(&mut self) {
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as i32;
        self.image_height = if self.image_height < 1 {1} else {self.image_height};
        self.pixel_samples_scale = 1.0 / self.samples_per_pixel as f64;
    
        self.center = self.lookfrom;
    
        // View Port Dimensions
        let theta = degrees_to_radians(self.vfov);
        let h = (theta/2.0).tan();
        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width = viewport_height * ((self.image_width as f64) / (self.image_height as f64));

        // Basis Vectors for Camera
        let w = (self.lookfrom - self.lookat).unit_vector();
        let u = self.vup.cross(w).unit_vector();
        let v = w.cross(u);
        // Vectors for Viewport vertical and horizontal directions
        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;
    
        self.pixel_du = viewport_u / self.image_width as f64;
        self.pixel_dv = viewport_v / self.image_height as f64;
    
        let viewport_upper_left = self.center - (self.focus_dist * w) - viewport_u/2.0 - viewport_v/2.0;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_du + self.pixel_dv);

        let defocus_radius = self.focus_dist * degrees_to_radians(self.defocus_angle / 2.0).tan();
        self.defocus_disk_u = u * defocus_radius;
        self.defocus_disk_v = v * defocus_radius;
    }

    fn ray_color(r: &Ray,depth:i32 ,world: &dyn Hittable) -> Color{

        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        let mut rec = HitRecord::default();
    
        if world.hit(r, &Interval::new(0.001, INFINITY), &mut rec) {
            let mut scattered = Ray::default();
            let mut attenuation = Color::default();

            if rec.mat.scatter(&r, &rec, &mut attenuation, &mut scattered) {
                let recursive_color = Camera::ray_color(&scattered, depth - 1, world);
                return attenuation * recursive_color;
            }
            return Color::new(0.0, 0.0, 0.0);
        }
    
        let unit_direction = r.direction().unit_vector();
        let a = 0.5 * (unit_direction.y + 1.0);
    
        let white =  Color::new(1.0, 1.0, 1.0);
        let blue =  Color::new(0.5, 0.7, 1.0);
    
        (1.0 - a) * white + a * blue
    }

    fn sample_square() -> Vec3 {
        Vec3{
            x: random_double() - 0.5, 
            y: random_double() - 0.5, 
            z: 0.0,
        }
    }

    fn defocus_disk_sample(&self) -> Vec3 {
        let p = Vec3::random_in_unit_disk();

        self.center + (p.x() * self.defocus_disk_u) + (p.y() * self.defocus_disk_v)
    }


    fn get_ray(&self, i: i32, j: i32) -> Ray {
        let offset = Camera::sample_square();
        let pixel_sample = self.pixel00_loc 
                                + ((i as f64 + offset.x()) * self.pixel_du)
                                + ((j as f64 + offset.y()) * self.pixel_dv);

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };

        let ray_direction = pixel_sample - ray_origin;
        
        Ray::new(ray_origin, ray_direction)
    }

    fn pixel_sample_square(&self) -> Vec3 {
        let px = -0.5 + random_double();
        let py = -0.5 + random_double();
        (px * self.pixel_du) + (py * self.pixel_dv)
    }

    pub fn render(&mut self, world: &dyn Hittable) -> io::Result<()> {
        self.initialize();

        let image_output_path = std::env::var("IMAGE_OUTPUT")
            .expect("IMAGE_OUTPUT must be set");
        let file = File::create(image_output_path)?;
        let mut writer = BufWriter::new(file);
        
        writeln!(writer, "P3\n{} {}\n255", self.image_width, self.image_height)?;

        let samples = self.samples_per_pixel;
        let max_depth = self.max_depth;
        let scale = self.pixel_samples_scale;
        let image_width = self.image_width;

        // Add progress bar
        let progress = std::sync::Arc::new(ProgressBar::new((self.image_height * self.image_width) as u64));
        progress.set_style(
            indicatif::ProgressStyle::default_bar()
                .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} ({eta})")
                .unwrap()
        );

        //Proggrs bar must be clone first to reference within parallel iterations
        let progress_ref = Arc::clone(&progress);
        let camera = &self;

        //Store all pixels data in a List
        let pixels: Vec<Color> = (0..self.image_height)
            .into_par_iter()
            .flat_map(move |j| {
                let progress = progress.clone();
                (0..image_width).into_par_iter().map(move |i| {
                    let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                    for _ in 0..samples {
                        let r = camera.get_ray(i, j);
                        pixel_color += Camera::ray_color(&r, max_depth, world);
                    }
                    progress.inc(1);
                    pixel_color * scale
                })
            })
            .collect();

        progress_ref.finish_with_message("Render complete");
        // Write pixels to file
        for pixel in pixels {
            write_color(&mut writer, pixel)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_camera_default() {
        let camera = Camera::default();
        assert_eq!(camera.aspect_ratio, 1.0);
        assert_eq!(camera.image_width, 100);
        assert_eq!(camera.image_height, 100);
    }

    #[test]
    fn test_camera_new() {
        let camera = Camera::new(
            2.0, 
            200,
            100,
            10,
            90.0,
            Point3::new(0.0, 0.0, -1.0),
            Point3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            0.0,  // defocus_angle
            10.0  // focus_dist
        );
        assert_eq!(camera.aspect_ratio, 2.0);
        assert_eq!(camera.image_width, 200);
        assert_eq!(camera.lookfrom, Point3::new(0.0, 0.0, -1.0));
        assert_eq!(camera.lookat, Point3::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn test_camera_initialize() {
        let mut camera = Camera::new(
            2.0, 
            200,
            100,
            10,
            90.0,
            Point3::new(0.0, 0.0, -1.0),
            Point3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            0.0,  // defocus_angle
            10.0  // focus_dist
        );
        camera.initialize();
        assert_eq!(camera.image_height, 100); // 200/2.0 = 100
        assert!(camera.pixel_du.length() > 0.0);
        assert!(camera.pixel_dv.length() > 0.0);
        assert_eq!(camera.center, Point3::new(0.0, 0.0, -1.0));
    }
}