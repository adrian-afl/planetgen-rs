use crate::random::random_3d_to_1d;
use glam::DVec3;
use rayon::prelude::*;
use std::sync::atomic::{AtomicI64, Ordering};

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

    let l0candidate1 = random_3d_to_1d(lbz);
    let l0candidate2 = random_3d_to_1d(rbz);
    let l0candidate3 = random_3d_to_1d(ltz);
    let l0candidate4 = random_3d_to_1d(rtz);

    let l0candidate5 = random_3d_to_1d(lbf);
    let l0candidate6 = random_3d_to_1d(rbf);
    let l0candidate7 = random_3d_to_1d(ltf);
    let l0candidate8 = random_3d_to_1d(rtf);

    let l1candidate1 = mix(l0candidate1, l0candidate2, fr.x);
    let l1candidate2 = mix(l0candidate3, l0candidate4, fr.x);
    let l1candidate3 = mix(l0candidate5, l0candidate6, fr.x);
    let l1candidate4 = mix(l0candidate7, l0candidate8, fr.x);

    let l2candidate1 = mix(l1candidate1, l1candidate2, fr.y);
    let l2candidate2 = mix(l1candidate3, l1candidate4, fr.y);

    let l3candidate1 = mix(l2candidate1, l2candidate2, fr.z);

    l3candidate1
}

pub fn super_value_noise(x: DVec3) -> f64 {
    (value_noise(x) + value_noise(x + 4.5)) * 0.5
}

pub fn fbm(pos: DVec3, iterations: u16, scaler: f64, weighter: f64) -> f64 {
    let mut res = 0.0;
    let mut w = 0.5;
    let mut s = 1.0;
    for i in 0..iterations {
        res += super_value_noise((pos + 2.0) * s) * w;
        s *= scaler;
        w *= weighter;
    }
    res
    // let mut atomic = AtomicI64::new(0);
    // (0..iterations).into_par_iter().for_each(|i| {
    //     let w = 0.5f64.powf(i as f64);
    //     let s = (i as f64) * 3.0 + 1.0;
    //     let val = value_noise((pos + 2.0) * s) * w;
    //     atomic.fetch_add((val * (i32::MAX) as f64) as i64, Ordering::Relaxed);
    // });
    // atomic.into_inner() as f64 / (i32::MAX) as f64 / iterations as f64
}
