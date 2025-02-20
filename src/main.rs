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

use commons::INFINITY;
use hittable::{HitRecord, Hittable};
use hittable_list::HittableList;
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
    let spheres: Vec<Box<dyn Hittable>> = vec![
        // Center sphere
        Box::new(Sphere::new(Vec3::new(0.0, 0.0, -2.5), 0.5)),
        // Left sphere
        Box::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5)),
        // Right sphere
        Box::new(Sphere::new(Vec3::new(1.0, 0.0, -3.5), 0.5)),
        // Ground sphere
        Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0))
    ];
    world.add_objects(spheres);

    let mut cam = Camera::new(
                                        16.0/9.0, 
                                        400,
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