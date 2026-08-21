mod vec3;
mod color;
mod ray;
mod geometry;
mod utility;
mod interval;
mod camera;
mod material;

use std::sync::Arc;

use crate::{camera::Camera, color::Color, geometry::{hittable_list::HittableList, sphere::Sphere}, material::{lambertian::Lambertian, metal::Metal}, vec3::Point3};

fn main() {
    let mut world: HittableList = HittableList::new();

    let material_ground = Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.)));
    let material_center = Arc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left =   Arc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.3));
    let material_right =  Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.));

    world.add(Sphere::new(Point3::new(0., -100.5, -1.), 100., material_ground));
    world.add(Sphere::new(Point3::new(0., 0., -1.2), 0.5, material_center));
    world.add(Sphere::new(Point3::new(-1., 0., -1.), 0.5, material_left));
    world.add(Sphere::new(Point3::new(1., 0., -1.), 0.5, material_right));

    let mut cam = Camera::default();

    cam.aspect_ratio = 16. / 9.;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;

    cam.render(&world);
}
