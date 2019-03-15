


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
