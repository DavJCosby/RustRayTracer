mod output;
mod render;
mod scenes;
mod utils;

use output::{ppm::PPMGenerator, tonemapping, ImageGenerator};
use render::sampler::Sampler;
use std::{path::Path, time::Instant};

extern crate image;

use scenes::scene1 as current_scene;

fn main() {
    let start = Instant::now();

    // scene import
    let scene = current_scene::generate();
    let img_size = scene.render_settings.img_size;

    // setup image output
    let output_path = Path::new("renders/r2.ppm");
    let mut ppm_generator = PPMGenerator::new(output_path, img_size);

    // begin rendering
    let mut sampler = Sampler::new(&scene);
    for y in 0..img_size.1 {
        for x in 0..img_size.0 {
            let sampled_color = sampler.sample(x, y);
            let tonemapped = tonemapping::aces(sampled_color);
            ppm_generator.set_pixel((x, y), tonemapped)
        }
        println!("{}/{} rows complete", y + 1, img_size.1);
    }
    ppm_generator.write();

    // benchmark info
    let elapsed = start.elapsed();
    println!("finished in {} seconds", elapsed.as_secs_f32());
}
