use std::ops::{Add, Index, Mul, Neg, Sub};
use std::str::FromStr;
use std::fmt;
use regex::Regex;

use super::almost_eq;

#[derive(Debug, Copy, Clone)]
pub struct Tuple4(f64, f64, f64, f64);

#[allow(clippy::float_cmp)]
/// Detect if a Tuple4 represents a point in space
pub fn is_point(Tuple4(_, _, _, w): Tuple4) -> bool {
    w == 1.0
}

/// Detect if a Tuple4 represents a vector, a direction in space.
pub fn is_vector(v: Tuple4) -> bool {
    !is_point(v)
}

/// Create a 3D point.
pub fn point(x: f64, y: f64, z: f64) -> Tuple4 {
    Tuple4(x, y, z, 1.0)
}

/// Create a 3D vector.
pub fn vector(x: f64, y: f64, z: f64) -> Tuple4 {
    Tuple4(x, y, z, 0.0)
}

/// Create a 4D tuple.
pub fn tuple(x: f64, y: f64, z: f64, w: f64) -> Tuple4 {
    Tuple4(x, y, z, w)
}

impl PartialEq for Tuple4 {
    fn eq(&self, Tuple4(b1, b2, b3, b4): &Tuple4) -> bool {
        let Tuple4(a1, a2, a3, a4) = self;
        almost_eq(*a1, *b1) && almost_eq(*a2, *b2) && almost_eq(*a3, *b3) && almost_eq(*a4, *b4)
    }
}

impl Add for Tuple4 {
    type Output = Tuple4;

    fn add(self, Tuple4(b1, b2, b3, b4): Tuple4) -> Tuple4 {
        let Tuple4(a1, a2, a3, a4) = self;
        Tuple4(a1 + b1, a2 + b2, a3 + b3, a4 + b4)
    }
}

impl Sub for Tuple4 {
    type Output = Tuple4;

    fn sub(self, Tuple4(b1, b2, b3, b4): Tuple4) -> Tuple4 {
        let Tuple4(a1, a2, a3, a4) = self;
        Tuple4(a1 - b1, a2 - b2, a3 - b3, a4 - b4)
    }
}

impl Neg for Tuple4 {
    type Output = Tuple4;

    fn neg(self) -> Tuple4 {
        let Tuple4(x, y, z, w) = self;
        Tuple4(-x, -y, -z, -w)
    }
}

impl Tuple4 {
    pub fn x(&self) -> f64 {
        self.0
    }

    pub fn y(&self) -> f64 {
        self.1
    }

    pub fn z(&self) -> f64 {
        self.2
    }

    pub fn w(&self) -> f64 {
        self.3
    }

    pub fn scale(self, factor: f64) -> Tuple4 {
        let Tuple4(x, y, z, w) = self;
        Tuple4(factor * x, factor * y, factor * z, factor * w)
    }

    pub fn magnitude(self) -> f64 {
        let Tuple4(x, y, z, w) = self;
        f64::sqrt(x.powi(2) + y.powi(2) + z.powi(2) + w.powi(2))
    }

    pub fn normalize(self) -> Tuple4 {
        let Tuple4(x, y, z, w) = self;
        let divisor = self.magnitude();
        Tuple4(x / divisor, y / divisor, z / divisor, w / divisor)
    }

    pub fn dot(self, Tuple4(x2, y2, z2, w2): Tuple4) -> f64 {
        let Tuple4(x1, y1, z1, w1) = self;
        x1 * x2 + y1 * y2 + z1 * z2 + w1 * w2
    }

    pub fn cross(self, Tuple4(b1, b2, b3, _b4): Tuple4) -> Tuple4 {
        let Tuple4(a1, a2, a3, _a4) = self;
        vector(a2 * b3 - a3 * b2, a3 * b1 - a1 * b3, a1 * b2 - a2 * b1)
    }

    // hadamard product
    pub fn mult_pairwise(self, Tuple4(b1, b2, b3, b4): Tuple4) -> Tuple4 {
        let Tuple4(a1, a2, a3, a4) = self;
        Tuple4(a1 * b1, a2 * b2, a3 * b3, a4 * b4)
    }

    pub fn min(a: Tuple4, b: Tuple4) -> Tuple4{
        Tuple4(a.x().min(b.x()), a.y().min(b.y()), a.z().min(b.z()), a.w().min(b.w()))
    }

    pub fn max(a: Tuple4, b: Tuple4) -> Tuple4{
        Tuple4(a.x().max(b.x()), a.y().max(b.y()), a.z().max(b.z()), a.w().max(b.w()))
    }

