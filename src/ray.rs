use crate::vec3::{Point3, Vec3};


#[derive(Debug, Clone, Copy)]
pub struct Ray {
    origin : Point3,
    direction : Vec3
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }

    pub fn origin(&self) -> Point3 {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + self.direction * t
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ray_at() {
        let r = Ray::new(Point3::new(1.0, 2.0, 3.0), Vec3::new(4.0, 5.0, 6.0));
        let p = r.at(2.0);
        assert_eq!(p, Point3::new(9.0, 12.0, 15.0));
    }

    #[test]
    fn test_ray_direction() {
        let r = Ray::new(Point3::new(1.0, 2.0, 3.0), Vec3::new(4.0, 5.0, 6.0));
        assert_eq!(r.direction(), Vec3::new(4.0, 5.0, 6.0));
    }

    #[test]
    fn test_ray_origin() {
        let r = Ray::new(Point3::new(1.0, 2.0, 3.0), Vec3::new(4.0, 5.0, 6.0));
        assert_eq!(r.origin(), Point3::new(1.0, 2.0, 3.0));
    }
}