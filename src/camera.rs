use super::*;
use crate::vec3::Point3;
use std::io::{self,BufWriter,Write};
use std::fs::File;

pub struct Camera{
    pub aspect_ratio : f64,
    pub image_width : i32,
    image_height :i32,
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
            image_height: 100, // Calculated based on aspect_ratio
            center: Point3::new(0.0, 0.0, 0.0),
            pixel00_loc: Point3::new(0.0, 0.0, 0.0),
            pixel_du: Vec3::new(0.0, 0.0, 0.0),
            pixel_dv: Vec3::new(0.0, 0.0, 0.0)
        }
    }
}

impl Camera {
    pub fn new(aspect_ratio:f64,image_width:i32) -> Self {
        Self {
            aspect_ratio,
            image_width,
            image_height: 0, // Calculated based on aspect_ratio
            center: Point3::new(0.0, 0.0, 0.0),
            pixel00_loc: Point3::new(0.0, 0.0, 0.0),
            pixel_du: Vec3::new(0.0, 0.0, 0.0),
            pixel_dv: Vec3::new(0.0, 0.0, 0.0)
        }
        
    }

    fn initialize(&mut self) {
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as i32;
        self.image_height = if self.image_height < 1 {1} else {self.image_height};
    
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

    fn ray_color(r: &Ray,world: &dyn Hittable) -> Color{
        let mut rec = HitRecord::default();
    
        if world.hit(r, 0.0, INFINITY, &mut rec) {
            return 0.5 * (rec.normal() + Color::new(1.0,1.0, 1.0));
        }
    
        let unit_direction = r.direction().unit_vector();
        let a = 0.5 * (unit_direction.y + 1.0);
    
        let white =  Color::new(1.0, 1.0, 1.0);
        let blue =  Color::new(0.5, 0.7, 1.0);
    
        (1.0 - a) * white //White
        + a * blue // Blue
    }

    pub fn render(&mut self, world: &dyn Hittable) -> io::Result<()> {
        self.initialize();
    
        let file = File::create("./image.ppm")?;
        let mut writer = BufWriter::new(file);
        
        // Header
        writeln!(writer, "P3\n{} {}\n255", self.image_width, self.image_height)?;
    
        // Render
        for j in 0..self.image_height {
            eprint!("\rScanlines remaining: {} ", self.image_height - j);
            io::stderr().flush()?;
            for i in 0..self.image_width {
                let pixel_center = self.pixel00_loc 
                    + (i as f64 * self.pixel_du) 
                    + (j as f64 * self.pixel_dv);
                let ray_direction = (pixel_center - self.center).unit_vector();
                let ray = Ray::new(self.center, ray_direction);
    
                let pixel_color = Camera::ray_color(&ray, world);
                write_color(&mut writer, pixel_color)?;
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
        let camera = Camera::new(2.0, 200);
        assert_eq!(camera.aspect_ratio, 2.0);
        assert_eq!(camera.image_width, 200);
    }

    #[test]
    fn test_camera_initialize() {
        let mut camera = Camera::new(2.0, 200);
        camera.initialize();
        assert_eq!(camera.image_height, 100); // 200/2.0 = 100
        assert!(camera.pixel_du.length() > 0.0);
        assert!(camera.pixel_dv.length() > 0.0);
    }
}