    pub fn min_all(items: &[Tuple4]) -> Tuple4 {
        use std::f64::INFINITY;
        let mut minp = tuple(INFINITY, INFINITY, INFINITY, INFINITY);

        for p in items {
            minp = Tuple4::min(*p, minp);
        }
        minp
    }

    pub fn max_all(items: &[Tuple4]) -> Tuple4 {
        use std::f64::NEG_INFINITY;
        let mut maxp = tuple(NEG_INFINITY, NEG_INFINITY, NEG_INFINITY, NEG_INFINITY);

        for p in items {
            maxp = Tuple4::max(*p, maxp);
        }
        maxp
    }
}

impl Index<usize> for Tuple4 {
    type Output = f64;

    fn index(&self, index: usize) -> &f64 {
        let Tuple4(x, y, z, w) = self;
        match index {
            0 => &x,
            1 => &y,
            2 => &z,
            3 => &w,
            _ => panic!("unexpected index"),
        }
    }
}


#[derive(Debug)]
pub enum Error {
    BadTuple
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl FromStr for Tuple4 {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {return Err(Error::BadTuple);}
        expect_braces(s)
    }

}

fn expect_braces(s: & str) -> Result<Tuple4, Error> {
    if s.starts_with("(") {
        let i = s.find(')').ok_or_else(|| Error::BadTuple)?;
        parse_tuple4(&s[1..i])
    } else {
        Err(Error::BadTuple)
    }
}


fn parse_tuple4(s: &str) -> Result<Tuple4, Error> {
    lazy_static! {
        static ref TUPLE4_RE: Regex = Regex::new(r"([^,]+),([^,]+),([^,]+)(,([^,]+))?$").unwrap();
    }
    let captures = TUPLE4_RE.captures(s).ok_or_else(|| Error::BadTuple)?;
    let mut parts: Vec<String> = Vec::with_capacity(4);

    for (i, om) in captures.iter().enumerate() {
        if i == 0 { continue; }
        if let Some(m) = om {
            parts.push(m.as_str().to_string());
        }
    }

    if parts.len() == 3 {
        return Ok(point(consume_one_f64(parts[0].trim())?,
                        consume_one_f64(parts[1].trim())?,
                        consume_one_f64(parts[2].trim())?,
        ));
    }
    if parts.len() == 5 {
        return Ok(tuple(consume_one_f64(parts[0].trim())?,
                        consume_one_f64(parts[1].trim())?,
                        consume_one_f64(parts[2].trim())?,
                        consume_one_f64(parts[4].trim())?,
        ));
    }

    Err(Error::BadTuple)
}

fn consume_one_f64(s: &str) -> Result<f64, Error> {
    let f = s[0..].parse::<f64>().or_else(|_| Err(Error::BadTuple))?;
    Ok(f)
}

#[derive(Debug, Copy, Clone)]
pub struct Matrix {
    dim: Dimensions,
    r1: Tuple4,
    r2: Tuple4,
    r3: Tuple4,
    r4: Tuple4,
}

#[derive(Debug, Copy, Clone)]
enum Dimensions {
    X2,
    X3,
    X4,
}

/// Specify the rows of a 4x4 matrix.  The first parameter is the first row,
/// The second parameter the second row, etc.
pub fn matrix(
    (x1, y1, z1, w1): (f64, f64, f64, f64),
    (x2, y2, z2, w2): (f64, f64, f64, f64),
    (x3, y3, z3, w3): (f64, f64, f64, f64),
    (x4, y4, z4, w4): (f64, f64, f64, f64),
) -> Matrix {
    Matrix {
        dim: Dimensions::X4,
        r1: Tuple4(x1, y1, z1, w1),
        r2: Tuple4(x2, y2, z2, w2),
        r3: Tuple4(x3, y3, z3, w3),
        r4: Tuple4(x4, y4, z4, w4),
    }
}

fn matrix2((x1, y1): (f64, f64), (x2, y2): (f64, f64)) -> Matrix {
    Matrix {
        dim: Dimensions::X2,
        r1: Tuple4(x1, y1, 0.0, 0.0),
        r2: Tuple4(x2, y2, 0.0, 0.0),
        r3: Tuple4(0.0, 0.0, 0.0, 0.0),
        r4: Tuple4(0.0, 0.0, 0.0, 0.0),
    }
}

