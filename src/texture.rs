use image::{GenericImageView, ImageBuffer, Pixel, RgbaImage};
use nalgebra::{Point3, Vector3};
use rand::Rng;
use std::{path::Path, sync::Arc};

// Define the Texture trait
pub trait Texture: Send + Sync {
    fn value(&self, u: f32, v: f32, p: &Point3<f32>) -> Vector3<f32>;
}

// Solid color texture
#[derive(Clone)]
pub struct SolidColor {
    color_value: Vector3<f32>,
}

impl SolidColor {
    pub fn new(color_value: Vector3<f32>) -> Self {
        Self { color_value }
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f32, _v: f32, _p: &Point3<f32>) -> Vector3<f32> {
        self.color_value
    }
}

// Checker texture
#[derive(Clone)]
pub struct CheckerTexture {
    odd: Arc<dyn Texture>,
    even: Arc<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(t0: Arc<dyn Texture>, t1: Arc<dyn Texture>) -> Self {
        Self { odd: t0, even: t1 }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f32, v: f32, p: &Point3<f32>) -> Vector3<f32> {
        let sines = (10.0 * p[0]).sin() * (10.0 * p[1]).sin() * (10.0 * p[2]).sin();
        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}

// pub struct Perlin {
//     ranvec: Vec<Vector3<f32>>,
//     perm_x: Vec<usize>,
//     perm_y: Vec<usize>,
//     perm_z: Vec<usize>,
// }

// impl Perlin {
//     pub fn new() -> Self {
//         let mut rng = rand::thread_rng();
//         let ranvec = (0..256)
//             .map(|_| Vector3::new(rng.gen(), rng.gen(), rng.gen()))
//             .collect();
//         let perm_x = Self::perlin_generate_perm();
//         let perm_y = Self::perlin_generate_perm();
//         let perm_z = Self::perlin_generate_perm();
//         Self {
//             ranvec,
//             perm_x,
//             perm_y,
//             perm_z,
//         }
//     }

//     fn perlin_generate_perm() -> Vec<usize> {
//         let mut p: Vec<usize> = (0..256).collect();
//         Self::permute(&mut p, 256);
//         p
//     }

//     fn permute(p: &mut [usize], n: usize) {
//         let mut rng = rand::thread_rng();
//         for i in (0..n).rev() {
//             let target = rng.gen_range(0..=i);
//             p.swap(i, target);
//         }
//     }
// }

// impl Texture for Perlin {
//     fn value(&self, u: f32, v: f32, p: &Point3<f32>) -> Vector3<f32> {
//         let scale = 4.0;
//         let noise = self.noise(&(scale * p));
//         Vector3::repeat(0.5) * (1.0 + (scale * p[2] + 10.0 * noise).sin())
//     }
// }

// impl Perlin {
//     fn noise(&self, p: &Point3<f32>) -> f32 {
//         let u = p[0] - p[0].floor();
//         let v = p[1] - p[1].floor();
//         let w = p[2] - p[2].floor();
//         let i = p[0].floor() as usize;
//         let j = p[1].floor() as usize;
//         let k = p[2].floor() as usize;
//         let mut c = [[[Vector3::zeros(); 2]; 2]; 2];
//         for di in 0..=1 {
//             for dj in 0..=1 {
//                 for dk in 0..=1 {
//                     c[di][dj][dk] = self.ranvec[self.perm_x[(i + di) & 255]
//                         ^ self.perm_y[(j + dj) & 255]
//                         ^ self.perm_z[(k + dk) & 255]];
//                 }
//             }
//         }
//         Self::perlin_interp(&c, u, v, w)
//     }

//     fn perlin_interp(c: &[[[Vector3<f32>; 2]; 2]; 2], u: f32, v: f32, w: f32) -> f32 {
//         let uu = u * u * (3.0 - 2.0 * u);
//         let vv = v * v * (3.0 - 2.0 * v);
//         let ww = w * w * (3.0 - 2.0 * w);
//         let mut accum = 0.0;
//         for i in 0..=1 {
//             for j in 0..=1 {
//                 for k in 0..=1 {
//                     let weight = Vector3::new(u - i as f32, v - j as f32, w - k as f32);
//                     accum += (i as f32 * uu + (1 - i) as f32 * (1.0 - uu))
//                         * (j as f32 * vv + (1 - j) as f32 * (1.0 - vv))
//                         * (k as f32 * ww + (1 - k) as f32 * (1.0 - ww))
//                         * c[i][j][k].dot(&weight);
//                 }
//             }
//         }
//         accum
//     }
// }
pub struct ImageTexture {
    data: RgbaImage,
    width: u32,
    height: u32,
}
impl ImageTexture {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let img = image::open(path).expect("Failed to open image").to_rgba8();
        let (width, height) = img.dimensions();
        Self {
            data: img,
            width,
            height,
        }
    }
}
impl Texture for ImageTexture {
    fn value(&self, u: f32, v: f32, _p: &Point3<f32>) -> Vector3<f32> {
        let i = (u * self.width as f32) as u32;
        let j = ((1.0 - v) * self.height as f32) as u32;
        let color_scale = 1.0 / 255.0;
        let pixel = self.data.get_pixel(i, j).channels();
        Vector3::new(
            color_scale * pixel[0] as f32,
            color_scale * pixel[1] as f32,
            color_scale * pixel[2] as f32,
        )
    }
}
use noise::{NoiseFn, Perlin};


#[derive(Clone)]
pub struct Wood {
    color1: Vector3<f32>,
    color2: Vector3<f32>,
    noise: Perlin,
    scale: f32,
}

impl Wood {
    pub fn new(color1: Vector3<f32>, color2: Vector3<f32>, scale: f32) -> Self {
        Self {
            color1,
            color2,
            noise: Perlin::new(23),
            scale,
        }
    }
}

impl Texture for Wood {
    fn value(&self, _u: f32, _v: f32, p: &Point3<f32>) -> Vector3<f32> {
        let noise_value = 0.5 * (1.0 + (self.scale * p.z + 10.0 * self.noise.get([p.x as f64, p.y as f64, p.z as f64]) as f32));
        let s = noise_value.sin();
        if s < 0.5 {
            self.color1
        } else {
            self.color2
        }
    }
    
}
#[derive(Clone)]
pub struct Fire {
    base_color: Vector3<f32>,
    secondary_color: Vector3<f32>,
    noise: Perlin,
    frequency: f32,
    amplitude: f32,
}

impl Fire {
    pub fn new(base_color: Vector3<f32>, secondary_color: Vector3<f32>, frequency: f32, amplitude: f32) -> Self {
        Self {
            base_color,
            secondary_color,
            noise: Perlin::new(56),
            frequency,
            amplitude,
        }
    }
}

impl Texture for Fire {
    fn value(&self, _u: f32, _v: f32, p: &Point3<f32>) -> Vector3<f32> {
        let noise_value = 0.5 * (1.0 + self.noise.get([
            (self.frequency * p.x as f32) as f64, 
            (self.frequency * p.y as f32) as f64, 
            (self.frequency * p.z as f32) as f64]));
        let weight = (self.amplitude * noise_value as f32).sin();
        self.base_color.lerp(&self.secondary_color, weight)
    }
}
