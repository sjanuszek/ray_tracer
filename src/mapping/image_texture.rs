use crate::{color::Color, interval::Interval, mapping::texture::Texture, rtw_stb_image::RtwImage};

pub struct ImageTexture {
    image: RtwImage
}

impl ImageTexture {
    pub fn new(filename: &str) -> Self {
        Self {
            image: RtwImage::new(filename)
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, p: crate::vec3::Point3) -> crate::color::Color {
        if self.image.height() <= 0 {
            return Color::new(0., 1., 1.);
        }

        let u = Interval::new(0., 1.).clamp(u);
        let v = 1. - Interval::new(0., 1.).clamp(v);

        let i = (u * self.image.width() as f64) as i32;
        let j = (v * self.image.height() as f64) as i32;
        let pixel = self.image.pixel_data(i, j);

        let color_scale = 1. / 255.;
        Color::new(color_scale*pixel[0] as f64, color_scale*pixel[1] as f64, color_scale*pixel[2] as f64)
    }
}
