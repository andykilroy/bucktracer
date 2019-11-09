mod all;


fn almost_eq(x1: f64, x2: f64) -> bool {
    f64::abs(x1 - x2) < 1e-5
}
