use std::io::prelude::*;
use std::path::Path;
use std::fs::File;

use super::ImageGenerator;


pub struct PPMGenerator {
    path: &'static Path,
    width: i32,
    height: i32,
    max_color: f64,
    pixel_array: Vec<(f64, f64, f64)>,
}

impl ImageGenerator for PPMGenerator {

    fn new(file_path: &'static Path, w: i32, h: i32) -> PPMGenerator {
        let array = vec![(0.0, 0.0, 0.0); (w * h) as usize];

        return PPMGenerator {
            path: file_path,
            width: w,
            height: h,
            max_color: 255.0,
            pixel_array: array
        };
    }

    fn set_pixel(&mut self, coord_xy: (i32, i32), color_rgb: (f64, f64, f64)) {
        let index = coord_xy.0 + (coord_xy.1 * self.width);
        self.pixel_array[index as usize] = color_rgb;
    }

    fn write(&self) {
        let display = self.path.display();

        let mut file = match File::create(self.path) {
            Err(why) => panic!("couldn't create {}: {}", display, why),
            Ok(file) => file,
        };

        match file.write_all(self.to_string().as_bytes()) {
            Err(why) => panic!("couldn't write to {}: {}", display, why),
            Ok(_) => println!("successfully wrote to {}", display),
        }
    }
}

impl PPMGenerator {
    fn to_string(&self) -> String {
        let mut out = format!("P3\n{} {}\n255\n", self.width, self.height);

        for color in self.pixel_array.iter() {
            out.push_str(&format!("{} {} {}\n",
                color.0 * self.max_color,
                color.1 * self.max_color,
                color.2 * self.max_color
            ));
        }

        return out;
    }
}
