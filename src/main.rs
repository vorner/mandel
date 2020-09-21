use anyhow::Error;
use image::{ImageBuffer, Luma};
use mandel::{Compute, Parallel};

fn main() -> Result<(), Error> {
    let mut img = vec![vec![0; 256]; 256];
    Parallel.compute(&mut img, 0.0125);

    let img = ImageBuffer::from_fn(256, 256, |x, y| {
        Luma([img[y as usize][x as usize]])
    });

    img.save("plot.png")?;

    Ok(())
}
