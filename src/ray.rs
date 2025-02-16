use crate::vec3::Vec3;

#[derive(Debug,Copy,Clone)]
pub struct Ray{
    origin:Vec3,
    direction:Vec3
}

impl Ray {
    pub fn new(origin:Vec3,direction:Vec3) -> Self{
        Self{origin,direction}
    }

    pub fn origin(&self) -> Vec3{
        self.origin
    }

    pub fn direction(&self) -> Vec3{
        self.direction
    }

    pub fn at(self,t:f64) -> Vec3{
        self.origin + t*self.direction
    }
}

impl Default for Ray{
    fn default() -> Self {
        Self {
            origin: Vec3::default(),
            direction: Vec3::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ray_creation(){
        let origin = Vec3::new(1.0,2.0,3.0);
        let direction = Vec3::new(2.0,0.0,0.0);

        let ray = Ray::new(origin,direction);

        assert_eq!(ray.origin(),origin);
        assert_eq!(ray.direction(),direction)
    }

    #[test]
    fn test_ray_at(){
        let origin = Vec3::new(1.0,2.0,3.0);
        let direction = Vec3::new(2.0,0.0,0.0);

        let ray = Ray::new(origin,direction);

        let ray_at = ray.at(2.0);

        assert_eq!(ray_at.x,5.0);
        assert_eq!(ray_at.y,2.0);
        assert_eq!(ray_at.z,3.0);
    }

    #[test]
    fn test_ray_at_neg(){
        let origin = Vec3::new(1.0,2.0,3.0);
        let direction = Vec3::new(2.0,-3.0,-1.0);

        let ray = Ray::new(origin,direction);

        let ray_at = ray.at(2.0);

        assert_eq!(ray_at.x,5.0);
        assert_eq!(ray_at.y,-4.0);
        assert_eq!(ray_at.z,1.0);
    }
}