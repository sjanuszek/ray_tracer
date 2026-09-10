use std::{io::{self, Write}, sync::atomic::{AtomicU64, AtomicUsize, Ordering}};

use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::{color::{Color, write_color}, geometry::hittable::{HitRecord, Hittable}, interval::Interval, ray::Ray, utility::{INFINITY, degrees_to_radians, random_double}, vec3::{Point3, Vec3, cross, random_in_unit_disk, random_on_hemisphere, random_unit_vector, unit_vector}};

#[derive(Default)]
pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: usize,
    pub samples_per_pixel: usize,
    pub max_depth: usize,
    pub background: Color,

    pub vfov: f64,
    pub lookfrom: Point3,
    pub lookat: Point3,
    pub vup: Vec3,

    pub defocus_angle: f64,
    pub focus_dist: f64,

    pub jobs: usize,

    image_height: usize,
    pixel_samples_scale: f64,
    sqrt_spp: usize,
    recip_sqrt_spp: f64,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3
}

impl Camera {
    pub fn render(&mut self, world: &(impl Hittable + Sync)) {
        self.initialize();

        let this = &*self;

        let scanlines_remaining = AtomicUsize::new(this.image_height);

        let image_buffer: Vec<Color> = (0..this.image_height)
            .into_par_iter()
            .flat_map(|j| {
                let remaining = scanlines_remaining.fetch_sub(1, Ordering::Relaxed) - 1;
                
                if remaining % 10 == 0 || remaining == 0 {
                    eprint!("\rScanlines remaining: {:4} ", remaining);
                }

                (0..this.image_width).into_par_iter().map(move |i| this.render_pixel(i, j, world))
            })
            .collect();

        write!(io::stderr(), "\rDone.               \n").unwrap();

        let mut stdout = io::stdout().lock();
        write!(io::stdout(), "P3\n {} {}\n255\n", self.image_width, self.image_height).unwrap();
        for color in image_buffer {
            write_color(&mut stdout, color).unwrap();
        }
    }

    fn render_pixel(&self, i: usize, j: usize, world: &impl Hittable) -> Color {
        let mut pixel_color = Color::new(0., 0., 0.);
        for s_j in 0..self.sqrt_spp {
            for s_i in 0..self.sqrt_spp {
                let r = self.get_ray(i, j, s_i, s_j);
                pixel_color += self.ray_color(&r, self.max_depth, world);
            }
        }
        pixel_color * self.pixel_samples_scale
    }

    fn initialize(&mut self) {
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as usize;
        self.image_height = self.image_height.max(1);

        self.sqrt_spp = self.samples_per_pixel.isqrt();
        self.pixel_samples_scale = 1. / (self.sqrt_spp * self.sqrt_spp) as f64;
        self.recip_sqrt_spp = 1. / self.sqrt_spp as f64;

        self.center = self.lookfrom;

        let theta = degrees_to_radians(self.vfov);
        let h = f64::tan(theta/2.);
        let viewport_height = 2. * h * self.focus_dist;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);
        
        self.w = unit_vector(self.lookfrom - self.lookat);
        self.u = unit_vector(cross(self.vup, self.w));
        self.v = cross(self.w, self.u);

        let viewport_u = viewport_width * self.u;
        let viewport_v = viewport_height * -self.v;

        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        let viewport_upper_left = self.center - (self.focus_dist * self.w) - viewport_u/2. - viewport_v/2.;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);

        let defocus_radius = self.focus_dist * f64::tan((degrees_to_radians(self.defocus_angle / 2.)));
        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius;
    }

    fn get_ray(&self, i: usize, j: usize, s_i: usize, s_j: usize) -> Ray {
        let offset = self.sample_square_stratified(s_i, s_j);
        let pixel_sample = self.pixel00_loc
                         + ((i as f64 + offset.x()) * self.pixel_delta_u)
                         + ((j as f64 + offset.y()) * self.pixel_delta_v);

        let ray_origin = if self.defocus_angle <= 0. { self.center } else { self.defocus_disk_sample() };
        let ray_direction = pixel_sample - ray_origin;
        let ray_time = random_double();

        Ray::new_with_time(ray_origin, ray_direction, ray_time)
    }

    fn sample_square_stratified(&self, s_i: usize, s_j: usize) -> Vec3 {
        let px = ((s_i as f64 + random_double()) * self.recip_sqrt_spp) - 0.5;
        let py = ((s_j as f64 + random_double()) * self.recip_sqrt_spp) - 0.5;

        Vec3::new(px, py, 0.)
    }

    fn sample_square() -> Vec3 {
        Vec3::new(random_double() - 0.5, random_double() - 0.5, 0.)
    }

    fn defocus_disk_sample(&self) -> Point3 {
        let p = random_in_unit_disk();
        self.center + (p[0] * self.defocus_disk_u) + (p[1] * self.defocus_disk_v)
    }

    fn ray_color(&self, r: &Ray, depth: usize, world: &impl Hittable) -> Color {
        if depth <= 0 {
            return Color::new(0., 0., 0.);
        }

        let mut rec = HitRecord::default();

        if world.hit(r, Interval::new(0.001, INFINITY), &mut rec) {
            if let Some(mat) = &rec.mat {
                let color_from_emission = mat.emitted(rec.u, rec.v, rec.p);

                if let Some((attenuation, scattered)) = mat.scatter(r, &rec) {
                    let scattering_pdf = mat.scattering_pdf(r, &rec, &scattered);
                    let pdf_value = scattering_pdf;

                    let color_from_scatter = (attenuation * scattering_pdf * self.ray_color(&scattered, depth - 1, world)) / pdf_value;

                    return color_from_emission + color_from_scatter;
                }

                return color_from_emission;
            }
        } 

        self.background
    }
}
