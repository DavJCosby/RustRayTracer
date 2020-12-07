use std::path::Path;

mod output;
use output::ImageGenerator;
use output::ppm::PPMGenerator;

fn main() {
    let img_width = 256;
    let img_height = 256;
    let file_path = Path::new("renders/r1.ppm");

    let mut generator = PPMGenerator::new(file_path, img_width, img_height);

    for y in 0..img_height {
        for x in 0..img_width {
            let r = (x as f64) / (img_width - 1) as f64;
            let g = (y as f64) / (img_height - 1) as f64;
            let b = 0.25;
            generator.set_pixel((x, y), (r, g, b));
        }
    }

    generator.write();
}
