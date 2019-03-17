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
