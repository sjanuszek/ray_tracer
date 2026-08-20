mod vec3;
mod color;
mod ray;

use std::io::{self, Write};

use crate::color::{Color, write_color};

fn main() {
    let image_width = 256;
    let image_height = 256;

    write!(io::stdout(), "P3\n {image_width} {image_height}\n255\n").unwrap();
    for j in 0..image_height {
        write!(io::stderr(), "\rScanlines remaining: {}", image_height - j).unwrap();
        io::stderr().flush().unwrap();
        for i in 0..image_width {
            let pixel_color = Color::new((i as f64) / (image_width - 1) as f64, (j as f64) / (image_height - 1) as f64, 0.);
            write_color(&mut io::stdout(), pixel_color).unwrap();
        }
    }

    write!(io::stderr(), "\rDone.               \n").unwrap();
}
