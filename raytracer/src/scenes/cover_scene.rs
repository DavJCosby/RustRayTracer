const RENDER_SETTINGS: RenderSettings = RenderSettings {
    img_size: (1280, 720),
    samples_per_pixel: 256,
    max_depth: 16,
    threads: 12,
};

const NUM_COMPONENTS: usize = 488;
const SEED: u64 = 420;

use image::{hdr::HdrDecoder};
use rand::{rngs::StdRng, Rng, SeedableRng};
use std::{convert::TryInto, fs::File, io::BufReader};

use crate::{
    render::{
        camera::Camera,
        materials::{environment::*, material::Material, *},
        scene::Scene,
        shapes::Shape,
    },
    scenes::RenderSettings,
    utils::{ray::Ray, vector::*},
};

pub fn generate() -> Scene<'static, NUM_COMPONENTS> {
    // Environment setup
    let environment = //Environment::DefaultSkyEnvironment {};
    Environment::HDRIEnvironment {
        texture: &fetch_hdr("tex/sky4.hdr"),
        size: (4096, 2048),
        brightness: 1.0,
    };

    // Components setup
    let mut components: Vec<Shape> = Vec::new();

    let ground = Shape::Sphere {
        center: Point3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: Material::Lambertian {
            albedo: Color::new(0.5, 0.5, 0.5),
        },
    };
    components.push(ground);

    let mut rng = StdRng::seed_from_u64(SEED);
    for a in -11..11 {
        for b in -11..11 {
            let center = Point3::new(
                a as f64 + 0.9 * rng.gen::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.gen::<f64>(),
            );
            let radius = 0.2;

            let seed = rng.gen::<f32>();
            match seed {
                s if s < 0.8 => {
                    // lambertian diffuse
                    let albedo = Color::new(rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>())
                        * Color::new(rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>());

                    let sphere = Shape::Sphere {
                        center,
                        radius,
                        material: Material::Lambertian { albedo },
                    };
                    components.push(sphere);
                }
                s if s < 0.8 && s >= 0.95 => {
                    // metal
                    let albedo = Color::new(0.5, 0.5, 0.5)
                        + Color::new(rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>()) / 2.0;
                    let fuzz = rng.gen_range(0.0, 0.5);

                    let sphere = Shape::Sphere {
                        center,
                        radius,
                        material: Material::Metal { albedo, fuzz },
                    };
                    components.push(sphere);
                }
                _ => {
                    // glass
                    let albedo = Color::new(1.0, 1.0, 1.0);
                    let sphere = Shape::Sphere {
                        center,
                        radius,
                        material: Material::Dielectric { albedo, ior: 1.5 },
                    };
                    components.push(sphere);
                }
            }
        }
    }

    let sphere1 = Shape::Sphere {
        center: Point3::new(0.0, 1.0, 0.0),
        radius: 1.0,
        material: Material::Dielectric {
            albedo: Color::new(1.0, 1.0, 1.0),
            ior: 1.5,
        },
    };

    let sphere2 = Shape::Sphere {
        center: Point3::new(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: Material::Lambertian {
            albedo: Color::new(0.4, 0.2, 0.1),
        },
    };

    let sphere3 = Shape::Sphere {
        center: Point3::new(4.0, 1.0, 0.0),
        radius: 1.0,
        material: Material::Metal {
            albedo: Color::new(0.7, 0.6, 0.5),
            fuzz: 0.0,
        },
    };

    components.push(sphere1);
    components.push(sphere2);
    components.push(sphere3);

    let boxed_slice = components.into_boxed_slice();
    let boxed_array: Box<[Shape; NUM_COMPONENTS]> = match boxed_slice.try_into() {
        Ok(ba) => ba,
        Err(o) => panic!("Expected a Vec of length {} but it was {}", NUM_COMPONENTS, o.len()),
    }; 

    let components_array = *boxed_array;

    // Camera setup
    let origin = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vfov = 20.0;
    let aperture = 0.1;
    let focal_point = Ray::new(origin, lookat - origin).at(10.0);
    let aspect_ratio = RENDER_SETTINGS.img_size.0 as f64 / RENDER_SETTINGS.img_size.1 as f64;

    let camera = Camera::new(origin, lookat, vfov, aspect_ratio, aperture, focal_point);

    let scene = Scene {
        components: components_array,
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
