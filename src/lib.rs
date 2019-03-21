use std::ops::*;
use std::vec::*;
use std::io::Result as IOResult;
use std::io::Write;

const EPSILON: f64 = 1e-5;

#[derive(Debug, Copy, Clone)]
pub struct Tuple4(f64, f64, f64, f64);

pub fn is_point(Tuple4(_, _, _, w): Tuple4) -> bool {
    w == 1.0
}

pub fn is_vector(v: Tuple4) -> bool {
    !is_point(v)
}

pub fn point(x: f64, y: f64, z: f64) -> Tuple4 {
    Tuple4(x, y, z, 1.0)
}

pub fn vector(x: f64, y: f64, z: f64) -> Tuple4 {
    Tuple4(x, y, z, 0.0)
}

pub fn tuple(x: f64, y: f64, z: f64, w: f64) -> Tuple4 {
    Tuple4(x, y, z, w)
}

impl PartialEq for Tuple4 {

    fn eq(&self, Tuple4(b1, b2, b3, b4): &Tuple4) -> bool {
        match self {
            Tuple4(a1, a2, a3, a4) => {
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

impl Add for Tuple4 {
    type Output = Tuple4;

    fn add(self, Tuple4(b1, b2, b3, b4): Tuple4) -> Tuple4 {
        match self {
            Tuple4(a1, a2, a3, a4) =>
                Tuple4(a1 + b1, a2 + b2, a3 + b3, a4 + b4)
        }
    }
}

impl Sub for Tuple4 {
    type Output = Tuple4;

    fn sub(self, Tuple4(b1, b2, b3, b4): Tuple4) -> Tuple4 {
        match self {
            Tuple4(a1, a2, a3, a4) =>
                Tuple4(a1 - b1, a2 - b2, a3 - b3, a4 - b4)
        }
    }
}

impl Neg for Tuple4 {
    type Output = Tuple4;

    fn neg(self) -> Tuple4 {
        match self {
            Tuple4(x, y, z, w) =>
                Tuple4(-x, -y, -z, -w)
        }
    }
}

impl Tuple4 {
    pub fn x(&self) -> f64 {
        match *self {
            Tuple4(x, y, z, w) => x
        }
    }

    pub fn y(&self) -> f64 {
        match *self {
            Tuple4(x, y, z, w) => y
        }
    }

    pub fn z(&self) -> f64 {
        match *self {
            Tuple4(x, y, z, w) => z
        }
    }

    pub fn w(&self) -> f64 {
        match *self {
            Tuple4(x, y, z, w) => w
        }
    }

    pub fn scale(self, c: f64) -> Tuple4 {
        match self {
            Tuple4(x, y, z, w) =>
                Tuple4(c*x, c*y, c*z, c*w)
        }
    }

    pub fn magnitude(self) -> f64 {
        match self {
            Tuple4(x, y, z, w) =>
                f64::sqrt(x.powi(2) + y.powi(2) + z.powi(2) + w.powi(2))
        }
    }

    pub fn normalize(self) -> Tuple4 {
        match self {
            Tuple4(x, y, z, w) => {
                let c = self.magnitude();
                Tuple4(x/c, y/c, z/c, w/c)
            }
        }
    }

    pub fn dot(self, Tuple4(x2, y2, z2, w2): Tuple4) -> f64 {
        match self {
            Tuple4(x1, y1, z1, w1) => {
                x1 * x2 +
                y1 * y2 +
                z1 * z2 +
                w1 * w2
            }
        }
    }

    pub fn cross(self, Tuple4(b1, b2, b3, b4): Tuple4) -> Tuple4 {
        match self {
            Tuple4(a1, a2, a3, a4) => {
                vector(a2 * b3 - a3 * b2,
                       a3 * b1 - a1 * b3,
                       a1 * b2 - a2 * b1)
            }
        }
    }

    // hadamard product
    pub fn mult_pairwise(self, Tuple4(b1, b2, b3, b4): Tuple4) -> Tuple4 {
        match self {
            Tuple4(a1, a2, a3, a4) => {
                Tuple4(a1 * b1,
                       a2 * b2,
                       a3 * b3,
                       a4 * b4)
            }
        }
    }
}

impl Index<usize> for Tuple4 {
    type Output = f64;

    fn index(&self, index: usize) -> &f64 {
        match self {
            Tuple4(x, y, z, w) => {
                match index {
                    0 => &x,
                    1 => &y,
                    2 => &z,
                    3 => &w,
                    _ => panic!("unexpected index")
                }
            }
        }
    }

}

pub fn colour(r: f64, g: f64, b: f64) -> Tuple4 {
    Tuple4(r, g, b, 0.0)
}

pub fn red(Tuple4(r, g, b, w): Tuple4) -> f64 {
    r
}
pub fn green(Tuple4(r, g, b, w): Tuple4) -> f64 {
    g
}
pub fn blue(Tuple4(r, g, b, w): Tuple4) -> f64 {
    b
}

pub struct Canvas { 
    pub width: usize, 
    pub height: usize,
    pixels: Vec<Tuple4>
}

pub fn canvas(w: usize, h: usize) -> Canvas {
    let length = w * h;
    let arr = vec![colour(0.0, 0.0, 0.0); length];
    Canvas {width: w, height: h, pixels: arr}
}

impl Canvas {
    pub fn colour_at(&self, x: usize, y: usize) -> Tuple4 {
        self.pixels[y * self.width + x]
    }
    pub fn set_colour_at(&mut self, x: usize, y: usize, c: Tuple4) {
        self.pixels[y * self.width + x] = c;
    }
}

pub fn encode_ppm(c: &Canvas, w: &mut Write) -> IOResult<()> {
    writeln!(w, "P3")?;
    writeln!(w, "{} {}", c.width, c.height)?;
    writeln!(w, "255")?;
    encode_ppm_pixels(c, w, 70)?;
    Ok(())
}

fn encode_ppm_pixels(c: &Canvas, w: &mut Write, line_width: usize) -> IOResult<()> {
    for row in 0..(c.height) {
        let mut char_width = 0;
        for col in 0..(c.width) {
            let p = c.colour_at(col, row);
            let s = format!("{:.0} {:.0} {:.0} ", 
                            clamp(red(p), 255),
                            clamp(green(p), 255),
                            clamp(blue(p), 255));

            if char_width + s.len() > line_width {
                writeln!(w)?;
                char_width = 0;
            }
            write!(w, "{}", s)?;
            char_width += s.len();
        }
        writeln!(w, "")?
    }
    Ok(())
}

fn clamp(p: f64, max: u32) -> f64 {
    if p < 0.0 {
        return 0.0;
    } else if p >= 1.0 {
        return f64::from(max);
    } else {
        return p * f64::from(max);
    }
}

#[derive(Debug)]
pub struct Matrix {
    r1: Tuple4,
    r2: Tuple4,
    r3: Tuple4,
    r4: Tuple4
}

pub fn matrix(
    (x1, y1, z1, w1) : (f64, f64, f64, f64),
    (x2, y2, z2, w2) : (f64, f64, f64, f64),
    (x3, y3, z3, w3) : (f64, f64, f64, f64),
    (x4, y4, z4, w4) : (f64, f64, f64, f64)
    ) -> Matrix {

    Matrix {
        r1: Tuple4(x1, y1, z1, w1),
        r2: Tuple4(x2, y2, z2, w2),
        r3: Tuple4(x3, y3, z3, w3),
        r4: Tuple4(x4, y4, z4, w4),
    }
}

impl Index<usize> for Matrix {
    type Output = Tuple4;
    
    fn index(&self, index: usize) -> &Tuple4 {
        match index {
            0 => &self.r1,
            1 => &self.r2,
            2 => &self.r3,
            3 => &self.r4,
            _ => panic!("unexpected index for matrix")
        }
    }
}

impl PartialEq for Matrix {
    fn eq(&self, y: &Matrix) -> bool {
        self.r1 == y.r1 &&
        self.r2 == y.r2 &&
        self.r3 == y.r3 &&
        self.r4 == y.r4
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
        (a4b1, a4b2, a4b3, a4b4))
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
        Tuple4(self[0][i], 
               self[1][i], 
               self[2][i], 
               self[3][i])
    }
}
