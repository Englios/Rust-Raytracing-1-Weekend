pub mod color;
pub mod vec3;
pub mod ray;

use rand;

pub const INFINITY:f64 = f64::INFINITY; 
pub const PI:f64 = 3.1415926535897932385;

pub fn degree_to_rad(degrees:f64) -> f64 {
    (degrees * PI) / 180.0
}

pub fn random_double() -> f64 {
    // Returns a random real in [0,1).
    rand::random::<f64>()
}

pub fn random_double_range(min:f64, max:f64) -> f64 {
    min + (max - min) * random_double()
}

