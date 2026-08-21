mod vec3;
mod color;
mod ray;
mod geometry;
mod utility;
mod interval;
mod camera;

use std::{io::{self, Write}};

use crate::{camera::Camera, color::{Color, write_color}, geometry::{hittable::{HitRecord, Hittable}, hittable_list::HittableList, sphere::Sphere}, interval::Interval, ray::Ray, utility::INFINITY, vec3::{Point3, Vec3, unit_vector}};

fn main() {
    let mut world: HittableList = HittableList::new();

    world.add(Sphere::new(Point3::new(0., 0., -1.), 0.5));
    world.add(Sphere::new(Point3::new(0., -100.5, -1.), 100.));

    let mut cam = Camera::default();

    cam.aspect_ratio = 16. / 9.;
    cam.image_width = 400;

    cam.render(&world);
}
