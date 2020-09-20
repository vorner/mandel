use std::ops::{Add, Mul, Sub};

pub type Image = Vec<Vec<u8>>;

pub trait Compute {
    fn compute(&self, image: &mut Image, pix_size: f32);
}

#[derive(Copy, Clone)]
struct Complex<T> {
    r: T,
    i: T,
}

impl<T> Complex<T>
where
    T: Copy + Mul<Output = T> + Add<Output = T>,
{
    #[inline]
    fn len_sq(self) -> T {
        self.r * self.r + self.i * self.i
    }
}

impl<T: Add<Output = T>> Add for Complex<T> {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self {
            r: self.r + rhs.r,
            i: self.i + rhs.i,
        }
    }
}

impl<T> Mul for Complex<T>
where
    T: Add<Output = T>,
    T: Sub<Output = T>,
    T: Mul<Output = T>,
    T: Copy,
{
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self {
        Self {
            r: self.r * rhs.r - self.i * rhs.i,
            i: self.r * rhs.i + self.i * rhs.r,
        }
    }
}

const LIMIT: f32 = 16.0;

pub struct Base;

impl Compute for Base {
    fn compute(&self, image: &mut Image, pix_size: f32) {
        let h = image.len();
        let w = image[0].len();
        let x_off = - (w as f32) / 2.0 * pix_size;
        let y_off = - (h as f32) / 2.0 * pix_size;
        for (y, row) in image.iter_mut().enumerate() {
            for (x, pix) in row.iter_mut().enumerate() {
                *pix = 255;
                let c = Complex {
                    r: x_off + (x as f32) * pix_size,
                    i: y_off + (y as f32) * pix_size,
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
    }
}
