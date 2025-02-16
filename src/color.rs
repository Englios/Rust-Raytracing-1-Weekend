use crate::vec3::Vec3;
use std::io::{self, Write};

pub type Color = Vec3;

pub fn write_color<W: Write>(writer: &mut W, pixel_color: Color) -> io::Result<()> {
    let r: f64 = pixel_color.x;
    let g: f64 = pixel_color.y;
    let b: f64 = pixel_color.z;

    // Convert from [0,1] to [0,255]
    let r_byte = (255.999 * r) as i32;
    let g_byte = (255.999 * g) as i32;
    let b_byte = (255.999 * b) as i32;

    writeln!(writer, "{} {} {}", r_byte, g_byte, b_byte)
}