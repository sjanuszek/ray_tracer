use rand::seq::SliceRandom;

use crate::{utility::random_double, vec3::{Point3, Vec3, dot, unit_vector}};

const POINT_COUNT: usize = 256;

pub struct Perlin {
    randvec: [Vec3; POINT_COUNT],
    perm_x: [usize; POINT_COUNT],
    perm_y: [usize; POINT_COUNT],
    perm_z: [usize; POINT_COUNT]
}

impl Perlin {
    pub fn new() -> Self {
        let mut randvec = [Vec3::default(); POINT_COUNT];
        for vec in randvec.iter_mut() {
            *vec = unit_vector(Vec3::random_range(-1., 1.));
        }
        
        let perm_x = Self::perlin_generate_perm();
        let perm_y = Self::perlin_generate_perm();
        let perm_z = Self::perlin_generate_perm();

        Self {
            randvec,
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

        let mut c = [[[Vec3::default(); 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.randvec[
                        self.perm_x[((i+di as i32) & 255) as usize] ^
                        self.perm_y[((j+dj as i32) & 255) as usize] ^
                        self.perm_z[((k+dk as i32) & 255) as usize]
                    ]
                }
            }
        }

        Self::perlin_interp(c, u, v, w)
    }

    pub fn turb(&self, p: &Point3, depth: usize) -> f64 {
        let mut accum = 0.;
        let mut temp_p = p.clone();
        let mut weight = 1.;

        for i in 0..depth {
            accum += weight * self.noise(&temp_p);
            weight *= 0.5;
            temp_p *= 2.;
        }

        accum.abs()
    }

    fn perlin_interp(c: [[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let uu = u*u*(3.-2.*u);
        let vv = v*v*(3.-2.*v);
        let ww = w*w*(3.-2.*w);
        let mut accum = 0.;

        for i in 0..2 {
            let fi = i as f64;
            for j in 0..2 {
                let fj = j as f64;
                for k in 0..2 {
                    let fk = k as f64;
                    let weight_v = Vec3::new(u-fi, v-fj, w-fk);
                    accum += (fi*uu + (1.-fi)*(1.-uu))
                           * (fj*vv + (1.-fj)*(1.-vv))
                           * (fk*ww + (1.-fk)*(1.-ww))
                           * dot(c[i][j][k], weight_v)
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
