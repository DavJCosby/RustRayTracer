use std::path::Path;

mod math;
mod objects;
mod output;
mod sampling;
mod scene;

use output::{ppm::PPMGenerator, ImageGenerator};
use sampling::{camera::Camera, sampler::Sampler};
use std::time::Instant;

fn main() {
    // Image

    let img_width = 720;
    let img_height = 480;
    let samples_per_pixel = 100;
    let file_path = Path::new("renders/r1.ppm");

    let mut generator = PPMGenerator::new(file_path, img_width, img_height);

    // Rendering

    let camera = Camera::new(img_width as f64, img_height as f64);
    let scene = scene::scene1();
    let mut sampler = Sampler::new((img_width, img_height), &scene, &camera);

    let start = Instant::now();

    for y in 0..img_height {
        for x in 0..img_width {
            let xy = (x, y);
            generator.set_pixel(xy, sampler.take_samples(xy, samples_per_pixel))
        }
        //println!("{}/{} rows complete", y + 1, img_height);
    }

    let elapsed = start.elapsed();

    println!("finished in {} seconds", elapsed.as_secs_f32());

    generator.write();
}
