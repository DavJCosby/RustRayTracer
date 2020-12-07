use std::path::Path;

mod output;
mod math;
use output::{ImageGenerator, ppm::PPMGenerator};
use math::vectors::*;

fn main() {


    let mut v1 = Vec3::new(1.0, 2.0, 3.0);
    let mut v2 = Vec3::new(0.0, 5.0, -6.0) + Vec3::new(2.0, 3.0, 4.0);
    v1 *= 2.0;
    v2 -= v1;
    let v3 = (16.0 * v2.unit()) - Vec3::new(0.0, 1.0, 0.0);


    let img_width = 256;
    let img_height = 256;
    let file_path = Path::new("renders/r1.ppm");

    let mut generator = PPMGenerator::new(file_path, img_width, img_height);

    for y in 0..img_height {
        println!("{}/{}", y+1, img_height);
        for x in 0..img_width {
            let r = (x as f64) / (img_width - 1) as f64;
            let g = (y as f64) / (img_height - 1) as f64;
            let b = 0.25;
            generator.set_pixel((x, y), (r, g, b));
        }
    }

    generator.write();
}
