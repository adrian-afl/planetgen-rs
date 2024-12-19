use glam::{DVec2, DVec3};
use std::f64::consts::PI;
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

fn hashv3GLSL(p: DVec3) -> f64{
    (4768.1232345456 * ((p.x+p.y*43.0+p.z*137.0)).sin()).fract()
}

fn mix(a: f64, b: f64, m: f64) -> f64 {
    a * (1.0 - m) + b * m
}

fn value_noise(x: DVec3) -> f64 {
    let p = x.floor();
    let mut fr = x - p;
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

    l3candidate1
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

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let p = polar_to_xyz(DVec2::new((x as f64) / (imgx as f64), (y as f64) / (imgy as f64)));
        let v3 = DVec3::new(0.0, (x as f64) / (imgx as f64), (y as f64) / (imgy as f64));

        let value = fbm(p * 1.0, 20, 3.0, 0.6);
        // let value = value_noise(p * 20.0);

        *pixel = image::Luma([
            (value * 255.0) as u8,
        ]);
    }

    imgbuf.save("fractal.png").unwrap();
}
