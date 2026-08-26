use crate::{color::Color, mapping::{perlin::Perlin, texture::Texture}};

pub struct NoiseTexture {
    noise: Perlin,
    scale: f64
}

impl NoiseTexture {
    pub fn new(scale: f64) -> Self {
        Self {
            noise: Perlin::new(),
            scale
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, u: f64, v: f64, p: crate::vec3::Point3) -> crate::color::Color {
        return Color::new(1., 1., 1.) * self.noise.turb(&p, 7);
    }
}
