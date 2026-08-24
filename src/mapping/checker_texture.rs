use crate::{color::Color, mapping::{solid_color::SolidColor, texture::Texture}};

pub struct CheckerTexture {
    inv_scale: f64,
    even: Box<dyn Texture>,
    odd: Box<dyn Texture>,
}

impl CheckerTexture {
    pub fn new_from_texture(scale: f64, even: Box<dyn Texture>, odd: Box<dyn Texture>) -> Self {
        Self {
            inv_scale: 1./scale,
            even,
            odd
        }
    }
    pub fn new_from_color(scale: f64, c1: Color, c2: Color) -> Self {
        Self::new_from_texture(scale, Box::new(SolidColor::new_from_color(c1)), Box::new(SolidColor::new_from_color(c2)))
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: crate::vec3::Point3) -> Color {
        let x_int = (self.inv_scale * p.x()).floor() as i32;
        let y_int = (self.inv_scale * p.y()).floor() as i32;
        let z_int = (self.inv_scale * p.z()).floor() as i32;

        let is_even = (x_int + y_int + z_int) % 2 == 0;

        if is_even { self.even.value(u, v, p) } else { self.odd.value(u, v, p) }
    }
}
