use crate::commons::vec3::{Point3, Vec3};
use crate::commons::ray::Ray;
use crate::hittable::{Hittable, HitRecord};
use crate::interval::Interval;
use crate::material::Material;
use std::sync::Arc;

pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Option<Arc<dyn Material>>,
}

impl Default for Sphere {
    fn default() -> Self {
        Self { center: Point3::default(), radius: 1.0 ,material: None}
    }
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Option<Arc<dyn Material>>) -> Self {
        Self { 
            center, 
            radius: f64::max(0.0, radius), 
            material 
        }
    }
}

impl Hittable for Sphere {
    fn hit(
        &self,
        r: &Ray,
        ray_t: Interval,
        rec: &mut HitRecord,
    ) -> bool {
        let oc = self.center - r.origin();
        let a = r.direction().length_squared();
        let half_b = oc.dot(r.direction());
        let c = oc.length_squared() - (self.radius * self.radius);

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (half_b - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (half_b +sqrtd) /a;
            if !ray_t.surrounds(root){
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        rec.normal = (rec.p - self.center) / self.radius;
        
        // Determine which side of the sphere the ray hit
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, &outward_normal);

        return true;
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commons::INFINITY;

    fn create_sphere() -> Sphere {
        let center = Point3::new(0.0, 0.0, 0.0);
        let radius = 1.0;
        Sphere::new(center, radius,None)
    }

    #[test]
    fn test_sphere_hit() {  
        let sphere = create_sphere();
        let r = Ray::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0));
        let mut rec = HitRecord::default();
        let hit = sphere.hit(&r, Interval::new(0.0, INFINITY), &mut rec);
        assert!(hit);
    }

    #[test]
    fn test_sphere_hit_miss() {
        let sphere = create_sphere();
        let r = Ray::new(
                Point3::new(1.0, 0.0, 0.0), 
                Vec3::new(0.0, 0.0, 1.0));
        let mut rec = HitRecord::default();
        let hit = sphere.hit(&r, Interval::new(0.0, INFINITY), &mut rec);
        assert!(!hit);
    }

    #[test]
    fn test_sphere_face_normal() {
        let sphere = create_sphere();
        let r = Ray::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0));
        let mut rec = HitRecord::default();// Default is not front face
        let hit = sphere.hit(&r, Interval::new(0.0, INFINITY), &mut rec);
        assert!(hit);
        assert!(!rec.front_face); 
        assert_eq!(rec.normal, Vec3::new(0.0, 0.0, -1.0)); // Normal should be outward
    }
}
