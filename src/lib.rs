
const EPSILON: f64 = 1e-5;

type Direction = (f64, f64, f64, f64);

pub fn is_point((x, y, z, w): Direction) -> bool {
    w == 1.0
}

pub fn is_vector(v: Direction) -> bool {
    !is_point(v)
}

pub fn point(x: f64, y: f64, z: f64) -> Direction {
    (x, y, z, 1.0)
}

pub fn vector(x: f64, y: f64, z: f64) -> Direction {
    (x, y, z, 0.0)
}

pub fn equalish((a1, a2, a3, a4): Direction, (b1, b2, b3, b4): Direction) -> bool {
    if a4 != b4 {
        return false;
    }

    if !almost_eq(a1, b1) {
        return false;
    }
    
    if !almost_eq(a2, b2) {
        return false;
    }

    if !almost_eq(a3, b3) {
        return false;
    }
    return true;
}

fn almost_eq(x1: f64, x2: f64) -> bool {
    f64::abs(x1 - x2) < EPSILON
}
