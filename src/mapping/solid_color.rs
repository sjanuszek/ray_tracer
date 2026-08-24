use crate::{color::Color, mapping::texture::Texture};

pub struct SolidColor {
    albedo: Color
}

impl SolidColor {
    pub fn new_from_color_values(red: f64, green: f64, blue: f64) -> Self {
        Self {
            albedo: Color::new(red, green, blue)
        }
    }

    pub fn new_from_color(albedo: Color) -> Self {
        Self {
            albedo
        }
    }
}

impl Texture for SolidColor {
    fn value(&self, u: f64, v: f64, p: crate::vec3::Point3) -> Color {
        return self.albedo;
    }
}
