use std::ops::{Add, Mul, Sub};

use multiversion::multiversion;
use rayon::prelude::*;
use slipstream::types::*;
use slipstream::Vector;

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

type V = f32x16;
type I = i32x16;
const L: usize = V::LANES;

#[multiversion]
#[clone(target = "[x86|x86_64]+sse+sse2+sse3+sse4.1+avx+avx2+fma")]
#[clone(target = "[x86|x86_64]+sse+sse2+sse3+sse4.1+avx")]
#[clone(target = "[x86|x86_64]+sse+sse2+sse3+sse4.1")]
fn vector_row(row: &mut [u8], i: f32, x_off: V, pix_size: f32) {
    assert!(row.len() % L == 0);

    for x_grp in 0..row.len() / L {
        let mut iter_cnt = I::splat(0);
        let mut inc = I::splat(1);
        let x_pos = L * x_grp;

        let c = Complex {
            r: x_off + V::splat(x_pos as f32 * pix_size),
            i: V::splat(i),
        };

        let mut z = Complex::default();

        for _ in 0..255 {
            z = z * z + c;
            let over_limit = z.len_sq().ge(V::splat(LIMIT));

            inc = inc.blend(I::default(), over_limit);
            iter_cnt += inc;

            if inc == I::default() {
                break;
            }
        }

        for (i, v) in iter_cnt.iter().enumerate() {
            row[x_pos + i] = *v as u8;
        }
    }
}

pub struct Simd;

impl Compute for Simd {
    fn compute(&self, image: &mut Image, pix_size: f32) {
        let h = image.len();
        let w = image[0].len();
        let x_off = - (w as f32) / 2.0 * pix_size;
        let y_off = - (h as f32) / 2.0 * pix_size;

        // TODO: Collect
        let x_off = (0..L).map(|i| i as f32 * pix_size + x_off).collect::<Vec<_>>();
        let x_off = V::new(x_off);

        image
            .par_iter_mut()
            .enumerate()
            .for_each(|(y, row)| {
                vector_row(row, y_off + (y as f32) * pix_size, x_off, pix_size);
            });
    }
}
