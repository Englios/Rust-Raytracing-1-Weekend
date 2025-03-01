use crate::commons::vec3::Vec3;
use crate::interval::Interval;

pub type Color = Vec3;

pub fn write_color(pixel_color: &Color) -> (i32, i32, i32) {
    let r = pixel_color.x() as f64;
    let g = pixel_color.y() as f64;
    let b = pixel_color.z() as f64;

    let intensity = Interval::new(0.0, 0.999);

    let r_byte = (255.999 * intensity.clamp(r)) as i32;
    let g_byte = (255.999 * intensity.clamp(g)) as i32;
    let b_byte = (255.999 * intensity.clamp(b)) as i32;

    (r_byte, g_byte, b_byte)
}
