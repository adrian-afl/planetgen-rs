use glam::{DVec3, DVec4};
use std::cmp::Ordering;

use std::hash::{DefaultHasher, Hash, Hasher};

fn hash(v: f64) -> f64 {
    let mut s = DefaultHasher::new();
    s.write_u64(v.to_bits());
    (s.finish() as f64) / (u64::MAX as f64)
}

fn hashv3(v: DVec3) -> f64 {
    let mut s = DefaultHasher::new();
    s.write_u64(v.x.to_bits());
    s.write_u64(v.y.to_bits());
    s.write_u64(v.z.to_bits());
    (s.finish() as f64) / (u64::MAX as f64)
}

fn mix(a: f64, b: f64, m: f64) -> f64 {
    b * m + a * (1.0 - m)
}

fn value_noise(x: DVec3) -> f64 {
    let p = x.floor();
    let mut fr = x.fract();
    fr = fr * fr * (3.0 - 2.0 * fr);
    let lbz = p + DVec3::new(0.0, 0.0, 0.0);
    let ltz = p + DVec3::new(0.0, 1.0, 0.0);
    let rbz = p + DVec3::new(1.0, 0.0, 0.0);
    let rtz = p + DVec3::new(1.0, 1.0, 0.0);

    let lbf = p + DVec3::new(0.0, 0.0, 1.0);
    let ltf = p + DVec3::new(0.0, 1.0, 1.0);
    let rbf = p + DVec3::new(1.0, 0.0, 1.0);
    let rtf = p + DVec3::new(1.0, 1.0, 1.0);

    let l0candidate1 = hashv3(lbz);
    let l0candidate2 = hashv3(rbz);
    let l0candidate3 = hashv3(ltz);
    let l0candidate4 = hashv3(rtz);

    let l0candidate5 = hashv3(lbf);
    let l0candidate6 = hashv3(rbf);
    let l0candidate7 = hashv3(ltf);
    let l0candidate8 = hashv3(rtf);

    let l1candidate1 = mix(l0candidate1, l0candidate2, fr.x);
    let l1candidate2 = mix(l0candidate3, l0candidate4, fr.x);
    let l1candidate3 = mix(l0candidate5, l0candidate6, fr.x);
    let l1candidate4 = mix(l0candidate7, l0candidate8, fr.x);

    let l2candidate1 = mix(l1candidate1, l1candidate2, fr.y);
    let l2candidate2 = mix(l1candidate3, l1candidate4, fr.y);

    let l3candidate1 = mix(l2candidate1, l2candidate2, fr.z);

    return l3candidate1;
}

fn fbm(mut pos: DVec3, iterations: i32, scaler: f64, weighter: f64) -> f64 {
    let mut res = 0.0;
    let mut w = 0.5;
    pos += 2.0;
    for i in 0..iterations {
        res += value_noise(pos) * w;
        pos *= scaler;
        w *= weighter;
    }
    res
}

fn main() {
    let a = DVec4::new(1.0, 2.0, 3.0, 4.0);
    let b = DVec4::new(3.0, 3.0, 33.0, 3.0);
    let c = DVec3::new(3.0, 3.0, 33.0);

    match a.x.partial_cmp(&b.x) {
        None => panic!("nanananana"),
        Some(x) => match x {
            Ordering::Less => println!("actually less"),
            Ordering::Equal => println!("actually more!"),
            Ordering::Greater => println!(" uh oh eq"),
        },
    }
    println!("{}", value_noise(c));
    println!("{}", hash(123.0));

    let imgx = 800;
    let imgy = 800;

    let scalex = 3.0 / imgx as f32;
    let scaley = 3.0 / imgy as f32;

    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let r = (0.3 * x as f32) as u8;
        let b = (0.3 * y as f32) as u8;
        *pixel = image::Rgb([r, 0, b]);
    }

    // A redundant loop to demonstrate reading image data
    for x in 0..imgx {
        print!("{}.", x);
        for y in 0..imgy {
            let p = DVec3::new(x as f64 / imgx as f64, y as f64 / imgy as f64, 10.0);

            let value = fbm(p * 10.0, 10, 2.0, 0.5);

            let pixel = imgbuf.get_pixel_mut(x, y);
            let image::Rgb(data) = *pixel;
            *pixel = image::Rgb([(value * 255.0) as u8, (value * 255.0) as u8, (value * 255.0) as u8]);
        }
    }

    // Save the image as “fractal.png”, the format is deduced from the path
    imgbuf.save("fractal.png").unwrap();
}
