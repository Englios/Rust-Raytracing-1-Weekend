use std::ops::{
                Add, 
                Sub, 
                Mul, 
                Div,
                AddAssign, 
                MulAssign, 
                DivAssign, 
                SubAssign, 
                Neg
            };


#[derive(Debug, Clone, Copy,PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
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

    pub fn unit_vector(&self) -> Self{
        let len = self.length();
        Self {
            x: self.x/len,
            y: self.y/len,
            z: self.z/len
        }
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
}




