use multiversion::multiversion;
use packed_simd::{f32x8, u8x8, SimdVector};
use rayon::prelude::*;

use super::{Complex, Compute, Image, LIMIT};

type V = f32x8;
type I = u8x8;
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

            inc = over_limit.select(I::default(), inc);
            iter_cnt += inc;

            if inc == I::default() {
                break;
            }
        }

        iter_cnt.write_to_slice_unaligned(&mut row[x_pos..x_pos + L]);
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
        let x_off = V::from_slice_unaligned(&x_off);

        image
            .par_iter_mut()
            .enumerate()
            .for_each(|(y, row)| {
                vector_row(row, y_off + (y as f32) * pix_size, x_off, pix_size);
            });
    }
}
