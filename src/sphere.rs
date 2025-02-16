use std::cmp::Ordering;

use crate::vec3::Point3;
use crate::hittable::{Hittable,HitRecord};
use crate::ray::Ray;


#[derive(Debug,Copy,Clone)]
pub struct Sphere {
    center:Point3,
    radius:f64
}

impl Sphere {
    pub fn new(center:Point3,radius:f64) -> Self{
        Self {
            center,
            radius : f64::max(0.0,radius),
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self,r:&Ray,t_min:f64,t_max:f64,rec:&mut HitRecord) -> bool {
        let oc = self.center - r.origin();
        let a = r.direction().length_squared();
        let h: f64 = r.direction().dot(oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h*h - a*c;
        if discriminant < 0.0 { return false; };

        let sqrtd = discriminant.sqrt();

        //Find nearest root in acceptable range
        let mut root = (h - sqrtd) / a;
        if root <= t_min || t_max <= root{
            root = (h + sqrtd) / a;
            
            if root <= t_min || t_max <= root {
                return false;
            }
        }

        rec.set_t(root);
        rec.set_p(r.at(rec.t()));
        let outward_normal = ((rec.p() - self.center) / self.radius).unit_vector();
        rec.set_face_normal(r, outward_normal);

        true
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec3::Vec3;

    
    #[test]
    fn test_sphere_new() {
        let center = Point3::new(0.0, 0.0, 0.0);
        let radius = 1.0;
        let sphere = Sphere::new(center, radius);

        assert_eq!(sphere.center,center);
        assert_eq!(sphere.radius,radius);
    }

    #[test]
    fn test_sphere_negative_radius() {
        let center = Point3::new(0.0, 0.0, 0.0);
        let radius = -1.0;
        let sphere = Sphere::new(center, radius);

        assert_eq!(sphere.radius,0.0);
    }

    #[test]
    fn test_ray_hits_sphere(){
        let sphere = Sphere::new(Point3::new(0.0, 0.0, 0.0), 1.0);
        let ray = Ray::new(
            Point3::new(0.0, 0.0, -5.0),
            Vec3::new(0.0, 0.0, 1.0)
        );
        let mut rec = HitRecord::default();

        let hit = sphere.hit(&ray, 0.0, f64::INFINITY, &mut rec);

        assert!(hit);
        assert_eq!(rec.t(), 4.0);
        assert_eq!(rec.p(), Point3::new(0.0, 0.0, -1.0));
        assert_eq!(rec.normal(), Vec3::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn test_ray_misses_sphere(){
        let sphere = Sphere::new(Point3::new(0.0, 0.0, 0.0), 1.0);
        let ray = Ray::new(
            Point3::new(0.0, 0.0, -5.0),
            Vec3::new(0.0, 1.0, 0.0)
        );
        let mut rec = HitRecord::default();

        let hit = sphere.hit(&ray, 0.0, f64::INFINITY, &mut rec);

        assert!(!hit);
    }

    #[test]
    fn test_ray_hits_sphere_from_inside() {
        let sphere = Sphere::new(Point3::new(0.0, 0.0, 0.0), 1.0);
        let ray = Ray::new(
            Point3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, -1.0)
        );
        let mut rec = HitRecord::default();

        let hit = sphere.hit(&ray, 0.0, f64::INFINITY, &mut rec);

        assert!(hit);
        assert_eq!(rec.t(), 1.0);
        assert_eq!(rec.p(), Point3::new(0.0, 0.0, -1.0));
        assert_eq!(rec.normal(), Vec3::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn test_ray_hits_sphere_at_t_min() {
        let sphere = Sphere::new(Point3::new(0.0, 0.0, 0.0), 1.0);
        let ray = Ray::new(
            Point3::new(0.0, 0.0, -5.0),
            Vec3::new(0.0, 0.0, 1.0)
        );
        let mut rec = HitRecord::default();

        let hit = sphere.hit(&ray, 2.0, f64::INFINITY, &mut rec);

        assert!(hit);
        assert_eq!(rec.t(), 4.0);
        assert_eq!(rec.p(), Point3::new(0.0, 0.0, -1.0));
        assert_eq!(rec.normal(), Vec3::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn test_ray_hits_sphere_at_t_max() {
        let sphere = Sphere::new(Point3::new(0.0, 0.0, -3.0), 1.0);
        let ray = Ray::new(
            Point3::new(0.0, 0.0, -5.0),
            Vec3::new(0.0, 0.0, 1.0)
        );
        let mut rec = HitRecord::default();

        let hit = sphere.hit(&ray, 0.0, 3.0, &mut rec);

        assert!(hit);
        assert_eq!(rec.t(), 1.0);
        assert_eq!(rec.p(), Point3::new(0.0, 0.0, -4.0));
        assert_eq!(rec.normal(), Vec3::new(0.0, 0.0, -1.0));
    }
}