use glam::{DVec2, DVec3, DVec4};
use rand_xoshiro::rand_core::{RngCore, SeedableRng};
use rand_xoshiro::Xoshiro256StarStar;

fn random_1d_hasher(v: f64) -> Xoshiro256StarStar {
    Xoshiro256StarStar::seed_from_u64(v.to_bits())
}

fn random_2d_hasher(v: DVec2) -> Xoshiro256StarStar {
    Xoshiro256StarStar::seed_from_u64(
        random_1d_to_1d(v.x).to_bits() ^ random_1d_to_1d(v.y + 1.234).to_bits(),
    )
}

fn random_3d_hasher(v: DVec3) -> Xoshiro256StarStar {
    Xoshiro256StarStar::seed_from_u64(
        random_1d_to_1d(v.x).to_bits()
            ^ random_1d_to_1d(v.y + 1.234).to_bits()
            ^ random_1d_to_1d(v.z + 2.345).to_bits(),
    )
}

fn random_4d_hasher(v: DVec4) -> Xoshiro256StarStar {
    Xoshiro256StarStar::seed_from_u64(
        random_1d_to_1d(v.x).to_bits()
            ^ random_1d_to_1d(v.y + 1.234).to_bits()
            ^ random_1d_to_1d(v.z + 2.345).to_bits()
            ^ random_1d_to_1d(v.z + 3.456).to_bits(),
    )
}

pub fn random_1d_to_1d(v: f64) -> f64 {
    let mut s = random_1d_hasher(v);
    (s.next_u32() as f64) / (u32::MAX as f64) // note here u32 is used to maintain precision...
}

pub fn random_1d_to_2d(v: f64) -> DVec2 {
    let mut s = random_1d_hasher(v);
    DVec2::new(
        (s.next_u32() as f64) / (u32::MAX as f64),
        (s.next_u32() as f64) / (u32::MAX as f64),
    )
}

pub fn random_1d_to_3d(v: f64) -> DVec3 {
    let mut s = random_1d_hasher(v);
    DVec3::new(
        (s.next_u32() as f64) / (u32::MAX as f64),
        (s.next_u32() as f64) / (u32::MAX as f64),
        (s.next_u32() as f64) / (u32::MAX as f64),
    )
}

pub fn random_1d_to_4d(v: f64) -> DVec4 {
    let mut s = random_1d_hasher(v);
    DVec4::new(
        (s.next_u32() as f64) / (u32::MAX as f64),
        (s.next_u32() as f64) / (u32::MAX as f64),
        (s.next_u32() as f64) / (u32::MAX as f64),
        (s.next_u32() as f64) / (u32::MAX as f64),
    )
}

pub fn random_1d_to_array<const COUNT: usize>(v: f64) -> [f64; COUNT] {
    let mut s = random_1d_hasher(v);
    [0.0; COUNT].map(|_| (s.next_u32() as f64) / (u32::MAX as f64))
}

pub fn random_2d_to_1d(v: DVec2) -> f64 {
    let mut s = random_2d_hasher(v);
    (s.next_u32() as f64) / (u32::MAX as f64) // note here u32 is used to maintain precision...
}

pub fn random_2d_to_2d(v: DVec2) -> DVec2 {
    let mut s = random_2d_hasher(v);
    DVec2::new(
        (s.next_u32() as f64) / (u32::MAX as f64),
        (s.next_u32() as f64) / (u32::MAX as f64),
    )
}

pub fn random_2d_to_3d(v: DVec2) -> DVec3 {
    let mut s = random_2d_hasher(v);
    DVec3::new(
        (s.next_u32() as f64) / (u32::MAX as f64),
        (s.next_u32() as f64) / (u32::MAX as f64),
        (s.next_u32() as f64) / (u32::MAX as f64),
    )
}

pub fn random_2d_to_4d(v: DVec2) -> DVec4 {
    let mut s = random_2d_hasher(v);
    DVec4::new(
        (s.next_u32() as f64) / (u32::MAX as f64),
        (s.next_u32() as f64) / (u32::MAX as f64),
        (s.next_u32() as f64) / (u32::MAX as f64),
        (s.next_u32() as f64) / (u32::MAX as f64),
    )
}

pub fn random_2d_to_array<const COUNT: usize>(v: DVec2) -> [f64; COUNT] {
    let mut s = random_2d_hasher(v);
    [0.0; COUNT].map(|_| (s.next_u32() as f64) / (u32::MAX as f64))
}

pub fn random_3d_to_1d(v: DVec3) -> f64 {
    let mut s = random_3d_hasher(v);
    (s.next_u32() as f64) / (u32::MAX as f64) // note here u32 is used to maintain precision...
}

pub fn random_3d_to_2d(v: DVec3) -> DVec2 {
    let mut s = random_3d_hasher(v);
    DVec2::new(
        (s.next_u32() as f64) / (u32::MAX as f64),
        (s.next_u32() as f64) / (u32::MAX as f64),
    )
}

pub fn random_3d_to_3d(v: DVec3) -> DVec3 {
    let mut s = random_3d_hasher(v);
    DVec3::new(
        (s.next_u32() as f64) / (u32::MAX as f64),
        (s.next_u32() as f64) / (u32::MAX as f64),
        (s.next_u32() as f64) / (u32::MAX as f64),
    )
}

pub fn random_3d_to_4d(v: DVec3) -> DVec4 {
    let mut s = random_3d_hasher(v);
    DVec4::new(
        (s.next_u32() as f64) / (u32::MAX as f64),
        (s.next_u32() as f64) / (u32::MAX as f64),
        (s.next_u32() as f64) / (u32::MAX as f64),
        (s.next_u32() as f64) / (u32::MAX as f64),
    )
}

pub fn random_3d_to_array<const COUNT: usize>(v: DVec3) -> [f64; COUNT] {
    let mut s = random_3d_hasher(v);
    [0.0; COUNT].map(|_| (s.next_u32() as f64) / (u32::MAX as f64))
}

pub fn random_4d_to_1d(v: DVec4) -> f64 {
    let mut s = random_4d_hasher(v);
    (s.next_u32() as f64) / (u32::MAX as f64) // note here u32 is used to maintain precision...
}

pub fn random_4d_to_2d(v: DVec4) -> DVec2 {
    let mut s = random_4d_hasher(v);
    DVec2::new(
        (s.next_u32() as f64) / (u32::MAX as f64),
        (s.next_u32() as f64) / (u32::MAX as f64),
    )
}

pub fn random_4d_to_3d(v: DVec4) -> DVec3 {
    let mut s = random_4d_hasher(v);
    DVec3::new(
        (s.next_u32() as f64) / (u32::MAX as f64),
        (s.next_u32() as f64) / (u32::MAX as f64),
        (s.next_u32() as f64) / (u32::MAX as f64),
    )
}

pub fn random_4d_to_4d(v: DVec4) -> DVec4 {
    let mut s = random_4d_hasher(v);
    DVec4::new(
        (s.next_u32() as f64) / (u32::MAX as f64),
        (s.next_u32() as f64) / (u32::MAX as f64),
        (s.next_u32() as f64) / (u32::MAX as f64),
        (s.next_u32() as f64) / (u32::MAX as f64),
    )
}

pub fn random_4d_to_array<const COUNT: usize>(v: DVec4) -> [f64; COUNT] {
    let mut s = random_4d_hasher(v);
    [0.0; COUNT].map(|_| (s.next_u32() as f64) / (u32::MAX as f64))
}
