pub fn mix(a: f64, b: f64, m: f64) -> f64 {
    a * (1.0 - m) + b * m
}

pub fn map(x: f64, in_min: f64, in_max: f64, out_min: f64, out_max: f64) -> f64 {
    (x - in_min) * (out_max - out_min) / (in_max - in_min) + out_min
}

pub fn usat(a: f64) -> f64 {
    a.min(1.0).max(0.0)
}
