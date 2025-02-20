use crate::interval::Interval;
use crate::ray::Ray;
use crate::hittable::{HitRecord,Hittable};

#[derive(Default)]
pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList { objects: Vec::new() }
    }

    pub fn add(&mut self,object: Box<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();

    }

    pub fn add_objects(&mut self,objects: Vec<Box<dyn Hittable>>){
        self.objects.extend(objects);
    }
}

impl Hittable for HittableList {

    fn hit(&self, ray: &Ray, t: &Interval, rec: &mut HitRecord) -> bool {
        
        let mut temp_rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closest = t.max();

        for object in &self.objects {
            if object.hit(ray,&Interval::new(0.0, closest),&mut temp_rec) {
                hit_anything = true;
                closest = temp_rec.t();
                *rec = temp_rec.clone();
            }
        }

        return hit_anything
    }
}

impl From<Vec<Box<dyn Hittable>>> for HittableList{
    fn from(objects: Vec<Box<dyn Hittable>>) -> Self {
        HittableList{objects}
    }
}

#[cfg(test)]
mod tests{

    use core::f64;

    use super::*;
    use crate::commons::INFINITY;
    use crate::sphere::Sphere;
    use crate::vec3::Vec3;
    use crate::material::Lambertian;
    use std::sync::Arc;

    const EPSILON:f64 = 1e-6;
    fn create_test_sphere(center:Vec3,radius:f64) -> Box<dyn Hittable> {
        let material = Arc::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5)));
        Box::new(Sphere::new(center, radius, material))
    }

    fn approx_eq(a:f64,b:f64) -> bool {
        (a-b).abs() < EPSILON
    }

    #[test]
    fn test_hittable_list_default() {
        let list = HittableList::new();
        assert!(list.objects.is_empty())
    }

    #[test]
    fn test_add_sphere(){
        let mut list = HittableList::new();
        let sphere = create_test_sphere(
            Vec3::new(0.0, 0.0, -1.0),
            0.5
        );

        list.add(sphere);

        assert_eq!(list.objects.len(),1);
    }

    #[test]
    fn test_add_multiple_spheres(){
        let mut list = HittableList::new();

        let spheres = vec![
            create_test_sphere(Vec3::default(), 0.5),
            create_test_sphere(Vec3::default(), 0.5),
            create_test_sphere(Vec3::default(), 0.5),
        ];

        list.add_objects(spheres);
        assert_eq!(list.objects.len(),3);
    }

    #[test]
    fn test_from_vec(){
        let spheres = vec![
            create_test_sphere(Vec3::default(), 0.5),
            create_test_sphere(Vec3::default(), 0.5),
            create_test_sphere(Vec3::default(), 0.5),
        ];

        let list = HittableList::from(spheres);
        assert_eq!(list.objects.len(),3);
    }
    

    #[test]
    fn test_clear() {
        let mut list = HittableList::new();
        list.add(create_test_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5));
        list.clear();
        assert!(list.objects.is_empty());
    } 

    #[test]
    fn test_hit_single_sphere() {
        let mut list = HittableList::new();
        list.add(
            create_test_sphere(
                Vec3::new(0.0, 0.0, -1.0), 0.5)
        );
        let ray = Ray::new(
            Vec3::new(0.0, 0.0, 0.0), 
            Vec3::new(0.0, 0.0, -1.0)
        );
        let mut rec = HitRecord::default();

        for object in list.objects {
            let hit = object.hit(&ray, &&Interval::new(0.0,INFINITY), &mut rec);
            assert!(hit)
        }
    }

    #[test]
    fn test_hit_closest_object(){
        let mut list =  HittableList::new();
        let mut rec = HitRecord::default();
        let ray = Ray::new(
            Vec3::new(0.0, 0.0, 0.0), 
            Vec3::new(0.0, 0.0, -1.0)
        );

        let material = Arc::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5)));
        let sphere1 = Sphere::new(
            Vec3::new(0.0, 0.0 , -1.0), 
            0.5,
            material.clone()
        );
        let sphere2 = Sphere::new(
            Vec3::new(0.0, 0.0, -1.7),
            0.5,
            material
        );

        list.add(Box::new(sphere1));
        list.add(Box::new(sphere2));

        let hit = list.hit(&ray, &Interval::new(0.0, INFINITY), &mut rec);

        assert!(hit);
        assert!(approx_eq(rec.t(), 0.5));
        assert!(approx_eq((rec.p() - Vec3::new(0.0, 0.0, -0.5)).length(), 0.0));
        assert!(approx_eq((rec.normal() - Vec3::new(0.0, 0.0, 1.0)).length(), 0.0));
    }
}