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
use material::{Dielectric, Metal,Lambertian};
use ray::Ray;
use sphere::Sphere;
use vec3::{Point3, Vec3};
use camera::Camera;
use color::{write_color,Color};
use dotenv::dotenv;
use std::sync::Arc;

fn toy_env() -> io::Result<()>{
    let mut world = HittableList::new();

    // let R = (PI/4.0).cos();


    let material_ground = Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Arc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Arc::new(Dielectric::new(1.50));
    let matrerial_bubble = Arc::new(Dielectric::new(1.0/1.50));
    let material_right = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2),0.7));

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
    let image_width = 400;
    let samples_per_pixel = 200;
    let max_depth = 100;

    let vfov = 45.0;
    let lookfrom = Point3::new(0.0,0.0,1.0);
    let lookat = Point3::new(0.0, 0.0, -1.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);

    let defocus_angle = 0.0;
    let focus_dist = 1.0;
    
    let mut cam = Camera::new(
                                        aspect_ratio,
                                        image_width,
                                        samples_per_pixel,
                                        max_depth,
                                        vfov,
                                        lookfrom,
                                        lookat,
                                        vup,
                                        defocus_angle,
                                        focus_dist
                                    );

    cam.render(&world)?;

    Ok(())
}

fn book_env() -> io::Result<()> {
    let mut world = HittableList::new();

    // Ground material and sphere
    let ground_material = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    // Random small spheres
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = commons::random_double();
            let center = Point3::new(
                a as f64 + 0.9 * commons::random_double(),
                0.2,
                b as f64 + 0.9 * commons::random_double(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // Diffuse
                    let albedo = Color::random() * Color::random();
                    let sphere_material = Arc::new(Lambertian::new(albedo));
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                } else if choose_mat < 0.95 {
                    // Metal
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = commons::random_double() * 0.5;
                    let sphere_material = Arc::new(Metal::new(albedo, fuzz));
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    // Glass
                    let sphere_material = Arc::new(Dielectric::new(1.5));
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    // Three large spheres
    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Box::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Box::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    // Camera settings
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 1200;
    let samples_per_pixel = 500;
    let max_depth = 50;

    let vfov = 20.0;
    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);

    let defocus_angle = 0.6;
    let focus_dist = 10.0;

    let mut cam = Camera::new(
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
    );

    cam.render(&world)?;

    Ok(())
}



fn main() -> io::Result<()>{
    //Load enviroment variables from .env
    dotenv().ok();

    // book_env()
    toy_env()
}