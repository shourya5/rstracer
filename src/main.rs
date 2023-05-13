pub mod aabb;
pub mod bvhnode;
pub mod camera;
pub mod cone;
pub mod cube;
pub mod cylinder;
pub mod dielectric;
pub mod hitrecord;
pub mod kdnode;
pub mod kdtree;
pub mod lambertian;
pub mod light;
pub mod material;
pub mod metal;
pub mod ray;
pub mod sphere;
pub mod util;

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use camera::Camera;
use cube::Cube;

use hitrecord::{HitRecord, Hitable};
use image::{ImageBuffer, Rgb};
// use kdnode::KdNode;

use kdnode::KdNode;

use material::Material;
use nalgebra::{Point3, Vector3};
// use rand::Rng;
use sphere::Sphere;


use std::sync::{Arc};
use util::{random_f32, ray_color_dup};

// use std::io::{prelude::*, self};
use crate::ray::Ray;
use rand::{Rng};
use rayon::iter::{IntoParallelIterator, ParallelIterator};




use rayon::prelude::*;

//     fn main() {
//         // Image
//         let aspect_ratio = 16.0 / 9.0;
//         let image_width = 400;
//         let image_height = (image_width as f32 / aspect_ratio) as u32;
//         let samples_per_pixel = 150;
//         let max_depth = 60;

//         // World
//         let world = random_scene();

//         // Camera
//         let lookfrom = Point3::new(10.0, 4.0, 9.0);
//         let lookat = Point3::new(0.0, 0.0, 0.0);
//         let vup = Vector3::new(0.0, 1.0, 0.0);
//         let dist_to_focus = 10.0;
//         let aperture = 0.09;
//         let camera = Camera::new(
//             lookfrom,
//             lookat,
//             vup,
//             20.0,
//             aspect_ratio,
//             aperture,
//             dist_to_focus,
//         );

//         //let mut image_data: Vec<u8> = Vec::<u8>::new();
//         let mut img = ImageBuffer::new(image_width, image_height);
//         //let mut row_data = Vec::<u8>::new();
//         for j in (0..image_height).rev() {
//             for i in 0..image_width {
//                 let mut pixel_color = Vector3::new(0.0, 0.0, 0.0);
//                 for _ in 0..samples_per_pixel {
//                     let u = (i as f32 + random_f32()) / (image_width - 1) as f32;
//                     let v = (j as f32 + random_f32()) / (image_height - 1) as f32;
//                     let ray = camera.get_ray(u, v);
//                     pixel_color += ray_color(&ray, &world, max_depth);
//                 }

//             // Apply gamma correction and write the pixel color
//             let scale = 1.0 / samples_per_pixel as f32;
//             let r = (pixel_color.x * scale).sqrt();
//             let g = (pixel_color.y * scale).sqrt();
//             let b = (pixel_color.z * scale).sqrt();

//             let ir = (255.99 * r.clamp(0.0, 0.999)) as u8;
//             let ig = (255.99 * g.clamp(0.0, 0.999)) as u8;
//             let ib = (255.99 * b.clamp(0.0, 0.999)) as u8;

//             // image_data.push(ir);
//             // image_data.push(ig);
//             // image_data.push(ib);
//             img.put_pixel(i, image_height - 1 - j, Rgb([ir, ig, ib]));
//         }
//         // let mut file = File::create("outputx.ppm").unwrap();
//         // file.write_all(format!("P6\n{} {}\n255\n", image_width, image_height).as_bytes())
//         //     .unwrap();
//         // file.write_all(&image_data).unwrap();
//         // let mut file = File::create("outputx.png").unwrap();

//     }
//     img.save("outputx.png").unwrap();
// }
fn render_scene(path : String) {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as u32;
    let samples_per_pixel = 80;
    let max_depth: u32 = 45;

    // World
    let world = random_scene();
    let scene_bounding_box = world.bounding_box.unwrap();
    let diagonal_length = (scene_bounding_box.max - scene_bounding_box.min).norm();
    let t_max = diagonal_length;
    //dbg!(t_max);

    // Camera
    let lookfrom = Point3::new(12.0, 8.00, 12.0);

    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vector3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.09;
    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        aspect_ratio as f32,
        aperture,
        dist_to_focus,
    );

    //let bs = Arc::new(Mutex::new(HashMap::<i32, i32>::new()));
    let mut img: ImageBuffer<Rgb<u8>, Vec<_>> = ImageBuffer::new(image_width, image_height);
    
    let world_arc = Arc::new(world);

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
                    pixel_color += ray_color_dup(&ray, &world, max_depth, t_max);
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
fn main() {
    render_scene("sample.png".to_owned());
}
// Implement the Material trait for diffuse materials (Lambertian)

// Helper function to generate random points in a unit sphere

// Metal struct representing a metal material

// Dielectric struct representing a dielectric material

// Helper function to refract a vector

fn random_vector3(min: f32, max: f32) -> Vector3<f32> {
    let mut rng = rand::thread_rng();
    Vector3::new(
        rng.gen_range(min..max),
        rng.gen_range(min..max),
        rng.gen_range(min..max),
    )
}
// fn random_scene() -> Vec<Box<dyn Hitable>> {
//     let mut rng = rand::thread_rng();
//     let mut world: Vec<Box<dyn Hitable>> = Vec::new();

//     // Ground
//     world.push(Box::new(Sphere::new(
//         Point3::new(0.0, -1000.0, 0.0),
//         1000.0,
//         Arc::new(Lambertian::new(Vector3::new(0.5, 0.5, 0.5))),
//     )));

