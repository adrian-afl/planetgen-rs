use glam::DVec3;
use std::hash::{DefaultHasher, Hash, Hasher};

fn hash(v: f64) -> f64 {
    let mut s = DefaultHasher::new();
    s.write_u64(v.to_bits());
    (s.finish() as f64) / (u64::MAX as f64)
}

pub fn hash3d(v: DVec3) -> f64 {
    let mut s = DefaultHasher::new();
    s.write_u64(v.x.to_bits());
    s.write_u64(v.y.to_bits());
    s.write_u64(v.z.to_bits());
    (s.finish() as f64) / (u64::MAX as f64)
}

fn mix(a: f64, b: f64, m: f64) -> f64 {
    a * (1.0 - m) + b * m
}

static VEC1: DVec3 = DVec3::new(0.0, 0.0, 0.0);
static VEC2: DVec3 = DVec3::new(0.0, 1.0, 0.0);
static VEC3: DVec3 = DVec3::new(1.0, 0.0, 0.0);
static VEC4: DVec3 = DVec3::new(1.0, 1.0, 0.0);
static VEC5: DVec3 = DVec3::new(0.0, 0.0, 1.0);
static VEC6: DVec3 = DVec3::new(0.0, 1.0, 1.0);
static VEC7: DVec3 = DVec3::new(1.0, 0.0, 1.0);
static VEC8: DVec3 = DVec3::new(1.0, 1.0, 1.0);

pub fn value_noise(x: DVec3) -> f64 {
    let p = x.floor();
    let mut fr = x - p;
    fr = fr * fr * (3.0 - 2.0 * fr);
    let lbz = p + VEC1;
    let ltz = p + VEC2;
    let rbz = p + VEC3;
    let rtz = p + VEC4;

    let lbf = p + VEC5;
    let ltf = p + VEC6;
    let rbf = p + VEC7;
    let rtf = p + VEC8;

    let l0candidate1 = hash3d(lbz);
    let l0candidate2 = hash3d(rbz);
    let l0candidate3 = hash3d(ltz);
    let l0candidate4 = hash3d(rtz);

    let l0candidate5 = hash3d(lbf);
    let l0candidate6 = hash3d(rbf);
    let l0candidate7 = hash3d(ltf);
    let l0candidate8 = hash3d(rtf);

    let l1candidate1 = mix(l0candidate1, l0candidate2, fr.x);
    let l1candidate2 = mix(l0candidate3, l0candidate4, fr.x);
    let l1candidate3 = mix(l0candidate5, l0candidate6, fr.x);
    let l1candidate4 = mix(l0candidate7, l0candidate8, fr.x);

    let l2candidate1 = mix(l1candidate1, l1candidate2, fr.y);
    let l2candidate2 = mix(l1candidate3, l1candidate4, fr.y);

    let l3candidate1 = mix(l2candidate1, l2candidate2, fr.z);

    l3candidate1
}

pub fn fbm(mut pos: DVec3, iterations: i32, scaler: f64, weighter: f64) -> f64 {
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
