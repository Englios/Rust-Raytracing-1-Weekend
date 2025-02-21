use std::ops::{
            Add,
            AddAssign, 
            Div, 
            DivAssign, 
            Mul, 
            MulAssign, 
            Neg, 
            Sub, 
            SubAssign
            };

use crate::commons::{random_double, random_double_range};

            

#[derive(Debug, Clone, Copy,PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub type Point3 = Vec3;

impl Default for Vec3 {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0
        }
    }
}

impl Vec3 {
    //Construct a new Vec3 instance
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn from_scalar(scalar: f64) -> Self {
        Self { x: scalar, y: scalar, z: scalar }
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn dot(&self,other:Vec3) -> f64{
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self,other:Vec3) ->  Self{
        Self{
            x: self.y*other.z - self.z*other.y,
            y: self.z*other.x - self.x * other.z,
            z: self.x*other.y - self.y * other.x
        }
    }

    pub fn unit_vector(self) -> Self {
        let len = self.length();
        if len == 0.0 {
            self
        } else {
            // Handle negative zero by using abs() for very small values
            Self {
                x: if self.x.abs() < 1e-8 { 0.0 } else { self.x / len },
                y: if self.y.abs() < 1e-8 { 0.0 } else { self.y / len },
                z: if self.z.abs() < 1e-8 { 0.0 } else { self.z / len },
            }
        }
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;

        self.x.abs() < s && self.y.abs() < s && self.z.abs() < s
    }

    pub fn random() -> Vec3 {
        Vec3 {
            x: random_double(),
            y: random_double(),
            z: random_double()
        }
    }

    pub fn random_range(min:f64,max:f64) -> Vec3 {
        Vec3 {
            x: random_double_range(min, max),
            y: random_double_range(min, max),
            z: random_double_range(min, max)
        }
    }

    pub fn random_unit_vector() -> Vec3{
        loop {
            let p = Vec3::random();
            let lensq = p.length_squared();

            if 1e-160 < lensq && lensq <= 1.0 {
                return p / lensq.sqrt();
            }
        }
    }

    pub fn random_on_hemisphere(normal: &Vec3) -> Vec3 {
        let on_unit_sphere = Vec3::random_unit_vector();

        if on_unit_sphere.dot(*normal) > 0.0 {
            on_unit_sphere
        }
        else {
            -on_unit_sphere
        }
        
    }

    pub fn reflect(v : &Vec3,n: &Vec3) -> Vec3 {
        *v - (2.0*v.dot(*n)) * *n
    }

    pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f64) -> Vec3 {
        
        let cos_theta = f64::min((-*uv).dot(*n), 1.0);
        let r_out_perp = etai_over_etat * (*uv + cos_theta * *n);
        let r_out_parallel = -f64::sqrt((1.0 - r_out_perp.length_squared()).abs()) * *n;

        r_out_perp + r_out_parallel
    }

    //getters
    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn z(&self) -> f64 {
        self.z
    }
}

