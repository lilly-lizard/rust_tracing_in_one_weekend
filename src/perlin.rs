use crate::tools::random_unit_vector;
use array_init::array_init;
use glam::Vec3;
use rand::{thread_rng, Rng};

const POINT_COUNT: usize = 256;

#[derive(Clone)]
pub struct Perlin {
    ranvec: [Vec3; POINT_COUNT],
    perm_x: [i32; POINT_COUNT],
    perm_y: [i32; POINT_COUNT],
    perm_z: [i32; POINT_COUNT],
}
impl Perlin {
    pub fn new() -> Self {
        Self {
            ranvec: array_init(|_| random_unit_vector()),
            perm_x: Perlin::generate_perm(),
            perm_y: Perlin::generate_perm(),
            perm_z: Perlin::generate_perm(),
        }
    }

    pub fn noise(&self, p: Vec3) -> f32 {
        let i = p.x().floor() as i32;
        let j = p.y().floor() as i32;
        let k = p.z().floor() as i32;

        let mut c = [[[Vec3::zero(); 2]; 2]; 2];
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.ranvec[(self.perm_x[((i + di as i32) & 255) as usize]
                        ^ self.perm_x[((j + dj as i32) & 255) as usize]
                        ^ self.perm_x[((k + dk as i32) & 255) as usize]) as usize];
                }
            }
        }

        let u = p.x() - p.x().floor();
        let v = p.y() - p.y().floor();
        let w = p.z() - p.z().floor();
        trilinear_interp(&c, u, v, w)
    }

    // summation of different noise frequencies
    pub fn turbulence(&self, p: &Vec3) -> f32 {
        let depth = 7;
        let mut accum = 0.0_f32;
        let mut temp_p = *p;
        let mut weight = 1.0_f32;

        for _i in 0..depth {
            accum = accum + weight * self.noise(temp_p);
            weight = weight * 0.5;
            temp_p = temp_p * 2.0;
        }
        accum.abs()
    }

    fn generate_perm() -> [i32; POINT_COUNT] {
        let mut perm: [i32; POINT_COUNT] = array_init(|i| i as i32);
        for i in (POINT_COUNT - 1)..0 {
            let target = thread_rng().gen_range(0, i);
            // swap perm[i] with perm[target]
            let tmp = perm[i];
            perm[i] = perm[target];
            perm[target] = tmp;
        }
        perm
    }
}

fn trilinear_interp(c: &[[[Vec3; 2]; 2]; 2], u: f32, v: f32, w: f32) -> f32 {
    // hermite cube
    let uu = u * u * (3.0 - 2.0 * u);
    let vv = v * v * (3.0 - 2.0 * v);
    let ww = w * w * (3.0 - 2.0 * w);
    let mut accum = 0.0_f32;
    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                let weight_v = Vec3::new(u - i as f32, v - j as f32, w - k as f32);
                accum = accum
                    + (i as f32 * uu + (1.0 - i as f32) * (1.0 - uu))
                        * (j as f32 * vv + (1.0 - j as f32) * (1.0 - vv))
                        * (k as f32 * ww + (1.0 - k as f32) * (1.0 - ww))
                        * c[i][j][k].dot(weight_v);
            }
        }
    }
    accum
}
