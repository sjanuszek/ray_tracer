use rand::seq::SliceRandom;

use crate::{utility::random_double, vec3::Point3};

const POINT_COUNT: usize = 256;

pub struct Perlin {
    randfloat: [f64; POINT_COUNT],
    perm_x: [usize; POINT_COUNT],
    perm_y: [usize; POINT_COUNT],
    perm_z: [usize; POINT_COUNT]
}

impl Perlin {
    pub fn new() -> Self {
        let mut randfloat = [0.; POINT_COUNT];
        for float in randfloat.iter_mut() {
            *float = random_double();
        }
        
        let perm_x = Self::perlin_generate_perm();
        let perm_y = Self::perlin_generate_perm();
        let perm_z = Self::perlin_generate_perm();

        Self {
            randfloat,
            perm_x,
            perm_y,
            perm_z
        }
    }

    pub fn noise(&self, p: &Point3) -> f64 {
        let i = ((4. * p.x()) as i32 & 255) as usize;
        let j = ((4. * p.y()) as i32 & 255) as usize;
        let k = ((4. * p.z()) as i32 & 255) as usize;

        self.randfloat[self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k]]
    }

    fn perlin_generate_perm() -> [usize; POINT_COUNT] {
        let mut p = [0; POINT_COUNT];
        for i in 0..POINT_COUNT {
            p[i] = i
        };
        p.shuffle(&mut rand::rng());
        p
    }
}