// Associate Methods
impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output{
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z 
        }
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self,other:Vec3) -> Self::Output{
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl Add<i32> for Vec3 {
    type Output = Vec3;

    fn add(self, other: i32) -> Self::Output {
        Self {
            x: self.x + other as f64,
            y: self.y + other as f64,
            z: self.z + other as f64
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self,other:Vec3) -> Self::Output{
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
}

impl Div for Vec3 {
    type Output = Vec3;
    fn div(self,other:Vec3) -> Self::Output{
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z
        }
    }
}

impl Div<Vec3> for f64{
    type Output = Vec3;
    fn div(self,other:Vec3) -> Vec3{
        Vec3 {
            x: self / other.x,
            y: self / other.y,
            z: self / other.z
        }
    }
}

impl Div<f64> for Vec3{
    type Output = Vec3;
    fn div(self,other:f64) -> Vec3{
        Vec3 {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self,other:Vec3) -> Vec3{
        Vec3 {
            x: self * other.x,
            y: self * other.y,
            z: self * other.z
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self,scalar:f64) -> Self::Output{
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar
        }
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl AddAssign for Vec3 {

    fn add_assign(&mut self,other:Self) -> () {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self,other:Self) -> () {
        *self = Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z
        }
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self,other:Self) -> () {
        *self = Self {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z
        }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self,other:Self) -> () {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
}



#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_vec3_creation() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);
        assert_eq!(v.z, 3.0);
    }

    #[test]
    fn test_vec3_from_scalar() {
        let v: Vec3 = Vec3::from_scalar(3.0);
        assert_eq!(v.x,3.0);
        assert_eq!(v.y,3.0);
        assert_eq!(v.z,3.0);
    }

    #[test]
    fn test_vec3_length_squared() {
        let v1: Vec3 = Vec3::new(1.0, 1.0, 1.0);
        let v2: Vec3 = Vec3::new(2.0, 3.0, 4.0);
        let v3: Vec3 = Vec3::new(9.0, 1.0, 5.0);

        let v1_lenght = v1.length_squared();
        let v2_lenght = v2.length_squared();
        let v3_lenght = v3.length_squared();

        assert_eq!(v1_lenght,3.0);
        assert_eq!(v2_lenght,29.0);
        assert_eq!(v3_lenght,107.0);
    }

    #[test]
    fn test_vec3_unit_test() {
        let v1 = Vec3::new(3.0, 0.0, 0.0);
        let unit_v1 = v1.unit_vector();

        assert_eq!(unit_v1.x,1.0);
        assert_eq!(unit_v1.y,0.0);
        assert_eq!(unit_v1.z,0.0);
        assert_eq!(unit_v1.length(),1.0);
    }

    #[test]
    fn test_vec3_dot(){
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(1.0, 2.0, 3.0);
        let dot = v1.dot(v2);

        assert_eq!(dot,14.0);
    }
    
    #[test]
    fn test_vec3_cross_product(){
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(1.0, 2.0, 3.0);
        let cross1 = v1.cross(v2);
        assert_eq!(cross1.x,0.0);
        assert_eq!(cross1.y,0.0);
        assert_eq!(cross1.z,0.0);
    }

    #[test]
    fn test_vec3_neg(){
        let v1 = Vec3::from_scalar(2.0);
        let result = v1.neg();

        assert_eq!(result.x,-2.0);
        assert_eq!(result.y,-2.0);
        assert_eq!(result.z,-2.0);
    }

    #[test]
    fn test_vec3_add() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);
        let result = v1 + v2;
        assert_eq!(result.x, 5.0);
        assert_eq!(result.y, 7.0);
        assert_eq!(result.z, 9.0);
    }

    #[test]
    fn test_vec3_sub() {
        let v1 = Vec3::from_scalar(2.0);
        let v2 = Vec3::from_scalar(1.0);
        let result = v1 - v2;
        assert_eq!(result.x,1.0);
        assert_eq!(result.y,1.0);
        assert_eq!(result.z,1.0);
    }

    #[test]
    fn test_vec3_division() {
        let v = Vec3::new(2.0, 4.0, 6.0);
        let result = v / 2.0;
        assert_eq!(result, Vec3::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn test_negative_zero_handling() {
        let v = Vec3::new(-0.0, -0.0, 1.0);
        let unit = v.unit_vector();
        
        assert_eq!(unit.x, 0.0);
        assert_eq!(unit.y, 0.0);
        assert_eq!(unit.z, 1.0);
        
        // Test that length is still 1.0
        assert!((unit.length() - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_very_small_components() {
        let v = Vec3::new(1e-10, -1e-10, 1.0);
        let unit = v.unit_vector();
        
        assert_eq!(unit.x, 0.0);
        assert_eq!(unit.y, 0.0);
        assert!((unit.z - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_ranndom_on_hemisphere() {
        let normal = Vec3::new(0.0, 0.0, 1.0);

        for _ in 0..100 {
            let v = Vec3::random_on_hemisphere(&normal);

            // Test unit length
            assert!((v.length() - 1.0).abs() < 1e-6);
            assert!(v.dot(normal) >= 0.0);
        }
    }

    #[test]
    fn test_reflect() {
        // Test reflection off a horizontal surface
        let v = Vec3::new(1.0, -1.0, 0.0);
        let n = Vec3::new(0.0, 1.0, 0.0);
        let reflected = Vec3::reflect(&v, &n);
        
        // Expected: (1.0, 1.0, 0.0)
        assert!((reflected.x - 1.0).abs() < 1e-6);
        assert!((reflected.y - 1.0).abs() < 1e-6);
        assert!(reflected.z.abs() < 1e-6);

        // Test that reflection preserves length
        assert!((v.length() - reflected.length()).abs() < 1e-6);
    }

    #[test]
    fn test_refract() {
        // Test with a simple vertical ray hitting a horizontal surface
        let uv = Vec3::new(0.0, -1.0, 0.0);  // straight down
        let n = Vec3::new(0.0, 1.0, 0.0);    // surface normal pointing up
        let etai_over_etat = 1.5;            // typical glass-to-air ratio
        
        let refracted = Vec3::refract(&uv, &n, etai_over_etat);
        
        // Only test essential properties
        assert!(!refracted.near_zero(), "Refracted vector should not be zero");
        assert!(refracted.y < 0.0, "Refracted ray should continue downward");
        assert!((refracted.length() - 1.0).abs() < 1e-6, "Should be unit vector");
    }
}




