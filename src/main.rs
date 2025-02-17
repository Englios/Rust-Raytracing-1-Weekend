use std::{error::Error, fs::File, io::{self, BufWriter, Write}};

mod vec3;
mod ray;
mod sphere;
mod color;
mod hittable_list;
mod hittable;
mod commons;
mod camera;

use commons::INFINITY;
use hittable::{HitRecord, Hittable};
use hittable_list::HittableList;
use ray::Ray;
use sphere::Sphere;
use vec3::Vec3;
use camera::Camera;
use color::{Color,write_color};




fn main() -> io::Result<()>{

    // World
    let mut world = HittableList::new();
    let spheres: Vec<Box<dyn Hittable>> = vec![
        Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0),0.5)),
        Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.00))
    ];
    world.add_objects(spheres);

    let mut cam = Camera::new(16.0/9.0, 400);

    cam.render(&world)?;
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;
}