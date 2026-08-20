mod vec3;
mod color;
mod ray;

use std::io::{self, Write};

use crate::{color::{Color, write_color}, ray::Ray, vec3::{Point3, Vec3, dot, unit_vector}};

fn hit_sphere(center: Point3, radius: f64, r: &Ray) -> f64 {
    let oc = center - *r.origin();
    let a = r.direction().length_squared();
    let h = dot(*r.direction(), oc);
    let c = oc.length_squared() - radius*radius;
    let discriminant = h*h - a*c;

    if discriminant < 0. {
        return -1.0;
    } else {
        return (h- discriminant.sqrt()) / a;
    }
}

fn ray_color(r: Ray) -> Color {
    let t = hit_sphere(Point3::new(0., 0., -1.), 0.5, &r);
    if t > 0. {
        let n = unit_vector(r.at(t) - Vec3::new(0., 0., -1.));
        return 0.5 * Color::new(n.x()+1., n.y()+1., n.z()+1.);
    }
    let unit_direction = unit_vector(*r.direction());
    let a = 0.5*(unit_direction.y() + 1.);
    (1.-a)*Color::new(1., 1., 1.) + a*Color::new(0.5, 0.7, 1.)
}

fn main() {
    let aspect_ratio = 16. / 9.;
    let image_width = 400;

    let mut image_height = (image_width as f64 / aspect_ratio) as u64;
    image_height = if image_height < 1 { 1 } else { image_height };

    let focal_length = 1.;
    let viewport_height = 2.;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center = Point3::default();

    let viewport_u = Vec3::new(viewport_width, 0., 0.);
    let viewport_v = Vec3::new(0., -viewport_height, 0.);

    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    let viewport_upper_left = camera_center - Vec3::new(0., 0., focal_length) - viewport_u/2. - viewport_v/2.;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    write!(io::stdout(), "P3\n {image_width} {image_height}\n255\n").unwrap();
    for j in 0..image_height {
        write!(io::stderr(), "\rScanlines remaining: {}", image_height - j).unwrap();
        io::stderr().flush().unwrap();
        for i in 0..image_width {
            let pixel_center = pixel00_loc + (i as f64 * pixel_delta_u) + (j as f64 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let r = Ray::new(&camera_center, &ray_direction);

            let pixel_color = ray_color(r);
            write_color(&mut io::stdout(), pixel_color).unwrap();
        }
    }

    write!(io::stderr(), "\rDone.               \n").unwrap();
}
