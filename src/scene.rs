use std::{collections::HashMap, convert, sync::Arc};

use image::{ImageBuffer, Rgb};
use nalgebra as na;
use nalgebra::{Point3, Vector3};
use rand::rngs::ThreadRng;
use rayon::prelude::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};

use crate::camera;
use crate::{
    camera::Camera,
    hitrecord::Hitable,
    kdnode::KdNode,
    light::Light,
    util::{random_f32, ray_color_dup},
};

pub struct Scene {
    scene: Vec<Arc<dyn Hitable + 'static>>,
    lit: Light,
    tree: Option<KdNode>,
    //centre to corresponding light
}

impl Scene {
    pub fn new(li: Light) -> Self {
        Scene {
            scene: Vec::new(),
            lit: li,
            tree: None,
        }
    }
    pub fn add_object(&mut self, object: impl Hitable + 'static) {
        let center = object.centre();
        let obj = Arc::new(object);
        self.scene.push(obj);
    }

    fn get_tree(&mut self) -> KdNode {
        return KdNode::new(&mut self.scene, 20);
    }
    pub fn set_tree(&mut self) {
        self.tree = Some(self.get_tree())
    }

    pub fn render_scene(&mut self, path: String, camera: Camera) {
        // Image
        let aspect_ratio = 16.0 / 9.0;
        let image_width = 400;
        let image_height = (image_width as f64 / aspect_ratio) as u32;
        let samples_per_pixel = 200;
        let max_depth: u32 = 100;

        // World
        let world = self.tree.as_ref().unwrap();

        let scene_bounding_box = world.bounding_box.unwrap();
        let diagonal_length = (scene_bounding_box.max - scene_bounding_box.min).norm();
        let t_max = diagonal_length;
        //dbg!(t_max);

        // Camera

        //let bs = Arc::new(Mutex::new(HashMap::<i32, i32>::new()));
        let mut img: ImageBuffer<Rgb<u8>, Vec<_>> = ImageBuffer::new(image_width, image_height);

        let world_arc = Arc::new(world);
        let lit = self.lit.clone();
        let data: Vec<(u32, u32, Rgb<u8>)> = (0..image_height)
            .into_par_iter()
            .rev()
            .flat_map(move |j| {
                let world = Arc::clone(&world_arc);
                (0..image_width).into_par_iter().map(move |i| {
                    let mut pixel_color = Vector3::new(0.0, 0.0, 0.0);

                    for _ in 0..samples_per_pixel {
                        let u = (i as f32 + random_f32()) / (image_width - 1) as f32;
                        let v = (j as f32 + random_f32()) / (image_height - 1) as f32;
                        let ray = camera.get_ray(u, v);
                        //println!("here camera");
                        pixel_color += ray_color_dup(&ray, &world, max_depth, t_max, &lit);
                    }

                    let scale = 1.0 / samples_per_pixel as f32;
                    let r = (pixel_color.x * scale).sqrt();
                    let g = (pixel_color.y * scale).sqrt();
                    let b = (pixel_color.z * scale).sqrt();

                    let ir = (255.99 * r.clamp(0.0, 0.999)) as u8;
                    let ig = (255.99 * g.clamp(0.0, 0.999)) as u8;
                    let ib = (255.99 * b.clamp(0.0, 0.999)) as u8;

                    (i, image_height - 1 - j, Rgb([ir, ig, ib]))
                })
            })
            .collect();

        for (i, j, pixel) in data {
            img.put_pixel(i, j, pixel);
        }
        img.save(path).unwrap();
    }
}
