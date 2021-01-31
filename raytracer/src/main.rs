mod output;
mod render;
mod scenes;
mod utils;

// INPUT AND OUTPUT SETTINGS
use scenes::scene1 as scene;
const OUTPUT_LOCATION: &str = "renders/r2.ppm";

// dependencies

use output::{ppm::PPMGenerator, tonemapping, ImageGenerator};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use render::sampler::Sampler;
use std::{path::Path, sync::mpsc::channel, time::Instant};

fn main() {
    let start = Instant::now();
    render();
    let elapsed = start.elapsed();
    println!("finished in {} seconds", elapsed.as_secs_f32());
}

fn render() {
    let render_settings = scene::get_render_settings();
    let img_size = render_settings.img_size;

    let output_path = Path::new(OUTPUT_LOCATION);
    let mut ppm_generator = PPMGenerator::new(output_path, img_size);

    let scene = scene::generate();
    let sampler = &Sampler::new(scene);

    let (s, r) = channel();

    (0..img_size.1).into_par_iter().for_each_with(s, |s, y| {
        for x in 0..img_size.0 {
            let sampled_color = sampler.sample((x, y), render_settings.samples_per_pixel);
            let tonemapped = tonemapping::aces(sampled_color);
            let compressed = (
                (tonemapped.x * 255.0) as u8,
                (tonemapped.y * 255.0) as u8,
                (tonemapped.z * 255.0) as u8,
            );
            s.send(((x, y), compressed)).unwrap();
        }
    });

    // collect messages

    let responses: Vec<_> = r.iter().collect();
    for (pos, col) in responses {
        ppm_generator.set_pixel(pos, col);
    }

    ppm_generator.write();
}