fn matrix3(
    (x1, y1, z1): (f64, f64, f64),
    (x2, y2, z2): (f64, f64, f64),
    (x3, y3, z3): (f64, f64, f64),
) -> Matrix {
    Matrix {
        dim: Dimensions::X3,
        r1: Tuple4(x1, y1, z1, 0.0),
        r2: Tuple4(x2, y2, z2, 0.0),
        r3: Tuple4(x3, y3, z3, 0.0),
        r4: Tuple4(0.0, 0.0, 0.0, 0.0),
    }
}

/// The identity matrix.
pub fn identity() -> Matrix {
    matrix(
        (1.0, 0.0, 0.0, 0.0),
        (0.0, 1.0, 0.0, 0.0),
        (0.0, 0.0, 1.0, 0.0),
        (0.0, 0.0, 0.0, 1.0),
    )
}

impl Index<usize> for Matrix {
    type Output = Tuple4;

    fn index(&self, index: usize) -> &Tuple4 {
        match index {
            0 => &self.r1,
            1 => &self.r2,
            2 => &self.r3,
            3 => &self.r4,
            _ => panic!("unexpected index for matrix"),
        }
    }
}

impl PartialEq for Matrix {
    fn eq(&self, y: &Matrix) -> bool {
        self.r1 == y.r1 && self.r2 == y.r2 && self.r3 == y.r3 && self.r4 == y.r4
    }
}

impl Mul for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Matrix) -> Matrix {
        mult_mat(&self, &rhs)
    }
}

fn mult_mat(a: &Matrix, b: &Matrix) -> Matrix {
    let c1 = b.col(0);
    let c2 = b.col(1);
    let c3 = b.col(2);
    let c4 = b.col(3);

    let a1b1 = a.r1.dot(c1);
    let a1b2 = a.r1.dot(c2);
    let a1b3 = a.r1.dot(c3);
    let a1b4 = a.r1.dot(c4);

    let a2b1 = a.r2.dot(c1);
    let a2b2 = a.r2.dot(c2);
    let a2b3 = a.r2.dot(c3);
    let a2b4 = a.r2.dot(c4);

    let a3b1 = a.r3.dot(c1);
    let a3b2 = a.r3.dot(c2);
    let a3b3 = a.r3.dot(c3);
    let a3b4 = a.r3.dot(c4);

    let a4b1 = a.r4.dot(c1);
    let a4b2 = a.r4.dot(c2);
    let a4b3 = a.r4.dot(c3);
    let a4b4 = a.r4.dot(c4);

    matrix(
        (a1b1, a1b2, a1b3, a1b4),
        (a2b1, a2b2, a2b3, a2b4),
        (a3b1, a3b2, a3b3, a3b4),
        (a4b1, a4b2, a4b3, a4b4),
    )
}

impl Matrix {
    pub fn mult(&self, rhs: Tuple4) -> Tuple4 {
        let a = self.r1.dot(rhs);
        let b = self.r2.dot(rhs);
        let c = self.r3.dot(rhs);
        let d = self.r4.dot(rhs);
        Tuple4(a, b, c, d)
    }

    pub fn col(&self, i: usize) -> Tuple4 {
        Tuple4(self[0][i], self[1][i], self[2][i], self[3][i])
    }

    pub fn transpose(&self) -> Matrix {
        matrix(
            (self[0][0], self[1][0], self[2][0], self[3][0]),
            (self[0][1], self[1][1], self[2][1], self[3][1]),
            (self[0][2], self[1][2], self[2][2], self[3][2]),
            (self[0][3], self[1][3], self[2][3], self[3][3]),
        )
    }

    pub fn det(&self) -> f64 {
        match self.dim {
            Dimensions::X2 => det_x2(self),
            Dimensions::X3 => det_x3(self),
            Dimensions::X4 => det_x4(self),
        }
    }

    pub fn submatrix(&self, row: usize, col: usize) -> Matrix {
        match self.dim {
            Dimensions::X3 => submatr_x3(self, row, col),
            Dimensions::X4 => submatr_x4(self, row, col),
            _ => panic!("no expected submatrix of a 2x2 matrix"),
        }
    }

    pub fn minor(&self, row: usize, col: usize) -> f64 {
        self.submatrix(row, col).det()
    }

    pub fn cofactor(&self, row: usize, col: usize) -> f64 {
        if (row + col) % 2 == 0 {
            self.minor(row, col)
        } else {
            -self.minor(row, col)
        }
    }

