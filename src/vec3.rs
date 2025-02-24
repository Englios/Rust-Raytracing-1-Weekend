use std::ops::{
    Add, AddAssign, Div, Mul, MulAssign, Neg, DivAssign, Sub, SubAssign
};

#[derive(Debug, Clone, Copy,PartialEq)]
pub struct Vec3 {
    e : [f64; 3] 
}

pub type Point3 = Vec3;


impl Default for Vec3 {
    fn default() -> Self {
        Vec3::new(0.0, 0.0, 0.0)
    }
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { e: [x, y, z] }
    }

    //Getters
    pub fn x(&self) -> f64 {
        self.e[0]
    }

    pub fn y(&self) -> f64 {
        self.e[1]
    }
    
    pub fn z(&self) -> f64 {
        self.e[2]
    }

    // Vector Ops
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.x().powi(2) + self.y().powi(2) + self.z().powi(2)
    }

    pub fn dot(&self, other: Vec3) -> f64 {
        self.x() * other.x() + self.y() * other.y() + self.z() * other.z()
    }

    pub fn cross(&self, other: Vec3) -> Vec3 {
        Vec3::new(
            self.y() * other.z() - self.z() * other.y(),
            self.z() * other.x() - self.x() * other.z(),
            self.x() * other.y() - self.y() * other.x()
        )
    }

    pub fn unit_vector(&self) -> Vec3 {
        self / self.length()
    }
}



// Operators for Vec3
impl Neg for Vec3 {
    
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3::new(-self.x(), -self.y(), -self.z())
    }
}

// Additions
impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        self.e[0] += other.x();
        self.e[1] += other.y();
        self.e[2] += other.z();
    }
}

impl AddAssign<f64> for Vec3 {
    fn add_assign(&mut self, other: f64) {
        self.e[0] += other;
        self.e[1] += other;
        self.e[2] += other;
    }
    
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Self::Output {
        Vec3::new(self.x() + other.x(), self.y() + other.y(), self.z() + other.z())
    }
}

impl Add<f64> for Vec3 {
    type Output = Vec3;

    fn add(self, other: f64) -> Self::Output {
        Vec3::new(self.x() + other, self.y() + other, self.z() + other)
    }
}

// Multiplication
impl MulAssign for Vec3 {
    fn mul_assign(&mut self, other: Vec3) {
        self.e[0] *= other.x();
        self.e[1] *= other.y();
        self.e[2] *= other.z();
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, other: f64) {
        self.e[0] *= other;
        self.e[1] *= other;
        self.e[2] *= other;
    }
    
}

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Self::Output {
        Vec3::new(
            self.x() * other.x(), 
            self.y() * other.y(), 
            self.z() * other.z()
        )
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: f64) -> Self::Output {
        Vec3::new(
            self.x() * other, 
            self.y() * other, 
            self.z() * other
        )
    }
}

impl Mul<f64> for &Vec3 {
    type Output = Vec3;

    fn mul(self, other: f64) -> Self::Output {
        Vec3::new(
            self.x() * other, 
            self.y() * other, 
            self.z() * other
        )
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Self::Output {
        other * self
    }
}

// Division
impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, other: f64) {
        self.e[0] /= other;
        self.e[1] /= other;
        self.e[2] /= other;
    }
}

impl Div for Vec3 {
    type Output = Vec3;

    fn div(self, other: Vec3) -> Self::Output {
        Vec3::new(
            self.x() / other.x(), 
            self.y() / other.y(), 
            self.z() / other.z()
        )
    }
}

impl Div<Vec3> for f64 {
    type Output = Vec3;

    fn div(self, other: Vec3) -> Self::Output {
        Vec3::new(
            self / other.x(), 
            self / other.y(), 
            self / other.z()
        )
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, other: f64) -> Self::Output {
        Vec3::new(
            self.x() / other, 
            self.y() / other, 
            self.z() / other
        )
    }
}

impl Div<f64> for &Vec3 {
    type Output = Vec3;

    fn div(self, other: f64) -> Self::Output {
        Vec3::new(
            self.x() / other,
            self.y() / other,
            self.z() / other
        )
    }
}

impl DivAssign<Vec3> for Vec3 {
    fn div_assign(&mut self, other: Vec3) {
        self.e[0] /= other.x();
        self.e[1] /= other.y();
        self.e[2] /= other.z();
    }
}




// Subtraction
impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Self::Output {
        Vec3::new(
            self.x() - other.x(), 
            self.y() - other.y(), 
            self.z() - other.z()
        )
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Vec3) {
        self.e[0] -= other.x();
        self.e[1] -= other.y();
        self.e[2] -= other.z();
    }
}

impl SubAssign<f64> for Vec3 {
    fn sub_assign(&mut self, other: f64) {
        self.e[0] -= other;
        self.e[1] -= other;
        self.e[2] -= other;
    }
    
}

impl Sub<f64> for Vec3 {
    type Output = Vec3;

    fn sub(self, other: f64) -> Self::Output {
        Vec3::new(
            self.x() - other, 
            self.y() - other, 
            self.z() - other
        )
    }
}

