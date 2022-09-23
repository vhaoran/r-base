pub fn is_zero(f: f64) -> bool {
    const ZERO: f64 = 0.0000000000000001_f64;
    f.abs() < ZERO
}
