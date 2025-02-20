use super::*;
use crate::color::Color;
use crate::commons::random_double;
use crate::interval::Interval;
use crate::vec3::Point3;
use std::io::{self,BufWriter,Write};
use std::fs::File;

pub struct Camera{
    pub aspect_ratio : f64,
    pub image_width : i32,
    pub samples_per_pixel : i32,
    pub max_depth: i32,

    image_height :i32,
    pixel_samples_scale: f64,
    center : Point3,
    pixel00_loc :Point3,
    pixel_du : Vec3,
    pixel_dv : Vec3
}


impl Default for Camera {
    fn default() -> Self {
        Self {
            aspect_ratio: 1.0,
            image_width: 100,
            samples_per_pixel: 100,
            max_depth:50,
            image_height: 100,
            pixel_samples_scale: 1.0,
            center: Point3::new(0.0, 0.0, 0.0),
            pixel00_loc: Point3::new(0.0, 0.0, 0.0),
            pixel_du: Vec3::new(0.0, 0.0, 0.0),
            pixel_dv: Vec3::new(0.0, 0.0, 0.0)
        }
    }
}

impl Camera {
    pub fn new(aspect_ratio:f64,image_width:i32,samples_per_pixel:i32,max_depth:i32) -> Self {
        Self {
            aspect_ratio,
            image_width,
            samples_per_pixel,
            max_depth,
            image_height: 0, // Calculated based on aspect_ratio
            pixel_samples_scale:0.0,
            center: Point3::new(0.0, 0.0, 0.0),
            pixel00_loc: Point3::new(0.0, 0.0, 0.0),
            pixel_du: Vec3::new(0.0, 0.0, 0.0),
            pixel_dv: Vec3::new(0.0, 0.0, 0.0)
        }
        
    }

    fn initialize(&mut self) {
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as i32;
        self.image_height = if self.image_height < 1 {1} else {self.image_height};
        self.pixel_samples_scale = 1.0 / self.samples_per_pixel as f64;
    
        self.center = Point3::new(0.0, 0.0, 0.0);
    
        // View Port Dimensions
        let focal_length = 0.5;  
        let viewport_height = 2.0;
        let viewport_width = viewport_height * ((self.image_width as f64) / (self.image_height as f64));
    
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);
    
        self.pixel_du = viewport_u / self.image_width as f64;
        self.pixel_dv = viewport_v / self.image_height as f64;
    
        let viewport_upper_left = self.center - Vec3::new(0.0, 0.0, focal_length) - viewport_u/2.0 - viewport_v/2.0;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_du + self.pixel_dv);
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

    fn get_ray(&self,i:i32,j:i32) -> Ray {
        let offset = Camera::sample_square();

        let pixel_center = self.pixel00_loc 
                    + ((i as f64 + offset.x) * self.pixel_du) 
                    + ((j as f64 + offset.y) * self.pixel_dv);

        let ray_direction = (pixel_center - self.center).unit_vector();

        Ray::new(self.center, ray_direction)
    }


    pub fn render(&mut self, world: &dyn Hittable) -> io::Result<()> {
        self.initialize();

        // Path to image output
        let image_output_path = std::env::var("IMAGE_OUTPUT")
                                                        .expect("IMAGE_OUTPUT must be set");
    
        let file = File::create(image_output_path)?;
        let mut writer = BufWriter::new(file);
        
        // Header
        writeln!(writer, "P3\n{} {}\n255", self.image_width, self.image_height)?;
    
        // Render
        for j in 0..self.image_height {
            eprint!("\rScanlines remaining: {} ", self.image_height - j);
            io::stderr().flush()?;
            for i in 0..self.image_width {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);

                for sample in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color += Camera::ray_color(
                                                    &r,
                                                    self.max_depth,
                                                    world
                                                );
                }

                write_color(
                    &mut writer, 
                    pixel_color * self.pixel_samples_scale
                )?;
            }
        }
    
        eprintln!("\rDone.                ");
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
        let camera = Camera::new(2.0, 200,100,10);
        assert_eq!(camera.aspect_ratio, 2.0);
        assert_eq!(camera.image_width, 200);
    }

    #[test]
    fn test_camera_initialize() {
        let mut camera = Camera::new(2.0, 200,100,10);
        camera.initialize();
        assert_eq!(camera.image_height, 100); // 200/2.0 = 100
        assert!(camera.pixel_du.length() > 0.0);
        assert!(camera.pixel_dv.length() > 0.0);
    }
}