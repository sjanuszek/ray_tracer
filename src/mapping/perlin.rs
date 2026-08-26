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
        let u = p.x() - p.x().floor();
        let v = p.y() - p.y().floor();
        let w = p.z() - p.z().floor();

        let i = p.x().floor() as i32;
        let j = p.y().floor() as i32;
        let k = p.z().floor() as i32;

        let mut c = [[[0.; 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.randfloat[
                        self.perm_x[((i+di as i32) & 255) as usize] ^
                        self.perm_y[((j+dj as i32) & 255) as usize] ^
                        self.perm_z[((k+dk as i32) & 255) as usize]
                    ]
                }
            }
        }

        Self::trilinear_interp(c, u, v, w)
    }

    fn trilinear_interp(c: [[[f64; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let mut accum = 0.;
        for i in 0..2 {
            let fi = i as f64;
            for j in 0..2 {
                let fj = j as f64;
                for k in 0..2 {
                    let fk = k as f64;
                    accum += (fi*u + (1.-fi)*(1.-u))
                           * (fj*v + (1.-fj)*(1.-v))
                           * (fk*w + (1.-fk)*(1.-w))
                           * c[i][j][k]
                }
            }
        }
        accum
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
