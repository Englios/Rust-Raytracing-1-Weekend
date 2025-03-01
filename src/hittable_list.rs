use crate::hittable::{Hittable,HitRecord};
use crate::commons::ray::Ray;
use crate::interval::Interval;
use std::sync::Arc;


#[derive(Clone)]
pub struct HittableList {
    objects: Vec<Arc<dyn Hittable>>,
}

impl Default for HittableList {
    fn default() -> Self {
        Self::new()
    }
}

impl HittableList {
    pub fn new() -> Self {
        Self { objects: Vec::new() }
    }

    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add_multiple(&mut self, objects: Vec<Arc<dyn Hittable>>) {
        self.objects.extend(objects);
    }
    
}


impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t:Interval, rec: &mut HitRecord) -> bool {
        let mut temp_rec = *rec;
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max();

        for object in &self.objects {
            if object.hit(r, Interval::new(ray_t.min(), closest_so_far),&mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t();
                *rec = temp_rec;
            }
        }

        hit_anything
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sphere::Sphere;
    use crate::commons::vec3::{Point3, Vec3};
    use crate::commons::ray::Ray;
    use crate::commons::INFINITY;
    use std::sync::Arc;

    fn create_test_list() -> HittableList {
        let mut list = HittableList::new();
        list.add(Arc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
        list.add(Arc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));
        list
    }

    #[test]
    fn test_new_list_is_empty() {
        let list = HittableList::new();
        assert_eq!(list.objects.len(), 0);
    }

    #[test]
    fn test_add_object() {
        let mut list = HittableList::new();
        list.add(Arc::new(Sphere::default()));
        assert_eq!(list.objects.len(), 1);
    }

    #[test]
    fn test_clear_list() {
        let mut list = create_test_list();
        assert_eq!(list.objects.len(), 2);
        list.clear();
        assert_eq!(list.objects.len(), 0);
    }

    #[test]
    fn test_add_multiple() {
        let mut list = HittableList::new();
        let objects: Vec<Arc<dyn Hittable>> = vec![
            Arc::new(Sphere::default()),
            Arc::new(Sphere::new(Point3::new(1.0, 1.0, 1.0), 0.5))
        ];
        list.add_multiple(objects);
        assert_eq!(list.objects.len(), 2);
    }

    #[test]
    fn test_hit_something() {
        let list = create_test_list();
        let r = Ray::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -1.0));
        let mut rec = HitRecord::default();
        assert!(list.hit(&r, Interval::new(0.0, INFINITY), &mut rec));
        assert!(rec.t() > 0.0);
    }

    #[test]
    fn test_hit_nothing() {
        let list = create_test_list();
        let r = Ray::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 0.0));
        let mut rec = HitRecord::default();
        assert!(!list.hit(&r, Interval::new(0.0, INFINITY), &mut rec));
    }

    #[test]
    fn test_closest_hit() {
        let list = create_test_list();
        let r = Ray::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -1.0));
        let mut rec = HitRecord::default();
        assert!(list.hit(&r, Interval::new(0.0, INFINITY), &mut rec));
        // Should hit the closer sphere at z = -1.0
        assert!((rec.t() - 0.5).abs() < 1e-6);
    }
}
