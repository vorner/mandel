use rayon::prelude::*;

use super::{Complex, Compute, Image, LIMIT};

#[inline]
fn scalar_row(row: &mut [u8], i: f32, x_off: f32, pix_size: f32) {
    for (x, pix) in row.iter_mut().enumerate() {
        *pix = 255;
        let c = Complex {
            r: x_off + (x as f32) * pix_size,
            i,
        };
        let mut z = Complex { r: 0.0, i: 0.0 };
        for i in 0..255 {
            z = z * z + c;
            if z.len_sq() >= LIMIT {
                *pix = i;
                break;
            }
        }
    }
}

pub struct Base;

impl Compute for Base {
    fn compute(&self, image: &mut Image, pix_size: f32) {
        let h = image.len();
        let w = image[0].len();
        let x_off = - (w as f32) / 2.0 * pix_size;
        let y_off = - (h as f32) / 2.0 * pix_size;
        for (y, row) in image.iter_mut().enumerate() {
            scalar_row(row, y_off + (y as f32) * pix_size, x_off, pix_size);
        }
    }
}

pub struct Parallel;

impl Compute for Parallel {
    fn compute(&self, image: &mut Image, pix_size: f32) {
        let h = image.len();
        let w = image[0].len();
        let x_off = - (w as f32) / 2.0 * pix_size;
        let y_off = - (h as f32) / 2.0 * pix_size;
        image
            .par_iter_mut()
            .enumerate()
            .for_each(|(y, row)| {
                scalar_row(row, y_off + (y as f32) * pix_size, x_off, pix_size);
            });
    }
}
