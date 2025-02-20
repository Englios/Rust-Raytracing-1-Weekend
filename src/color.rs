use crate::{interval::Interval, vec3::Vec3};
use std::io::{self, Write};

pub type Color = Vec3;

pub fn default() -> Vec3 {
    Vec3::new(0.0, 0.0, 0.0)
}

pub fn write_color<W: Write>(writer: &mut W, pixel_color: Color) -> io::Result<()> {

    let r: f64 = linear_to_gamma(pixel_color.x);
    let g: f64 = linear_to_gamma(pixel_color.y);
    let b: f64 = linear_to_gamma(pixel_color.z);

    // Convert from [0,1] to [0,255]
    let intensity = Interval::new(0.000, 0.999);
    
    let r_byte = (256 as f64* intensity.clamp(r)) as i32;
    let g_byte = (256 as f64 * intensity.clamp(g)) as i32;
    let b_byte = (256 as f64 * intensity.clamp(b)) as i32;

    writeln!(writer, "{} {} {}", r_byte, g_byte, b_byte)
}

pub fn linear_to_gamma(linear_component:f64) -> f64{

    if linear_component > 0.0 {
        linear_component.sqrt()
    } else {
        0.0
    }
}