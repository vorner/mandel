use anyhow::Error;
use image::{ImageBuffer, Luma};
use mandel::{Compute, Simd};

const SIZE: usize = 512;

fn main() -> Result<(), Error> {
    let mut img = vec![vec![0; SIZE]; SIZE];
    Simd.compute(&mut img, 0.00625);

    let img = ImageBuffer::from_fn(SIZE as _, SIZE as _, |x, y| {
        Luma([img[y as usize][x as usize]])
    });

    img.save("plot.png")?;

    Ok(())
}
