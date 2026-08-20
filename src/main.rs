mod vec3;

use std::io::{self, Write};

use crate::vec3::Vec3;

fn main() {
    let image_width = 256;
    let image_height = 256;

    write!(io::stdout(), "P3\n {image_width} {image_height}\n255\n").unwrap();
    for j in 0..image_height {
        write!(io::stderr(), "\rScanlines remaining: {}", image_height - j).unwrap();
        io::stderr().flush().unwrap();
        for i in 0..image_width {
            let r = (i as f64) / (image_width - 1) as f64;
            let g = (j as f64) / (image_width - 1) as f64;
            let b = 0.;

            let ir = (255.999 * r) as u32;
            let ig = (255.999 * g) as u32;
            let ib = (255.999 * b) as u32;

            write!(io::stdout(), "{ir} {ig} {ib}\n").unwrap();
        }
    }

    write!(io::stderr(), "\rDone.               \n").unwrap();
}
