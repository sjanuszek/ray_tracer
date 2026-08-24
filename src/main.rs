mod vec3;
mod color;
mod ray;
mod geometry;
mod utility;
mod interval;
mod camera;
mod material;
mod mapping;
mod bvh;

use std::sync::Arc;

use crate::{bvh::bvh_node::BVHNode, camera::Camera, color::Color, geometry::{hittable_list::HittableList, sphere::Sphere}, mapping::checker_texture::CheckerTexture, material::{dielectric::Dielectric, lambertian::Lambertian, material::Material, metal::Metal}, utility::{random_double, random_double_range}, vec3::{Point3, Vec3}};

fn main() {
    let mut world: HittableList = HittableList::new();

    let checkered = CheckerTexture::new_from_color(0.32, Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9));
    world.add(Sphere::new(Point3::new(0., -1000., 0.), 1000., Arc::new(Lambertian::new_from_texture(Box::new(checkered)))));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double();
            let center = Point3::new(a as f64 + 0.9*random_double(), random_double_range(0.2, 2.3), b as f64 + 0.9 * random_double());

            if (center - Point3::new(4., 0.2, 0.)).length() > 0.9 {
                let sphere_material: Arc<dyn Material>;

                if choose_mat < 0.8 {
                    let albedo = Color::random() * Color::random();
                    sphere_material = Arc::new(Lambertian::new_from_color(albedo));
                    let center2 = center + Vec3::new(0., random_double_range(0., 0.5), 0.);
                    world.add(Sphere::new_moving(center, center2, 0.2, sphere_material));
                } else if choose_mat < 0.95 {
                    let albedo = Color::random_range(0.5, 1.);
                    let fuzz = random_double_range(0., 0.5);
                    sphere_material = Arc::new(Metal::new(albedo, fuzz));
                    world.add(Sphere::new(center, 0.2, sphere_material));
                } else {
                    sphere_material = Arc::new(Dielectric::new(1.5));
                    world.add(Sphere::new(center, 0.2, sphere_material));
                }
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Sphere::new(Point3::new(0., 1., 0.), 1., material1));

    let material2 = Arc::new(Lambertian::new_from_color(Color::new(0.4, 0.2, 0.1)));
    world.add(Sphere::new(Point3::new(-4., 1., 0.), 1., material2));

    let material3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.));
    world.add(Sphere::new(Point3::new(4., 1., 0.), 1., material3));

    let mut cam = Camera::default();

    world = HittableList::new_from_hittable(BVHNode::new(&mut world));

    cam.aspect_ratio = 16. / 9.;
    cam.image_width = 1000;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;
    
    cam.vfov = 20.;
    cam.lookfrom = Point3::new(13., 2., 3.);
    cam.lookat = Point3::new(0., 0., 0.);
    cam.vup = Vec3::new(0., 1., 0.);

    cam.defocus_angle = 0.6;
    cam.focus_dist = 10.;

    cam.jobs = 8;

    cam.render(&world);
}
