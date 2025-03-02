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
use hittable::Hittable;
use commons::vec3::{Point3,Vec3};
use commons::{PI,random_double};
use sphere::Sphere;
use std::fs::File;
use std::io::BufWriter;
use camera::Camera;
use std::sync::Arc;
use material::{Lambertian, Metal, Dielectric};
use commons::color::Color;


//Scenes
fn main_scene() -> Vec<Arc<dyn Hittable>> {

    let material_ground = Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Arc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Arc::new(Dielectric::new(1.50));
    let material_bubble = Arc::new(Dielectric::new(1.00/1.50));
    let material_right = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2),1.0));

    let world_list: Vec<Arc<dyn Hittable>> = vec![
        Arc::new(Sphere::new(Point3::new(0.0, 0.0, -1.2), 0.5, Some(material_center))),
        Arc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.00, Some(material_ground))),
        Arc::new(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, Some(material_left))),
        Arc::new(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.4, Some(material_bubble))),
        Arc::new(Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, Some(material_right))),
    ];
    
    world_list
}

fn fov_scene() -> Vec<Arc<dyn Hittable>> {

    let r = (PI / 4.0).cos();

    let material_left = Arc::new(Lambertian::new(Color::new(0.0, 0.0, 1.0)));
    let material_right = Arc::new(Lambertian::new(Color::new(1.0, 0.0, 0.0)));

    let world_list: Vec<Arc<dyn Hittable>> = vec![
        Arc::new(Sphere::new(Point3::new(-1.0, 0.0, -1.0), r, Some(material_left))),
        Arc::new(Sphere::new(Point3::new(1.0, 0.0, -1.0), r, Some(material_right))),
    ];

    world_list
}


fn book_scene() -> Vec<Arc<dyn Hittable>> {

    let mut world_list: Vec<Arc<dyn Hittable>> = vec![];

    let ground_material = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    let ground = Arc::new(Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, Some(ground_material)));

    world_list.push(ground);
    
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double();
            let center = Point3::new(a as f64 + 0.9 * random_double(), 0.2, b as f64 + 0.9 * random_double());

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {

                if choose_mat < 0.8 {
                    //Lambertian
                    let albedo = Color::random() * Color::random();
                    let sphere_material = Arc::new(Lambertian::new(albedo));
                    let sphere = Arc::new(Sphere::new(center, 0.2, Some(sphere_material)));
                    world_list.push(sphere);
                } else if choose_mat < 0.95 {
                    //Metal
                    let sphere_material = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
                    let sphere = Arc::new(Sphere::new(center, 0.2, Some(sphere_material)));
                    world_list.push(sphere);
                } else {
                    //Dielectric
                    let sphere_material = Arc::new(Dielectric::new(1.5));
                    let sphere = Arc::new(Sphere::new(center, 0.2, Some(sphere_material)));
                    world_list.push(sphere);
                }   
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    world_list.push(Arc::new(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, Some(material1))));

    let material2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world_list.push(Arc::new(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, Some(material2))));

    let material3 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    let sphere = Arc::new(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, Some(material3)));
    world_list.push(sphere);

    world_list
}
    

//Camera
fn main_camera() -> Camera {
    let mut camera = Camera::new();
    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 400;
    camera.samples_per_pixel = 100;
    camera.max_depth = 50;
    camera.vfov = 20.0;

    camera.lookfrom = Point3::new(-2.0, 2.0, 1.0);
    camera.lookat = Point3::new(0.0, 0.0, -1.0);
    camera.vup = Vec3::new(0.0, 1.0, 0.0);

    camera.defocus_angle = 10.0;
    camera.focus_dist = 3.4;
    
    camera
}


fn book_camera() -> Camera {
    let mut camera = Camera::new();
    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 1200;
    camera.samples_per_pixel = 500;
    camera.max_depth = 50;

    camera.vfov = 20.0;
    camera.lookfrom = Point3::new(13.0, 2.0, 3.0);
    camera.lookat = Point3::new(0.0, 0.0, 0.0);
    camera.vup = Vec3::new(0.0, 1.0, 0.0);

    camera.defocus_angle = 0.6;
    camera.focus_dist = 10.0;
    
    camera
}
    
    
fn main() -> std::io::Result<()> {
    dotenv().ok();

    let mut world = HittableList::new();
    let world_list = book_scene();

    world.add_multiple(world_list);

    //Camera
    let mut camera = book_camera();

    let image_output_path = std::env::var("IMAGE_OUTPUT")
            .expect("IMAGE_OUTPUT must be set");
    let file = File::create(image_output_path)?;
    let mut writer = BufWriter::new(file);

    camera.render(&mut writer, &world)?;

    Ok(())
}

