use std::path::Path;

use ga::Array;
use image::{self, GenericImage};

pub fn array_from_images<P: AsRef<Path>>(width: usize, height: usize, paths: &[P]) -> Array<f32> {
    let mut array = Array::new(vec![paths.len(), width, height], 0.0);

    for (i, path) in paths.iter().enumerate() {
        let img = image::open(path).unwrap().to_luma();

        let (w, h) = img.dimensions();
        assert!(w as usize == width);
        assert!(h as usize == height);

        for (x, y, pixel) in img.enumerate_pixels() {
            array[&[i, y as usize, x as usize]] = (pixel.data[0] as f32) / 255.0;
        }
    }

    array
}