//     //Random small spheres
//     for a in -11..11 {
//         for b in -11..11 {
//             let choose_mat: f32 = rng.gen_range(0.0..1.0);
//             let center = Point3::new(
//                 a as f32 + 0.9 * rng.gen_range(0.0..1.0),
//                 0.2,
//                 b as f32 + 0.9 * rng.gen_range(0.0..1.0),
//             );

//             if (center - Point3::new(4.0, 0.2, 0.0)).magnitude() > 0.9 {
//                 if choose_mat < 0.8 {
//                     // Lambertian material
//                     let albedo = random_vector3(0.0, 1.0).component_mul(&random_vector3(0.0, 1.0));
//                     world.push(Box::new(Sphere::new(
//                         center,
//                         0.2,
//                         Arc::new(Lambertian::new(albedo)),
//                     )));
//                 } else if choose_mat < 0.95 {
//                     // Metal material
//                     let albedo = Vector3::new(
//                         rng.gen_range(0.5..1.0),
//                         rng.gen_range(0.5..1.0),
//                         rng.gen_range(0.5..1.0),
//                     );
//                     let fuzz = rng.gen_range(0.0..0.5);
//                     world.push(Box::new(Sphere::new(
//                         center,
//                         0.2,
//                         Arc::new(Metal::new(albedo, fuzz)),
//                     )));
//                 } else {
//                     // Dielectric material
//                     world.push(Box::new(Sphere::new(
//                         center,
//                         0.2,
//                         Arc::new(Dielectric::new(1.5)),
//                     )));
//                 }
//             }
//         }
//     }

//     // Large spheres
//     world.push(Box::new(Sphere::new(
//         Point3::new(0.0, 1.0, 0.0),
//         1.0,
//         Arc::new(Dielectric::new(1.5)),
//     )));
//     world.push(Box::new(Sphere::new(
//         Point3::new(-4.0, 1.0, 0.0),
//         1.0,
//         Arc::new(Lambertian::new(Vector3::new(0.4, 0.2, 0.1))),
//         //Arc::new(Metal::new(Vector3::new(0.7, 0.6, 0.5), 0.0)),
//     )));
//     world.push(Box::new(Sphere::new(
//         Point3::new(4.0, 1.0, 0.0),
//         1.0,
//         Arc::new(Metal::new(Vector3::new(0.7, 0.6, 0.5), 0.0)),
//     )));
//     BVHNode::from_objects(&mut world, 0, world.len());
//     world
// }
fn random_scene() -> KdNode {
    let mut rng = rand::thread_rng();
    let mut world: Vec<Arc<dyn Hitable>> = Vec::new();

    // Ground
    world.push(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        //Arc::new(Lambertian::new(Vector3::new(0.5, 0.5, 0.5))),
        Material::Lambertian {
            albedo: (Vector3::new(0.5, 0.5, 0.5)),
        },
    )));

    //Random small spheres
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f32 = rng.gen_range(0.0..1.0);
            let center = Point3::new(
                a as f32 + 0.9 * rng.gen_range(0.0..1.0),
                0.2,
                b as f32 + 0.9 * rng.gen_range(0.0..1.0),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).magnitude() > 0.9 {
                if choose_mat < 0.8 {
                    // Lambertian material
                    let albedo = random_vector3(0.0, 1.0).component_mul(&random_vector3(0.0, 1.0));
                    world.push(Arc::new(Sphere::new(
                        center,
                        0.2,
                        Material::Lambertian { albedo: (albedo) },
                    )));
                } else if choose_mat < 0.95 {
                    // Metal material
                    let albedo = random_vector3(0.5, 1.0);
                    let fuzz = rng.gen_range(0.0..0.5);
                    world.push(Arc::new(Sphere::new(
                        center,
                        0.2,
                        Material::Metal {
                            albedo: (albedo),
                            fuzz: (fuzz),
                        },
                    )));
                } else {
                    // Dielectric material
                    world.push(Arc::new(Sphere::new(
                        center,
                        0.2,
                        Material::Dielectric { ref_idx: (1.5) },
                    )));
                }
            }
        }
    }

    // Large spheres
    // world.push(Arc::new(Sphere::new(
    //     Point3::new(0.0, 1.0, 0.0),
    //     1.0,
    //     Arc::new(Dielectric::new(1.5)),
    // )));
    let mini = Point3::new(0.0, 0.0, 0.0); // Lower left back corner
    let maxi = Point3::new(2.0, 2.0, 4.0); // Upper right front corner
    world.push(Arc::new(Cube::new(
        mini,
        maxi,
        Material::Dielectric { ref_idx: (1.5) },
    )));
    world.push(Arc::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        //Arc::new(Lambertian::new(Vector3::new(0.1, 0.2, 0.1))),
        Material::Lambertian {
            albedo: (Vector3::new(0.1, 0.2, 0.1)),
        },
    )));
    world.push(Arc::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        //Arc::new(Metal::new(Vector3::new(0.7, 0.6, 0.5), 0.0)),
        Material::Metal {
            albedo: (Vector3::new(0.7, 0.6, 0.5)),
            fuzz: (0.0),
        },
    )));

    // let time0: f32 = 0.0;
    // let time1: f32 = 1.0;

    //  let kdtree =  KdTree::new(&mut  world, time0, time1);
    // let t0 = 0.0;
    //     let t1 = 1.0;
    //     let axis = 0;
    //     let kdtree = KdTree::build(&mut world, t0, t1, axis);
    //print!("{}", world.len());
    //dbg!(world.len());
    let kdtree = KdNode::new(&mut world, 7);
    return kdtree;
}
