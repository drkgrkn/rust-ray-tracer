pub fn random_f64() -> f64 {
    fastrand::f64()
}

pub fn random_f64_between(min: f64, max: f64) -> f64 {
    min + (max - min) * fastrand::f64()
}
