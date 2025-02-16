use std::{error::Error, fs::File, io::{BufWriter, Write}};

mod vec3;
mod ray;
mod sphere;
mod color;
mod hittable_list;
mod hittable;
mod commons;

use commons::INFINITY;
use hittable::{HitRecord, Hittable};
use hittable_list::HittableList;
use ray::Ray;
use sphere::Sphere;
use vec3::Vec3;
use color::{Color,write_color};

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


fn main() -> Result<(), Box<dyn Error>> {

    // Image
    let aspect_ratio = 16.0/9.0;
    let image_width = 400;
    let mut image_height = (image_width as f64 / aspect_ratio) as i32;
    image_height = if image_height < 1 {1} else {image_height};

    // World
    let mut world = HittableList::new();
    let spheres: Vec<Box<dyn Hittable>> = vec![
        Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0),0.5)),
        Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.00))
    ];
    world.add_objects(spheres);

    //Camera
    let focal_lenght = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64)  ;
    let camera_center = Vec3::new(0.0,0.0,0.0);

    //Calculate the vectors accross the horizontal and down the viewport edges
    let viewport_u = Vec3::new(viewport_width,0.0,0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    //Calculate the horizontal and vertrical delta from pixel to pixel
    let pixel_du = viewport_u / (image_width as f64);
    let pixel_dv = viewport_v / (image_height as f64);

    //Calculate the location of upper left pixel
    let viewport_upper_left = camera_center 
                                    - Vec3::new(0.0, 0.0, focal_lenght)
                                    - viewport_u/2.0 
                                    - viewport_v/2.0 ;

    let pixel00_loc = viewport_upper_left 
                            + 0.5 * (pixel_du+pixel_dv);

    // Create file
    let file = File::create("./image.ppm")?;
    let mut writer = BufWriter::new(file);
    
    // Header
    writeln!(writer, "P3\n{} {}\n255", image_width, image_height)?;

    // Render
    for j in 0..image_height{
        eprint!("\rScanlines remaining: {} ", image_height - j);
        std::io::stderr().flush()?;
        for i in 0..image_width {
            let pixel_center = pixel00_loc 
                                + (i as f64 * pixel_du) 
                                + (j as f64 * pixel_dv);
            let ray_direction = pixel_center - camera_center;
            let ray = Ray::new(camera_center,ray_direction);

            let pixel_color = ray_color(&ray,&world);
            write_color(&mut writer, pixel_color)?;
        }
    }

    eprintln!("\rDone.                ");

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ray_color() {
        let origin = Vec3::new(0.0, 0.0, 0.0);
        let direction = Vec3::new(0.0, 1.0, 0.0);
        let ray = Ray::new(origin, direction);
        
        // Create a world for testing
        let world = HittableList::new();
        
        let color = ray_color(&ray, &world);
        assert!(color.y > 0.5);
    }

    #[test]
    fn test_ray_color_gradient() {
        let world = HittableList::new();
        
        let ray_up = Ray::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0)
        );
        let color_up = ray_color(&ray_up, &world);
        
        let ray_down = Ray::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, -1.0, 0.0)
        );
        let color_down = ray_color(&ray_down, &world);

        assert!(color_down.y > color_up.y);
    }
}