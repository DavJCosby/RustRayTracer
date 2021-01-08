use std::{fs::File, io::BufReader};

use image::{hdr::HdrDecoder, Rgb};

use crate::{
    render::{
        camera::Camera,
        materials::{environment::*, *},
        shapes::{hit::Hittable, sphere::Sphere},
        scene::Scene,
    },
    utils::vector::*,
};

use super::RenderSettings;

const RENDER_SETTINGS: RenderSettings = RenderSettings {
    img_size: (600, 400),
    samples_per_pixel: 400,
    max_depth: 16,
    threads: 12,
};

pub fn generate() -> Scene {
    // Environment
    let environment: Box<dyn Environment> = Box::new(environment::HDRIEnvironment {
        texture: fetch_hdr("tex/sky4.hdr"),
        size: (4096, 2048),
        brightness: 1.0,
    });

    // Components
    let mut components: Vec<Box<dyn Hittable>> = Vec::new();

    let sphere = Sphere {
        center: Point3::new(0.0, -0.05, -1.0),
        radius: 0.5,
        material: Box::new(material::Dielectric {
            albedo: Color::new(0.9, 0.9, 0.9),
            ior: 1.33,
        }),
    };

    let ground = Sphere {
        center: Point3::new(0.0, -100.5, -1.0),
        radius: 100.0,
        material: Box::new(material::Lambertian {
            albedo: Color::new(0.1, 0.5, 0.05),
        }),
    };

    // Camera
    let origin = Point3::new(0.0, 0.125, 1.0);
    let lookat = sphere.center;
    let vfov = 50.0;
    let aperture = 0.05;
    let focal_point = Point3::new(0.0, -0.55, -1.0);
    let aspect_ratio = RENDER_SETTINGS.img_size.0 as f64 / RENDER_SETTINGS.img_size.1 as f64;

    let camera = Camera::new(origin, lookat, vfov, aspect_ratio, aperture, focal_point);

    // Scene
    components.push(Box::new(sphere));
    components.push(Box::new(ground));
    let scene = Scene {
        components,
        camera,
        environment,
        render_settings: RENDER_SETTINGS
    };

    return scene;
}

fn fetch_hdr(file_path: &str) -> Vec<Rgb<f32>> {
    let f = File::open(file_path).unwrap();
    let reader = BufReader::new(f);
    let d = HdrDecoder::new(reader).unwrap();
    return d.read_image_hdr().unwrap();
}
