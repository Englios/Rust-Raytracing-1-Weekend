use std::io;

mod vec3;
mod ray;
mod sphere;
mod color;
mod hittable_list;
mod hittable;
mod commons;
mod camera;
mod interval;
mod material;

use commons::INFINITY;
use hittable::{HitRecord, Hittable};
use hittable_list::HittableList;
use material::{Dielectric, Metal};
use ray::Ray;
use sphere::Sphere;
use vec3::Vec3;
use camera::Camera;
use color::{Color,write_color};
use dotenv::dotenv;


fn main() -> io::Result<()>{
    //Load enviroment variables from .env
    dotenv().ok();

    // World
    let mut world = HittableList::new();
    use std::sync::Arc;
    use material::Lambertian;
    use color::Color;

    let material_ground = Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Arc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Arc::new(Dielectric::new(1.5));
    let material_right = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2),1.0));

    let spheres: Vec<Box<dyn Hittable>> = vec![
        // Center sphere
        Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.2), 0.5, material_center)),
        // Left sphere
        Box::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, material_left)),
        // Right sphere
        Box::new(Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, material_right)),
        // Ground sphere
        Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, material_ground))
    ];
    world.add_objects(spheres);

    let mut cam = Camera::new(
                                        16.0/9.0, 
                                        900,
                                        100,
                                        50
                                    );

    cam.render(&world)?;
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;
}