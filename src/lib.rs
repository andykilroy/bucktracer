use std::ops::*;

const EPSILON: f64 = 1e-5;

#[derive(Debug)]
pub struct Direction(f64, f64, f64, f64);

pub fn is_point(Direction(_, _, _, w): Direction) -> bool {
    w == 1.0
}

pub fn is_vector(v: Direction) -> bool {
    !is_point(v)
}

pub fn point(x: f64, y: f64, z: f64) -> Direction {
    Direction(x, y, z, 1.0)
}

pub fn vector(x: f64, y: f64, z: f64) -> Direction {
    Direction(x, y, z, 0.0)
}

impl PartialEq for Direction {

    fn eq(&self, Direction(b1, b2, b3, b4): &Direction) -> bool {
        match self {
            Direction(a1, a2, a3, a4) => {
                if *a4 != *b4 {
                    return false;
                }

                if !almost_eq(*a1, *b1) {
                    return false;
                }

                if !almost_eq(*a2, *b2) {
                    return false;
                }

                if !almost_eq(*a3, *b3) {
                    return false;
                }
                return true;
            }
        }
        return false;
    }

}

fn almost_eq(x1: f64, x2: f64) -> bool {
    f64::abs(x1 - x2) < EPSILON
}

impl Add for Direction {
    type Output = Direction;

    fn add(self, Direction(b1, b2, b3, b4): Direction) -> Direction {
        match self {
            Direction(a1, a2, a3, a4) =>
                Direction(a1 + b1, a2 + b2, a3 + b3, a4 + b4)
        }
    }
}

impl Sub for Direction {
    type Output = Direction;

    fn sub(self, Direction(b1, b2, b3, b4): Direction) -> Direction {
        match self {
            Direction(a1, a2, a3, a4) =>
                Direction(a1 - b1, a2 - b2, a3 - b3, a4 - b4)
        }
    }
}

impl Neg for Direction {
    type Output = Direction;

    fn neg(self) -> Direction {
        match self {
            Direction(x, y, z, w) =>
                Direction(-x, -y, -z, -w)
        }
    }
}

impl Direction {
    pub fn scale(self, c: f64) -> Direction {
        match self {
            Direction(x, y, z, w) =>
                Direction(c*x, c*y, c*z, c*w)
        }
    }

    pub fn magni(self) -> f64 {
        match self {
            Direction(x, y, z, w) =>
                f64::sqrt(x.powi(2) + y.powi(2) + z.powi(2) + w.powi(2))
        }
    }
}
