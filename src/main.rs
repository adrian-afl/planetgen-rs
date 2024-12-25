mod erosion;
mod input_data;
mod noise;
mod random;

use glam::{DVec2, DVec3};
use rayon::iter::ParallelIterator;
use std::f64::consts::PI;
use std::hash::{Hash, Hasher};
use std::time::Instant;

use crate::noise::fbm;
use crate::random::random_1d_to_array;

fn polar_to_xyz(xyin: DVec2) -> DVec3 {
    let xy = xyin * DVec2::new(2.0 * PI, PI);
    let z = xy.y.cos();
    let x = xy.x.cos() * xy.y.sin();
    let y = xy.x.sin() * xy.y.sin();
    DVec3::new(x, y, z).normalize()
}

fn main() {
    let imgx = 2048 * 2;
    let imgy = 2048;

    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);

    // for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
    //     let p = polar_to_xyz(DVec2::new((x as f64) / (imgx as f64), (y as f64) / (imgy as f64)));
    //     let v3 = DVec3::new(0.0, (x as f64) / (imgx as f64), (y as f64) / (imgy as f64));
    //
    //     let value = fbm(p * 1.0, 20, 3.0, 0.6);
    //     // let value = value_noise(p * 20.0);
    //
    //     *pixel = image::Luma([
    //         (value * 255.0) as u8,
    //     ]);
    // }

    let start = Instant::now();
    imgbuf.par_enumerate_pixels_mut().for_each(|(x, y, pixel)| {
        let p = polar_to_xyz(DVec2::new(
            (x as f64) / (imgx as f64),
            (y as f64) / (imgy as f64),
        ));
        let v3 = DVec3::new(0.0, (x as f64) / (imgx as f64), (y as f64) / (imgy as f64));

        let value = fbm(p * 1.0, 10, 3.0, 0.6);

        *pixel = image::Luma([(value * 255.0) as u8]);
    });
    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);

    let arr: [f64; 5] = random_1d_to_array(1.234);
    println!("{:?}", arr);

    imgbuf.save("fractal.png").unwrap();
}
