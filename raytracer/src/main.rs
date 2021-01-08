mod output;
mod render;
mod scenes;
mod utils;

use output::{ppm::PPMGenerator, tonemapping, ImageGenerator};
use render::sampler::Sampler;
use std::{
    path::Path,
    sync::mpsc,
    sync::mpsc::{Receiver, Sender},
    thread,
    time::Instant,
};
use utils::vector::Color;

use scenes::cover_scene as scene;
const OUTPUT_LOCATION: &str = "renders/r2.ppm";

fn main() {
    let start = Instant::now();
    let render_settings = scene::get_render_settings();
    let num_threads = render_settings.threads;

    let mut color_array: Vec<Vec<Color>> =
        vec![
            vec![Color::new(0.0, 0.0, 0.0); render_settings.img_size.1 as usize];
            render_settings.img_size.0 as usize
        ];

    let output_path = Path::new(OUTPUT_LOCATION);
    let mut ppm_generator = PPMGenerator::new(output_path, render_settings.img_size);

    let (base_producer, consumer): (Sender<(u32, u32, Color)>, Receiver<(u32, u32, Color)>) =
        mpsc::channel();
    let mut thread_container = Vec::new();

    for thread_id in 0..num_threads {
        let producer = base_producer.clone();
        let thread = thread::spawn(move || {
            // scene import

            let scene = scene::generate();
            let img_size = scene.render_settings.img_size;

            // begin rendering

            let mut sampler = Sampler::new(&scene);
            for y in 0..img_size.1 {
                for x in 0..img_size.0 {
                    let sampled_color =
                        sampler.sample((x, y), render_settings.samples_per_pixel / num_threads);
                    //let tonemapped = tonemapping::aces(sampled_color);
                    producer
                        .send((x, y, sampled_color / num_threads as f64))
                        .unwrap();
                    //ppm_generator.set_pixel((x, y), tonemapped)
                }
                //println!("{}/{} rows complete", y + 1, img_size.1);
            }
        });
        thread_container.push(thread);
    }

    // collect messages
    let size = render_settings.img_size.0 * render_settings.img_size.1 * num_threads;
    let mut ids = Vec::with_capacity(size as usize);
    for _ in 0..size {
        ids.push(consumer.recv().unwrap());
    }

    for thread in thread_container {
        thread.join().expect("rip, the child thread panicked");
    }

    for response in ids {
        color_array[response.0 as usize][response.1 as usize] =
            color_array[response.0 as usize][response.1 as usize] + response.2;
    }

    for x in 0..render_settings.img_size.0 {
        for y in 0..render_settings.img_size.1 {
            let sampled = color_array[x as usize][y as usize];
            let tonemapped = tonemapping::aces(sampled);
            ppm_generator.set_pixel((x as u32, y as u32), tonemapped)
        }
    }

    ppm_generator.write();

    // benchmark info
    let elapsed = start.elapsed();
    println!("finished in {} seconds", elapsed.as_secs_f32());
}