    pub fn cofactors(&self) -> Matrix {
        matrix(
            (
                self.cofactor(0, 0),
                self.cofactor(0, 1),
                self.cofactor(0, 2),
                self.cofactor(0, 3),
            ),
            (
                self.cofactor(1, 0),
                self.cofactor(1, 1),
                self.cofactor(1, 2),
                self.cofactor(1, 3),
            ),
            (
                self.cofactor(2, 0),
                self.cofactor(2, 1),
                self.cofactor(2, 2),
                self.cofactor(2, 3),
            ),
            (
                self.cofactor(3, 0),
                self.cofactor(3, 1),
                self.cofactor(3, 2),
                self.cofactor(3, 3),
            ),
        )
    }

    fn scale_elems(&self, y: f64) -> Matrix {
        Matrix {
            dim: self.dim,
            r1: self.r1.scale(y),
            r2: self.r2.scale(y),
            r3: self.r3.scale(y),
            r4: self.r4.scale(y),
        }
    }

    pub fn inverse(&self) -> Matrix {
        self.cofactors().transpose().scale_elems(1.0 / self.det())
    }

    pub fn translate(self, x: f64, y: f64, z: f64) -> Matrix {
        translation(x, y, z) * self
    }

    pub fn scale(self, x: f64, y: f64, z: f64) -> Matrix {
        scaling(x, y, z) * self
    }

    pub fn rotate_x(self, r: f64) -> Matrix {
        rotation_x(r) * self
    }

    pub fn rotate_y(self, r: f64) -> Matrix {
        rotation_y(r) * self
    }

    pub fn rotate_z(self, r: f64) -> Matrix {
        rotation_z(r) * self
    }
}

fn det_x2(orig: &Matrix) -> f64 {
    let a = orig[0][0];
    let b = orig[0][1];
    let c = orig[1][0];
    let d = orig[1][1];
    (a * d) - (b * c)
}

fn det_x3(x: &Matrix) -> f64 {
    x[0][0] * x.cofactor(0, 0) + x[0][1] * x.cofactor(0, 1) + x[0][2] * x.cofactor(0, 2)
}

fn det_x4(x: &Matrix) -> f64 {
    x[0][0] * x.cofactor(0, 0)
        + x[0][1] * x.cofactor(0, 1)
        + x[0][2] * x.cofactor(0, 2)
        + x[0][3] * x.cofactor(0, 3)
}

fn submatr_x3(x: &Matrix, row: usize, col: usize) -> Matrix {
    if row >= 3 {
        panic!("row for a 3x3 matrix expected to be 0, 1, or 2")
    }
    if col >= 3 {
        panic!("col for a 3x3 matrix expected to be 0, 1, or 2")
    }
    let mut write_index = 0;
    let mut collected: [f64; 4] = [0.0; 4];
    for r in 0..3 {
        for c in 0..3 {
            if r != row && c != col {
                collected[write_index] = x[r][c];
                write_index += 1;
            }
        }
    }

    matrix2((collected[0], collected[1]), (collected[2], collected[3]))
}

fn submatr_x4(x: &Matrix, row: usize, col: usize) -> Matrix {
    if row >= 4 {
        panic!("row for a 4x4 matrix expected to be 0, 1, 2 or 3")
    }
    if col >= 4 {
        panic!("col for a 4x4 matrix expected to be 0, 1, 2 or 3")
    }

    let mut write_index = 0;
    let mut collected: [f64; 9] = [0.0; 9];
    for r in 0..4 {
        for c in 0..4 {
            if r != row && c != col {
                collected[write_index] = x[r][c];
                write_index += 1;
            }
        }
    }

    matrix3(
        (collected[0], collected[1], collected[2]),
        (collected[3], collected[4], collected[5]),
        (collected[6], collected[7], collected[8]),
    )
}

/// Create a translation matrix.
pub fn translation(x: f64, y: f64, z: f64) -> Matrix {
    matrix(
        (1.0, 0.0, 0.0, x),
        (0.0, 1.0, 0.0, y),
        (0.0, 0.0, 1.0, z),
        (0.0, 0.0, 0.0, 1.0),
    )
}

/// Create a scaling matrix.
pub fn scaling(x: f64, y: f64, z: f64) -> Matrix {
    matrix(
        (x, 0.0, 0.0, 0.0),
        (0.0, y, 0.0, 0.0),
        (0.0, 0.0, z, 0.0),
        (0.0, 0.0, 0.0, 1.0),
    )
}


/// Create a matrix specifying a clockwise rotation around the x axis.
/// The argument is specified in radians.
pub fn rotation_x(r: f64) -> Matrix {
    matrix(
        (1.0, 0.0, 0.0, 0.0),
        (0.0, cos(r), -sin(r), 0.0),
        (0.0, sin(r), cos(r), 0.0),
        (0.0, 0.0, 0.0, 1.0),
    )
}

