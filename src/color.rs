use std::io::{self, Write};

use crate::{interval::Interval, vec3::Vec3};

pub type Color = Vec3;

pub const INTENSITY: Interval = Interval::new(0.000, 0.999);

pub fn write_color(out: &mut impl Write, pixel_color: Color) -> io::Result<()> {
    let r = pixel_color.x();
    let g = pixel_color.y();
    let b = pixel_color.z();

    let rbyte = (255.999 * INTENSITY.clamp(r)) as u64;
    let gbyte = (255.999 * INTENSITY.clamp(g)) as u64;
    let bbyte = (255.999 * INTENSITY.clamp(b)) as u64;

    writeln!(out, "{} {} {}", rbyte, gbyte, bbyte)
}
