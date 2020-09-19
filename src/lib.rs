pub type Image = Vec<Vec<u8>>;

pub trait Compute {
    fn compute(&self, image: &mut Image, pix_size: f32, x_off: f32, y_off: f32);
}

pub struct Base;

impl Compute for Base {
    fn compute(&self, image: &mut Image, _: f32, _: f32, _: f32) {
        for (y, row) in image.iter_mut().enumerate() {
            for (x, pix) in row.iter_mut().enumerate() {
                *pix = if (x + y) % 2 == 0 {
                    0
                } else {
                    255
                }
            }
        }
    }
}