impl Sub<Vec3> for f64 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Self::Output {
        Vec3::new(
            self - other.x(), 
            self - other.y(), 
            self - other.z()
        )
    }
}

impl Sub<f64> for &Vec3 {
    type Output = Vec3;

    fn sub(self, other: f64) -> Self::Output {
        Vec3::new(
            self.x() - other,
            self.y() - other,
            self.z() - other
        )
    }
}

impl Sub<Vec3> for &Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Self::Output {
        Vec3::new(
            self.x() - other.x(),
            self.y() - other.y(),
            self.z() - other.z()
        )
    }
}

impl Sub<&Vec3> for f64 {
    type Output = Vec3;

    fn sub(self, other: &Vec3) -> Self::Output {
        Vec3::new(
            self - other.x(),
            self - other.y(),
            self - other.z()
        )
    }
}

impl Sub<&Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, other: &Vec3) -> Self::Output {
        Vec3::new(
            self.x() - other.x(),
            self.y() - other.y(),
            self.z() - other.z()
        )
    }
}

// Tests
#[cfg(test)] 
mod tests {

    use super::*;

    #[test]
    fn test_vec3_default() {
        let vec = Vec3::default();
        assert_eq!(vec.x(), 0.0);
        assert_eq!(vec.y(), 0.0);
        assert_eq!(vec.z(), 0.0);
    }

    #[test]
    fn test_vec3_new() {
        let vec = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(vec.x(), 1.0);
        assert_eq!(vec.y(), 2.0);
        assert_eq!(vec.z(), 3.0);
    }

    #[test]
    fn test_vec3_add() {
        let vec1 = Vec3::new(1.0, 2.0, 3.0);
        let vec2 = Vec3::new(4.0, 5.0, 6.0);
        let vec3 = vec1 + vec2;
        assert_eq!(vec3.x(), 5.0);
        assert_eq!(vec3.y(), 7.0);
        assert_eq!(vec3.z(), 9.0);
    }

    #[test]
    fn test_vec3_add_assign() {
        let mut vec1 = Vec3::new(1.0, 2.0, 3.0);
        let vec2 = Vec3::new(4.0, 5.0, 6.0);
        vec1 += vec2;
        assert_eq!(vec1.x(), 5.0);
        assert_eq!(vec1.y(), 7.0);
        assert_eq!(vec1.z(), 9.0);
    }

    #[test]
    fn test_vec3_add_scalar() {
        let vec1 = Vec3::new(1.0, 2.0, 3.0);
        let vec2 = vec1 + 1.0;
        assert_eq!(vec2.x(), 2.0);
        assert_eq!(vec2.y(), 3.0);
        assert_eq!(vec2.z(), 4.0);
    }

    #[test]
    fn test_vec3_add_assign_scalar() {
        let mut vec1 = Vec3::new(1.0, 2.0, 3.0);
        vec1 += 1.0;
        assert_eq!(vec1.x(), 2.0);
        assert_eq!(vec1.y(), 3.0);
        assert_eq!(vec1.z(), 4.0);
    }

    #[test]
    fn test_vec3_mul() {
        let vec1 = Vec3::new(1.0, 2.0, 3.0);
        let vec2 = Vec3::new(4.0, 5.0, 6.0);
        let vec3 = vec1 * vec2;
        assert_eq!(vec3.x(), 4.0);
        assert_eq!(vec3.y(), 10.0);
        assert_eq!(vec3.z(), 18.0);
    }

    #[test]
    fn test_vec3_mul_assign() {
        let mut vec1 = Vec3::new(1.0, 2.0, 3.0);
        let vec2 = Vec3::new(4.0, 5.0, 6.0);
        vec1 *= vec2;
        assert_eq!(vec1.x(), 4.0);
        assert_eq!(vec1.y(), 10.0);
        assert_eq!(vec1.z(), 18.0);
    }

    #[test]
    fn test_vec3_mul_scalar() {
        let vec1 = Vec3::new(1.0, 2.0, 3.0);
        let vec2 = vec1 * 2.0;
        assert_eq!(vec2.x(), 2.0);
        assert_eq!(vec2.y(), 4.0);
        assert_eq!(vec2.z(), 6.0);
    }

    #[test]
    fn test_vec3_mul_assign_scalar() {
        let mut vec1 = Vec3::new(1.0, 2.0, 3.0);
        vec1 *= 2.0;
        assert_eq!(vec1.x(), 2.0);
        assert_eq!(vec1.y(), 4.0);
        assert_eq!(vec1.z(), 6.0);
    }

    #[test]
    fn test_vec3_neg() {
        let vec1 = Vec3::new(1.0, 2.0, 3.0);
        let vec2 = -vec1;
        assert_eq!(vec2.x(), -1.0);
        assert_eq!(vec2.y(), -2.0);
        assert_eq!(vec2.z(), -3.0);
    }
    
