use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use super::ImageGenerator;
use crate::utils::vector::*;

pub struct PPMGenerator {
    path: &'static Path,
    width: u32,
    height: u32,
    pixel_array: Vec<(u8, u8, u8)>,
}

impl ImageGenerator for PPMGenerator {
    fn new(file_path: &'static Path, size: (u32, u32)) -> PPMGenerator {
        let array = vec![(0, 0, 0); (size.0 * size.1) as usize];

        return PPMGenerator {
            path: file_path,
            width: size.0,
            height: size.1,
            pixel_array: array,
        };
    }

    fn set_pixel(&mut self, coord_xy: (u32, u32), color: (u8, u8, u8)) {
        let index = coord_xy.0 + ((self.height - 1 - coord_xy.1) * self.width);
        self.pixel_array[index as usize] = color;
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
            out.push_str(&format!("{} {} {}\n", color.0, color.1, color.2));
        }

        return out;
    }
}
