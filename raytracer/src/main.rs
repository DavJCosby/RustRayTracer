use std::path::Path;

mod math;
mod objects;
mod output;
mod scene;
use math::{ray::Ray, vector::*};
use objects::{hit::Hittable, sphere::Sphere};
use output::{ppm::PPMGenerator, ImageGenerator};
use scene::*;

const CLIP_RANGE: (f64, f64) = (0.0, f64::INFINITY);

fn ray_color(r: &Ray, s: &Scene) -> Color {
    match s.hit(r, CLIP_RANGE) {
        Some(hit_data) => {
            return 0.5 * (hit_data.normal + Vec3::new(1.0, 1.0, 1.0));
        }
        None => {
            let unit_dir = r.direction.unit();
            let t = 0.5 * (unit_dir.y + 1.0);
            return (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0);
        }
    }
}

fn main() {
    // Image

    let img_width = 720;
    let img_height = 480;
    let file_path = Path::new("renders/r1.ppm");

    // Camera

    let viewport_height = 2.0;
    let viewport_width = (img_width as f64 / img_height as f64) * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    // Scene

    let scene = scene::scene1();

    // Render

    let mut generator = PPMGenerator::new(file_path, img_width, img_height);

    for y in 0..img_height {
        //println!("{}/{}", y + 1, img_height);
        for x in 0..img_width {
            let u = x as f64 / (img_width - 1) as f64;
            let v = y as f64 / (img_height - 1) as f64;
            let d = lower_left_corner + u * horizontal + v * vertical - origin;
            let r = Ray::new(origin, d);
            generator.set_pixel((x, y), ray_color(&r, &scene));
        }
    }

    generator.write();
}
