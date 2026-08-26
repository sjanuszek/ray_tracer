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
mod rtw_stb_image;

use std::{env, sync::Arc};

use crate::{bvh::bvh_node::BVHNode, camera::Camera, color::Color, geometry::{hittable_list::HittableList, quad::Quad, sphere::Sphere}, mapping::{checker_texture::CheckerTexture, image_texture::ImageTexture, noise_texture::NoiseTexture}, material::{dielectric::Dielectric, lambertian::Lambertian, material::Material, metal::Metal}, utility::{random_double, random_double_range}, vec3::{Point3, Vec3}};

fn bouncing_spheres(image_width: usize, samples: usize, max_depth: usize) {
    let mut world: HittableList = HittableList::new();

    let checkered = Arc::new(CheckerTexture::new_from_color(0.32, Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9)));
    world.add(Sphere::new(Point3::new(0., -1000., 0.), 1000., Arc::new(Lambertian::new_from_texture(checkered))));

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
    cam.image_width = image_width;
    cam.samples_per_pixel = samples;
    cam.max_depth = max_depth;
    
    cam.vfov = 20.;
    cam.lookfrom = Point3::new(13., 2., 3.);
    cam.lookat = Point3::new(0., 0., 0.);
    cam.vup = Vec3::new(0., 1., 0.);

    cam.defocus_angle = 0.6;
    cam.focus_dist = 10.;

    cam.jobs = 8;

    cam.render(&world);
}

fn checkered_spheres(image_width: usize, samples: usize, max_depth: usize) {
    let mut world: HittableList = HittableList::new();

    let checkered = Arc::new(CheckerTexture::new_from_color(0.32, Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9)));

    world.add(Sphere::new(Point3::new(0., -10., 0.), 10., Arc::new(Lambertian::new_from_texture(checkered.clone()))));
    world.add(Sphere::new(Point3::new(0., 10., 0.), 10., Arc::new(Lambertian::new_from_texture(checkered))));

    let mut cam = Camera::default();

    world = HittableList::new_from_hittable(BVHNode::new(&mut world));

    cam.aspect_ratio = 16. / 9.;
    cam.image_width = image_width;
    cam.samples_per_pixel = samples;
    cam.max_depth = max_depth;
    
    cam.vfov = 20.;
    cam.lookfrom = Point3::new(13., 2., 3.);
    cam.lookat = Point3::new(0., 0., 0.);
    cam.vup = Vec3::new(0., 1., 0.);

    cam.defocus_angle = 0.0;
    cam.focus_dist = 10.;

    cam.jobs = 8;

    cam.render(&world);
}

fn earth(image_width: usize, samples: usize, max_depth: usize) {
    let mut world: HittableList = HittableList::new();

    let earth_texture = Arc::new(ImageTexture::new("earthmap.jpg"));
    let earth_surface = Arc::new(Lambertian::new_from_texture(earth_texture));
    let globe = Sphere::new(Point3::new(0., 0., 0.), 2., earth_surface);

    world.add(globe);

    let mut cam = Camera::default();

    world = HittableList::new_from_hittable(BVHNode::new(&mut world));

    cam.aspect_ratio = 16. / 9.;
    cam.image_width = image_width;
    cam.samples_per_pixel = samples;
    cam.max_depth = max_depth;
    
    cam.vfov = 20.;
    cam.lookfrom = Point3::new(13., 2., 3.);
    cam.lookat = Point3::new(0., 0., 0.);
    cam.vup = Vec3::new(0., 1., 0.);

    cam.defocus_angle = 0.0;
    cam.focus_dist = 10.;

    cam.jobs = 8;

    cam.render(&world);
}

fn perlin_spheres(image_width: usize, samples: usize, max_depth: usize) {
    let mut world: HittableList = HittableList::new();

    let pertext = Arc::new(NoiseTexture::new(4.));
    let material = Arc::new(Lambertian::new_from_texture(pertext));
    world.add(Sphere::new(Point3::new(0., -1000., 0.), 1000., material.clone()));
    world.add(Sphere::new(Point3::new(0., 2., 0.), 2., material));

    let mut cam = Camera::default();

    world = HittableList::new_from_hittable(BVHNode::new(&mut world));

    cam.aspect_ratio = 16. / 9.;
    cam.image_width = image_width;
    cam.samples_per_pixel = samples;
    cam.max_depth = max_depth;
    
    cam.vfov = 20.;
    cam.lookfrom = Point3::new(13., 2., 3.);
    cam.lookat = Point3::new(0., 0., 0.);
    cam.vup = Vec3::new(0., 1., 0.);

    cam.defocus_angle = 0.0;
    cam.focus_dist = 10.;

    cam.jobs = 8;

    cam.render(&world);
}

fn quads(image_width: usize, samples: usize, max_depth: usize) {
    let mut world: HittableList = HittableList::new();

    let left_red = Arc::new(Lambertian::new_from_color(Color::new(1., 0.2, 0.2)));
    let back_green = Arc::new(Lambertian::new_from_color(Color::new(0.2, 1., 0.2)));
    let right_blue = Arc::new(Lambertian::new_from_color(Color::new(0.2, 0.2, 1.)));
    let upper_orange = Arc::new(Lambertian::new_from_color(Color::new(1., 0.5, 0.)));
    let lower_teal = Arc::new(Lambertian::new_from_color(Color::new(0.2, 0.8, 0.8)));

    world.add(Quad::new(Point3::new(-3., -2., 5.), Vec3::new(0., 0., -4.), Vec3::new(0., 4., 0.), left_red));
    world.add(Quad::new(Point3::new(-2., -2., 0.), Vec3::new(4., 0., 0.), Vec3::new(0., 4., 0.), back_green));
    world.add(Quad::new(Point3::new(3., -2., 1.), Vec3::new(0., 0., 4.), Vec3::new(0., 4., 0.), right_blue));
    world.add(Quad::new(Point3::new(-2., 3., 1.), Vec3::new(4., 0., 0.), Vec3::new(0., 0., 4.), upper_orange));
    world.add(Quad::new(Point3::new(-2., -3., 5.), Vec3::new(4., 0., 0.), Vec3::new(0., 0., -4.), lower_teal));

    let mut cam = Camera::default();

    world = HittableList::new_from_hittable(BVHNode::new(&mut world));

    cam.aspect_ratio = 1.;
    cam.image_width = image_width;
    cam.samples_per_pixel = samples;
    cam.max_depth = max_depth;
    
    cam.vfov = 80.;
    cam.lookfrom = Point3::new(0., 0., 9.);
    cam.lookat = Point3::new(0., 0., 0.);
    cam.vup = Vec3::new(0., 1., 0.);

    cam.defocus_angle = 0.0;
    cam.focus_dist = 10.;

    cam.jobs = 8;

    cam.render(&world);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let scene = &args[1].parse::<usize>().unwrap();
    let image_width = &args[2].parse::<usize>().unwrap();
    let samples = &args[3].parse::<usize>().unwrap();
    let depth = &args[4].parse::<usize>().unwrap();

    match scene {
        0 => bouncing_spheres(*image_width, *samples, *depth),
        1 => checkered_spheres(*image_width, *samples, *depth),
        2 => earth(*image_width, *samples, *depth),
        3 => perlin_spheres(*image_width, *samples, *depth),
        4 => quads(*image_width, *samples, *depth),
        _ => ()
    }
}
