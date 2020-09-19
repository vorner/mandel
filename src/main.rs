use anyhow::Error;
use image::{ImageBuffer, Luma};
use mandel::{Compute, Base};

fn main() -> Result<(), Error> {
    let mut img = vec![vec![0; 256]; 256];
    Base.compute(&mut img, 0.25, -64.0, -64.0);

    let img = ImageBuffer::from_fn(256, 256, |x, y| {
        Luma([img[y as usize][x as usize]])
    });

    img.save("plot.png")?;

    Ok(())
}
