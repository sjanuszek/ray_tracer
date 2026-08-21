use std::io::{self, Write};

use crate::{color::{Color, write_color}, geometry::hittable::{HitRecord, Hittable}, interval::Interval, ray::Ray, utility::INFINITY, vec3::{Point3, Vec3, unit_vector}};

#[derive(Default)]
pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: u64,

    image_height: u64,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3
}

impl Camera {
    pub fn render(&mut self, world: &impl Hittable) {
        self.initialize();

        write!(io::stdout(), "P3\n {} {}\n255\n", self.image_width, self.image_height).unwrap();
        for j in 0..self.image_height {
            write!(io::stderr(), "\rScanlines remaining: {}", self.image_height - j).unwrap();
            io::stderr().flush().unwrap();
            for i in 0..self.image_width {
                let pixel_center = self.pixel00_loc + (i as f64 * self.pixel_delta_u) + (j as f64 * self.pixel_delta_v);
                let ray_direction = pixel_center - self.center;
                let r = Ray::new(&self.center, &ray_direction);

                let pixel_color = Camera::ray_color(&r, world);
                write_color(&mut io::stdout(), pixel_color).unwrap();
            }
        }

        write!(io::stderr(), "\rDone.               \n").unwrap();
    }

    fn initialize(&mut self) {
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as u64;
        self.image_height = if self.image_height < 1 { 1 } else { self.image_height };

        self.center = Point3::new(0., 0., 0.);

        let focal_length = 1.;
        let viewport_height = 2.;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);
        let viewport_u = Vec3::new(viewport_width, 0., 0.);
        let viewport_v = Vec3::new(0., -viewport_height, 0.);

        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        let viewport_upper_left = self.center - Vec3::new(0., 0., focal_length) - viewport_u/2. - viewport_v/2.;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
    }

    fn ray_color(r: &Ray, world: &impl Hittable) -> Color {
        let mut rec = HitRecord::default();
        if world.hit(r, Interval::new(0., INFINITY), &mut rec) {
            return 0.5 * (rec.normal + Color::new(1., 1., 1.));
        }

        let unit_direction = unit_vector(*r.direction());
        let a = 0.5*(unit_direction.y() + 1.);
        (1.-a)*Color::new(1., 1., 1.) + a*Color::new(0.5, 0.7, 1.)
    }
}
