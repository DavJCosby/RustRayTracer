const RENDER_SETTINGS: RenderSettings = RenderSettings {
    img_size: (600, 400),
    samples_per_pixel: 100,
    max_depth: 16,
    threads: 12,
};

use image::{hdr::HdrDecoder, Rgb};
use rand::{prelude::ThreadRng, Rng};
use std::{fs::File, io::BufReader};

use crate::{
    render::{
        camera::Camera,
        materials::{environment::*, *},
        scene::Scene,
        shapes::{hit::Hittable, sphere::Sphere},
    },
    scenes::RenderSettings,
    utils::{ray::Ray, vector::*},
};

pub fn generate() -> Scene {
    // Environment setup
    let environment: Box<dyn Environment> = Box::new(HDRIEnvironment {
        texture: fetch_hdr("tex/sky4.hdr"),
        size: (4096, 2048),
        brightness: 1.0,
    });
    // Box::new(environment::DefaultSkyEnvironment {});

    // Components setup
    let mut components: Vec<Box<dyn Hittable>> = Vec::new();

    let ground = Sphere {
        center: Point3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: Box::new(material::Lambertian {
            albedo: Color::new(0.5, 0.5, 0.5),
        }),
    };
    components.push(Box::new(ground));

    let mut rng = rand::thread_rng();
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
                    let sphere = Sphere {
                        center,
                        radius,
                        material: Box::new(material::Lambertian {
                            albedo: random_color(rng) * random_color(rng),
                        }),
                    };
                    components.push(Box::new(sphere));
                }
                s if s < 0.8 && s >= 0.95 => {
                    // metal
                    let sphere = Sphere {
                        center,
                        radius,
                        material: Box::new(material::Metal {
                            albedo: Color::new(0.5, 0.5, 0.5) + random_color(rng) / 2.0,
                            fuzz: rng.gen_range(0.0, 0.5),
                        }),
                    };
                    components.push(Box::new(sphere));
                }
                _ => {
                    // glass
                    let sphere = Sphere {
                        center,
                        radius,
                        material: Box::new(material::Dielectric {
                            albedo: Color::new(1.0, 1.0, 1.0),
                            ior: 1.5,
                        }),
                    };
                    components.push(Box::new(sphere));
                }
            }
        }
    }

    let sphere1 = Sphere {
        center: Point3::new(0.0, 1.0, 0.0),
        radius: 1.0,
        material: Box::new(material::Dielectric {
            albedo: Color::new(1.0, 1.0, 1.0),
            ior: 1.5,
        }),
    };

    let sphere2 = Sphere {
        center: Point3::new(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: Box::new(material::Lambertian {
            albedo: Color::new(0.4, 0.2, 0.1),
        }),
    };

    let sphere3 = Sphere {
        center: Point3::new(4.0, 1.0, 0.0),
        radius: 1.0,
        material: Box::new(material::Metal {
            albedo: Color::new(0.7, 0.6, 0.5),
            fuzz: 0.0,
        }),
    };

    components.push(Box::new(sphere1));
    components.push(Box::new(sphere2));
    components.push(Box::new(sphere3));

    // Camera setup
    let origin = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vfov = 20.0;
    let aperture = 0.1;
    let focal_point = Ray::new(origin, lookat - origin).at(10.0);
    let aspect_ratio = RENDER_SETTINGS.img_size.0 as f64 / RENDER_SETTINGS.img_size.1 as f64;

    let camera = Camera::new(origin, lookat, vfov, aspect_ratio, aperture, focal_point);

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

fn fetch_hdr(file_path: &str) -> Vec<Rgb<f32>> {
    let f = File::open(file_path).unwrap();
    let reader = BufReader::new(f);
    let d = HdrDecoder::new(reader).unwrap();
    return d.read_image_hdr().unwrap();
}

fn random_color(mut rng: ThreadRng) -> Color {
    Color::new(rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>())
}
