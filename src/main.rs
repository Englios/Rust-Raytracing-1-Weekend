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

use commons::{INFINITY, PI};
use hittable::{HitRecord, Hittable};
use hittable_list::HittableList;
use material::{Dielectric, Metal,Lambertian};
use ray::Ray;
use sphere::Sphere;
use vec3::{Point3, Vec3};
use camera::Camera;
use color::{write_color,Color};
use dotenv::dotenv;
use std::sync::Arc;


fn main() -> io::Result<()>{
    //Load enviroment variables from .env
    dotenv().ok();

    // World
    let mut world = HittableList::new();

    // let R = (PI/4.0).cos();


    let material_ground = Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Arc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Arc::new(Dielectric::new(1.50));
    let matrerial_bubble = Arc::new(Dielectric::new(1.00/1.50));
    let material_right = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2),1.0));

    let spheres: Vec<Box<dyn Hittable>> = vec![
        // Center sphere
        Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.2), 0.5, material_center)),
        // Left sphere
        Box::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, material_left)),
        // Bubble Sphere
        Box::new(Sphere::new(Vec3::new(-1.0,0.0,-1.0),0.4,matrerial_bubble)),
        // Right sphere
        Box::new(Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, material_right)),
        // Ground sphere
        Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, material_ground))
    ];
    world.add_objects(spheres);

    let aspect_ratio = 16.0/9.0;
    let image_width = 1080;
    let samples_per_pixel = 200;
    let max_depth = 100;

    let vfov = 90.0;
    let lookfrom = Point3::new(-2.0,2.0,1.0);
    let lookat = Point3::new(0.0, 0.0, -1.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    
    let mut cam = Camera::new(
                                        aspect_ratio,
                                        image_width,
                                        samples_per_pixel,
                                        max_depth,
                                        vfov,
                                        lookfrom,
                                        lookat,
                                        vup
                                    );

    cam.render(&world)?;
    Ok(())
}