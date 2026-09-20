use std::io::{self, Write};

use crate::{interval::Interval, vec3::Vec3};

pub type Color = Vec3;

pub const INTENSITY: Interval = Interval::new(0.000, 0.999);

fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0. {
        return linear_component.sqrt();
    }

    return 0.;
}

pub fn write_color(out: &mut impl Write, pixel_color: Color) -> io::Result<()> {
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();

    if r != r { r = 0. };
    if g != g { g = 0. };
    if b != b { b = 0. };

    r = linear_to_gamma(r);
    g = linear_to_gamma(g);
    b = linear_to_gamma(b);

    let rbyte = (255.999 * INTENSITY.clamp(r)) as u64;
    let gbyte = (255.999 * INTENSITY.clamp(g)) as u64;
    let bbyte = (255.999 * INTENSITY.clamp(b)) as u64;

    writeln!(out, "{} {} {}", rbyte, gbyte, bbyte)
}
