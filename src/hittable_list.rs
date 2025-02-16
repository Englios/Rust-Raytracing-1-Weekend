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

    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        
        let mut temp_rec = HitRecord::default();
        let hit_anything = false;
        let closest = t_max;

        for object in &self.objects {
            if object.hit(ray,t_min,closest,&mut temp_rec) {
                let hit_anything = true;
                let closest = temp_rec.t();
                *rec = temp_rec;
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
    use crate::sphere::{self, Sphere};
    use crate::vec3::Vec3;

    fn create_test_sphere(center:Vec3,radius:f64) -> Box<dyn Hittable> {
        Box::new(Sphere::new(center,radius))
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
            let hit = object.hit(&ray, 0.0, f64::INFINITY, &mut rec);
            assert!(hit)
        }
    }

    #[test]
    fn test_hit_closest_object(){
        let mut list =  HittableList::new();
        let sphere1 = Sphere::new(
            Vec3::new(0.0, 0.0 , -1.0), 
            0.5
        );
        let sphere2 =  Sphere::new(
            Vec3::new(0.7, 0.0, -1.0),
            0.5
        );

        list.add(Box::new(sphere1));
        list.add(Box::new(sphere2));
    }
}