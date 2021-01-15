pub const RENDER_SETTINGS: RenderSettings = RenderSettings {
    img_size: (600, 400),
    samples_per_pixel: 1024,
    max_depth: 16,
};

const NUM_COMPONENTS: usize = 2;

use image::codecs::hdr::HdrDecoder;
use std::{fs::File, io::BufReader, mem};

use crate::{
    render::{
        camera::Camera,
        materials::{environment::*, material::Material, *},
        scene::Scene,
        shapes::Shape,
    },
    scenes::RenderSettings,
    utils::vector::*,
};

pub fn generate<'a>() -> Scene<'a, NUM_COMPONENTS> {
    // Environment
    let environment = //Environment::DefaultSkyEnvironment {};

    Environment::HDRIEnvironment {
        texture: &fetch_hdr("tex/sky4.hdr"),
        size: (4096, 2048),
        brightness: 1.0,
    };

    // Components
    let center = Point3::new(0.0, -0.05, -1.0);

    let mut components: [Shape; NUM_COMPONENTS] = [
        // Glass Ball
        Shape::Sphere {
            center,
            radius: 0.5,
            material: Material::Dielectric {
                albedo: Color::new(0.5, 0.75, 1.0),
                ior: 1.33,
            },
        },
        // Ground
        Shape::Sphere {
            center: Point3::new(0.0, -100.5, -1.0),
            radius: 100.0,
            material: Material::Lambertian {
                albedo: Color::new(0.1, 0.5, 0.05),
            },
        },
    ];

    // Camera
    let origin = Point3::new(0.0, 0.125, 1.0);
    let lookat = center;
    let vfov = 50.0;
    let aperture = 0.025;
    let focal_point = Point3::new(0.0, -0.55, -1.0);
    let aspect_ratio = RENDER_SETTINGS.img_size.0 as f64 / RENDER_SETTINGS.img_size.1 as f64;

    let camera = Camera::new(origin, lookat, vfov, aspect_ratio, aperture, focal_point);

    // Scene
    //components.push(sphere);

    //println!("{}", mem::size_of::<Environment>());
    //components.push(Box::new(ground));
    let scene = Scene {
        components,
        camera,
        environment,
        render_settings: RENDER_SETTINGS,
    };

    return scene;
}

pub fn get_render_settings() -> RenderSettings {
    return RENDER_SETTINGS;
}

fn fetch_hdr(file_path: &str) -> &'static [Color] {
    let f = File::open(file_path).unwrap();
    let reader = BufReader::new(f);
    let d = HdrDecoder::new(reader).unwrap();
    let vec = d.read_image_hdr().unwrap();
    let mut vec2: Vec<Color> = Vec::new();
    for c in vec {
        let b = c.clone();
        vec2.push(Color::new(b.0[0] as f64, b.0[1] as f64, b.0[2] as f64));
    }

    let static_ref: &'static [Color] = vec2.leak();
    return static_ref;
}
