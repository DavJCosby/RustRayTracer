use std::{fs::File, path::Path};

mod output;
mod render;
mod scene;
mod utils;

use output::{ppm::PPMGenerator, tonemapping, ImageGenerator};
use render::sampler::Sampler;
use scene::{
    camera::Camera,
    materials::{environment, material},
    shapes::sphere::Sphere,
    Scene,
};
use std::time::Instant;
use utils::vector::*;

extern crate image;
use image::{hdr::HdrDecoder, Rgb};
use std::io::BufReader;

const IMG_WIDTH: u32 = 720;
const IMG_HEIGHT: u32 = 480;
const SAMPLES_PER_PIXEL: u32 = 1000;
const MAX_DEPTH: u32 = 16;

fn main() {
    let output_path = Path::new("renders/r2.ppm");
    let mut ppm_generator = PPMGenerator::new(output_path, IMG_WIDTH, IMG_HEIGHT);

    let camera = Camera::new(IMG_WIDTH as f64, IMG_HEIGHT as f64);
    let scene = scene1();
    let mut sampler = Sampler::new(IMG_WIDTH, IMG_HEIGHT, &scene, &camera);
    sampler.max_depth = MAX_DEPTH;

    // Rendering
    let start = Instant::now();

    for y in 0..IMG_HEIGHT {
        for x in 0..IMG_WIDTH {
            let sampled_color = sampler.take_samples(x, y, SAMPLES_PER_PIXEL);
            let tonemapped = tonemapping::aces(sampled_color);
            ppm_generator.set_pixel((x, y), tonemapped)
        }
        println!("{}/{} rows complete", y + 1, IMG_HEIGHT);
    }
    let elapsed = start.elapsed();
    println!("finished in {} seconds", elapsed.as_secs_f32());

    ppm_generator.write();
}

fn scene1() -> Scene {
    let mut scene = Scene::new();

    scene.environment = /*Box::new(environment::ColorEnvironment {
        color: Color::new(0.4, 0.6, 1.0),
    });
    */
    Box::new(environment::TexturedEnvironment {
        texture: fetch_hdr("tex/sky1.hdr"),
        size: (4096, 2048),
        brightness: 1.0,
    });
    
    let sphere = Sphere {
        center: Point3::new(0.0, -0.05, -1.0),
        radius: 0.5,
        material: Box::new(material::Metal {
            albedo: Color::new(0.7, 0.7, 0.4),
            fuzz: 0.75
        }),
    };

    let ground = Sphere {
        center: Point3::new(0.0, -100.5, -1.0),
        radius: 100.0,
        material: Box::new(material::Metal {
            albedo: Color::new(0.2, 0.75, 0.1),
            fuzz: 0.25
        }),
    };

    scene.components.push(Box::new(sphere));
    //scene.components.push(Box::new(ground));

    return scene;
}

fn fetch_hdr(file_path: &str) -> Vec<Rgb<f32>> {
    let f = File::open(file_path).unwrap();
    let reader = BufReader::new(f);
    let d = HdrDecoder::new(reader).unwrap();
    return d.read_image_hdr().unwrap();
}
