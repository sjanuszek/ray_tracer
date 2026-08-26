use crate::{color::Color, mapping::{perlin::Perlin, texture::Texture}};

pub struct NoiseTexture {
    noise: Perlin 
}

impl NoiseTexture {
    pub fn new() -> Self {
        Self {
            noise: Perlin::new()
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, u: f64, v: f64, p: crate::vec3::Point3) -> crate::color::Color {
        return Color::new(1., 1., 1.) * self.noise.noise(&p);
    }
}
