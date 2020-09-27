use std::ops::{Add, Mul, Sub};

pub mod packed;
pub mod scalar;
pub mod slipstream;

pub type Image = Vec<Vec<u8>>;

pub trait Compute {
    fn compute(&self, image: &mut Image, pix_size: f32);
}

#[derive(Copy, Clone, Default)]
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
