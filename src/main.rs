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
pub mod scene;
pub mod sphere;
pub mod texture;
pub mod transform;
pub mod util;

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use camera::Camera;
use cube::Cube;

use hitrecord::{HitRecord, Hitable};
use image::{ImageBuffer, Rgb};
// use kdnode::KdNode;

use kdnode::KdNode;

use light::Light;
use material::Material;
use nalgebra::{Point3, Vector3};
use scene::Scene;
// use rand::Rng;
use sphere::Sphere;
use texture::{CheckerTexture, ImageTexture, SolidColor, Wood, Fire};

use std::fs::create_dir_all;
use std::path::Path;
use std::sync::Arc;
use util::{random_f32, ray_color_dup};

// use std::io::{prelude::*, self};
use crate::ray::Ray;
use rand::Rng;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use rayon::prelude::*;


/// It may be more prudent to turn all materials into shader constructs.Will simply and Modularize code by a lot
fn main() {
    let red = Vector3::new(1.0, 0.0, 0.0);
    let solid_red_texture = SolidColor::new(red);
    let solid_red_texture = Arc::new(solid_red_texture); // Wrap in Arc for shared ownership

    // Create another solid color texture
    let blue = Vector3::new(1.0, 1.0, 1.0);
    let solid_blue_texture = SolidColor::new(blue);
    let solid_blue_texture = Arc::new(solid_blue_texture); // Wrap in Arc for shared ownership
    //let perlin_texture = Arc::new(Perlin::new());
    let image_texture = Arc::new(ImageTexture::new(Path::new("bricks.jpg")));
    let lookfrom = Point3::new(-12.0, 8.00, -12.0);
    let color1 = Vector3::new(0.48, 0.83, 0.53); // some suitable color for wood
    let color2 = Vector3::new(0.36, 0.25, 0.20); // another color for wood
    let scale = 5.0; // frequency of the wood rings
    let base_color = Vector3::new(1.0, 0.0, 0.0); // Red color
    let secondary_color = Vector3::new(1.0, 1.0, 0.0); // Yellow color
    let frequency = 10.0;
    let amplitude = 2.0;

    let fire_texture = Arc::new(Fire::new(base_color, secondary_color, frequency, amplitude));
    let wood_texture = Arc::new(Wood::new(color1, color2, scale));

    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vector3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.09;
    let aspect_ratio = 16.0 / 9.0;
    let mut camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        aspect_ratio as f32,
        aperture,
        dist_to_focus,
    );
    // Create a checker texture
    let checker_texture =
        Arc::new(CheckerTexture::new(solid_red_texture.clone(), solid_blue_texture.clone()));

    let mut screen = Scene::new(Light {
        source: (Point3::new(0.0, 0.0, 1.0)),
        radius: (1.0),
    });
    screen.add_object(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        //Arc::new(Lambertian::new(Vector3::new(0.5, 0.5, 0.5))),
        Material::Lambertian {
            albedo: wood_texture,
        },
    ));
    let mini = Point3::new(-1.0, -1.0, -1.0); // Lower left back corner
    let maxi = Point3::new(2.0, 2.0, 4.0); // Upper right front corner
    screen.add_object(Cube::new(
        mini,
        maxi,
        Material::Dielectric { ref_idx: (1.4) },
    ));
    screen.add_object(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        //Arc::new(Metal::new(Vector3::new(0.7, 0.6, 0.5), 0.0)),
        Material::Metal {
            albedo: (Vector3::new(0.7, 0.6, 0.5)),
            fuzz: (0.0),
        },
    ));

    screen.add_object(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        //Arc::new(Lambertian::new(Vector3::new(0.1, 0.2, 0.1))),
        Material::Lambertian {
            albedo: (fire_texture),
        },
    ));
    screen.set_tree();
    screen.render_scene("scene.png".to_owned(), camera);
    let dir = Path::new("image_sequence");
    create_dir_all(&dir).expect("Failed to create directory");
    let num_frames = 300; // The number of frames to render
                          //let camera_movement = Vector3::new(0.1, 0.0, 0.0); // The amount to move the camera each frame

    for frame in 0..num_frames {
        // Move the camera
        camera.move_camera(Vector3::new(0.05, 0.0, 0.05));

        // Render the scene
        let filename = dir.join(format!("frame_{:04}.png", frame));
        screen.render_scene(filename.to_str().unwrap().to_owned(), camera);
    }
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