    #[test]
    fn test_vec3_sub() {
        let vec1 = Vec3::new(1.0, 2.0, 3.0);
        let vec2 = Vec3::new(4.0, 5.0, 6.0);
        let vec3 = vec1 - vec2;
        assert_eq!(vec3.x(), -3.0);
        assert_eq!(vec3.y(), -3.0);
        assert_eq!(vec3.z(), -3.0);
    }

    #[test]
    fn test_vec3_sub_assign() {
        let mut vec1 = Vec3::new(1.0, 2.0, 3.0);
        let vec2 = Vec3::new(4.0, 5.0, 6.0);
        vec1 -= vec2;
        assert_eq!(vec1.x(), -3.0);
        assert_eq!(vec1.y(), -3.0);
        assert_eq!(vec1.z(), -3.0);
    }

    #[test]
    fn test_vec3_sub_scalar() {
        let vec1 = Vec3::new(1.0, 2.0, 3.0);
        let vec2 = vec1 - 1.0;
        assert_eq!(vec2.x(), 0.0);
        assert_eq!(vec2.y(), 1.0);
        assert_eq!(vec2.z(), 2.0);
    }

    #[test]
    fn test_vec3_sub_assign_scalar() {
        let mut vec1 = Vec3::new(1.0, 2.0, 3.0);
        vec1 -= 1.0;
        assert_eq!(vec1.x(), 0.0);
        assert_eq!(vec1.y(), 1.0);
        assert_eq!(vec1.z(), 2.0);
    }

    #[test]
    fn test_vec3_div() {
        let vec1 = Vec3::new(1.0, 2.0, 3.0);
        let vec2 = Vec3::new(4.0, 5.0, 6.0);
        let vec3 = vec1 / vec2;
        assert_eq!(vec3.x(), 0.25);
        assert_eq!(vec3.y(), 0.4);
        assert_eq!(vec3.z(), 0.5);
    }

    #[test]
    fn test_vec3_div_assign() {
        let mut vec1 = Vec3::new(1.0, 2.0, 3.0);
        let vec2 = Vec3::new(4.0, 5.0, 6.0);
        vec1 /= vec2;
        assert_eq!(vec1.x(), 0.25);
        assert_eq!(vec1.y(), 0.4);
        assert_eq!(vec1.z(), 0.5);
    }

    #[test]
    fn test_vec3_div_scalar() {
        let vec1 = Vec3::new(1.0, 2.0, 3.0);
        let vec2 = vec1 / 2.0;
        assert_eq!(vec2.x(), 0.5);
        assert_eq!(vec2.y(), 1.0);
        assert_eq!(vec2.z(), 1.5);
    }

    #[test]
    fn test_vec3_div_assign_scalar() {
        let mut vec1 = Vec3::new(1.0, 2.0, 3.0);
        vec1 /= 2.0;
        assert_eq!(vec1.x(), 0.5);
        assert_eq!(vec1.y(), 1.0);
        assert_eq!(vec1.z(), 1.5);
    }

    #[test]
    fn test_vec3_cross() {
        let vec1 = Vec3::new(1.0, 0.0, 0.0);
        let vec2 = Vec3::new(0.0, 1.0, 0.0);
        let vec3 = vec1.cross(vec2);
        assert_eq!(vec3.x(), 0.0);
        assert_eq!(vec3.y(), 0.0);
        assert_eq!(vec3.z(), 1.0);
    }

    #[test]
    fn test_vec3_dot() {
        let vec1 = Vec3::new(1.0, 2.0, 3.0);
        let vec2 = Vec3::new(4.0, 5.0, 6.0);
        let dot = vec1.dot(vec2);
        assert_eq!(dot, 32.0);
    }

    #[test]
    fn test_vec3_length() {
        let vec = Vec3::new(1.0, 2.0, 3.0);
        let length = vec.length();
        assert_eq!(length, 14.0_f64.sqrt());
    }

    #[test]
    fn test_vec3_length_squared() {
        let vec = Vec3::new(1.0, 2.0, 3.0);
        let length_squared = vec.length_squared();
        assert_eq!(length_squared, 14.0);
    }

    #[test]
    fn test_vec3_unit_vector() {
        let vec = Vec3::new(1.0, 2.0, 3.0);
        let unit_vector = vec.unit_vector();
        let length = vec.length();
        assert_eq!(unit_vector.x(), 1.0/length);
        assert_eq!(unit_vector.y(), 2.0/length);
        assert_eq!(unit_vector.z(), 3.0/length);
    }

    #[test]
    fn test_vec3_div_scalar_ref() {
        let vec = Vec3::new(1.0, 2.0, 3.0);
        let vec2 = &vec / 2.0;
        assert_eq!(vec2.x(), 0.5);
        assert_eq!(vec2.y(), 1.0);
        assert_eq!(vec2.z(), 1.5);
    }

    #[test]
    fn test_vec3_mul_scalar_ref() {
        let vec = Vec3::new(1.0, 2.0, 3.0);
        let vec2 = &vec * 2.0;
        assert_eq!(vec2.x(), 2.0);
        assert_eq!(vec2.y(), 4.0);
        assert_eq!(vec2.z(), 6.0);
    }

}