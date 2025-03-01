// Imports from files
mod hittable;
mod sphere;
mod hittable_list;
mod commons;
mod interval;
mod camera;
mod material;

use dotenv::dotenv;
use hittable_list::HittableList;
use hittable::{HitRecord, Hittable};
use commons::vec3::{Point3,Vec3};
use sphere::Sphere;
use std::fs::File;
use std::io::BufWriter;
use camera::Camera;
use std::sync::Arc;
use commons::ray::Ray;


fn main() -> std::io::Result<()> {
    dotenv().ok();

    //World
    let mut world = HittableList::new();
    let world_list: Vec<Arc<dyn Hittable>> = vec![
        Arc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5,None)),
        Arc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.00,None)),
    ];

    world.add_multiple(world_list);

    //Camera
    let mut camera = Camera::new();
    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 400;
    camera.samples_per_pixel = 100;
    camera.max_depth = 50;
    
    // Output file path
    let image_output_path = std::env::var("IMAGE_OUTPUT")
            .expect("IMAGE_OUTPUT must be set");
    let file = File::create(image_output_path)?;
    let mut writer = BufWriter::new(file);

    camera.render(&mut writer, &world)?;

    Ok(())
}




fn hit_sphere(center: &Point3, radius: f64, r: &Ray) -> f64 {
    let oc = center - r.origin();
    
    let a = r.direction().length_squared();
    let half_b = r.direction().dot(oc);
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;

    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (half_b - discriminant.sqrt()) / a;
    }
}