/// Create a matrix specifying a clockwise rotation around the y axis.
/// The argument is specified in radians.
pub fn rotation_y(r: f64) -> Matrix {
    matrix(
        ( cos(r), 0.0, sin(r), 0.0),
        (    0.0, 1.0,    0.0, 0.0),
        (-sin(r), 0.0, cos(r), 0.0),
        (    0.0, 0.0,    0.0, 1.0),
    )
}

/// Create a matrix specifying a clockwise rotation around the z axis.
/// The argument is specified in radians.
pub fn rotation_z(r: f64) -> Matrix {
    matrix(
        (cos(r), -sin(r), 0.0, 0.0),
        (sin(r),  cos(r), 0.0, 0.0),
        (   0.0,     0.0, 1.0, 0.0),
        (   0.0,     0.0, 0.0, 1.0),
    )
}

pub fn view_transform(from: Tuple4, to: Tuple4, up: Tuple4) -> Matrix {
    let forward = (to - from).normalize();
    let left = forward.cross(up.normalize());
    let trueup = left.cross(forward);
    let m = matrix(
        (    left.x(),     left.y(),     left.z(), 0.0),
        (  trueup.x(),   trueup.y(),   trueup.z(), 0.0),
        (-forward.x(), -forward.y(), -forward.z(), 0.0),
        (         0.0,          0.0,          0.0, 1.0),
    );
    m * translation(-from.x(), -from.y(), -from.z())
}

fn sin(r: f64) -> f64 {
    r.sin()
}

fn cos(r: f64) -> f64 {
    r.cos()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn determinant() {
        let a = matrix2((1.0, 5.0), (-3.0, 2.0));
        assert_eq!(17.0, a.det());
    }

    #[test]
    fn submatrix_of_3x3() {
        let a = matrix3((1.0, 5.0, 0.0), (-3.0, 2.0, 7.0), (0.0, 6.0, -3.0));

        assert_eq!(a.submatrix(0, 2), matrix2((-3.0, 2.0), (0.0, 6.0)));
    }

    #[test]
    fn submatrix_of_4x4() {
        let a = matrix(
            (-6.0, 1.0, 1.0, 6.0),
            (-8.0, 5.0, 8.0, 6.0),
            (-1.0, 0.0, 8.0, 2.0),
            (-7.0, 1.0, -1.0, 1.0),
        );

        assert_eq!(
            a.submatrix(2, 1),
            matrix3(
                (-6.0, 1.0, 6.0),
                (-8.0, 8.0, 6.0),
                (-7.0, -1.0, 1.0)
            )
        );
    }

    #[test]
    fn minor_of_3x3() {
        let a = matrix3(
            (3.0, 5.0, 0.0),
            (2.0, -1.0, -7.0),
            (6.0, -1.0, 5.0)
        );
        let b = a.submatrix(1, 0);

        assert_eq!(25.0, b.det());
        assert_eq!(25.0, a.minor(1, 0));
    }

    #[test]
    fn cofactors_of_3x3() {
        let a = matrix3(
            (3.0, 5.0, 0.0),
            (2.0, -1.0, -7.0),
            (6.0, -1.0, 5.0)
        );
        assert_eq!(a.minor(0, 0), -12.0);
        assert_eq!(a.cofactor(0, 0), -12.0);
        assert_eq!(a.minor(1, 0), 25.0);
        assert_eq!(a.cofactor(1, 0), -25.0);
    }

    #[test]
    fn determinant_of_3x3() {
        let a = matrix3(
            (1.0, 2.0, 6.0),
            (-5.0, 8.0, -4.0),
            (2.0, 6.0, 4.0));
        assert_eq!(a.cofactor(0, 0), 56.0);
        assert_eq!(a.cofactor(0, 1), 12.0);
        assert_eq!(a.cofactor(0, 2), -46.0);
        assert_eq!(a.det(), -196.0);
    }

    #[test]
    fn determinant_of_4x4() {
        let a = matrix(
            (-2.0, -8.0, 3.0, 5.0),
            (-3.0, 1.0, 7.0, 3.0),
            (1.0, 2.0, -9.0, 6.0),
            (-6.0, 7.0, 7.0, -9.0),
        );
        assert_eq!(a.cofactor(0, 0), 690.0);
        assert_eq!(a.cofactor(0, 1), 447.0);
        assert_eq!(a.cofactor(0, 2), 210.0);
        assert_eq!(a.cofactor(0, 3), 51.0);
        assert_eq!(a.det(), -4071.0);
    }

}
