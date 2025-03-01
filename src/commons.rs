pub mod color;
pub mod vec3;
pub mod ray;

pub const INFINITY:f64 = f64::INFINITY; 
pub const PI:f64 = 3.1415926535897932385;

fn degree_to_rad(degrees:f64) -> f64 {
    (degrees * PI) / 180.0
}