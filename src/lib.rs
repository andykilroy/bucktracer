use std::ops::*;
use std::vec::*;
use std::io::Result as IOResult;
use std::io::Write;
use std::str::FromStr;


const EPSILON: f64 = 1e-5;

#[derive(Debug, Copy, Clone)]
pub struct Tuple4(f64, f64, f64, f64);


#[allow(clippy::float_cmp)]
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
        let Tuple4(a1, a2, a3, a4) = self;
        almost_eq(*a1, *b1) &&
        almost_eq(*a2, *b2) &&
        almost_eq(*a3, *b3) &&
        almost_eq(*a4, *b4)
    }
}

fn almost_eq(x1: f64, x2: f64) -> bool {
    f64::abs(x1 - x2) < EPSILON
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
        match *self {
            Tuple4(x, _, _, _) => x
        }
    }

    pub fn y(&self) -> f64 {
        match *self {
            Tuple4(_, y, _, _) => y
        }
    }

    pub fn z(&self) -> f64 {
        match *self {
            Tuple4(_, _, z, _) => z
        }
    }

    pub fn w(&self) -> f64 {
        match *self {
            Tuple4(_, _, _, w) => w
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

    pub fn cross(self, Tuple4(b1, b2, b3, _b4): Tuple4) -> Tuple4 {
        match self {
            Tuple4(a1, a2, a3, _a4) => {
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

pub fn red(Tuple4(r, _g, _b, _w): Tuple4) -> f64 {
    r
}
pub fn green(Tuple4(_r, g, _b, _w): Tuple4) -> f64 {
    g
}
pub fn blue(Tuple4(_r, _g, b, _w): Tuple4) -> f64 {
    b
}

pub fn white() -> Tuple4 {
    colour(1.0, 1.0, 1.0)
}

#[derive(Debug)]
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
        writeln!(w)?
    }
    Ok(())
}

fn clamp(p: f64, max: u32) -> f64 {
    if p < 0.0 {
        0.0
    } else if p >= 1.0 {
        f64::from(max)
    } else {
        p * f64::from(max)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Matrix {
    dim: Dimensions,
    r1: Tuple4,
    r2: Tuple4,
    r3: Tuple4,
    r4: Tuple4
}

#[derive(Debug, Copy, Clone)]
enum Dimensions {
    X2,
    X3,
    X4
}

pub fn matrix(
    (x1, y1, z1, w1) : (f64, f64, f64, f64),
    (x2, y2, z2, w2) : (f64, f64, f64, f64),
    (x3, y3, z3, w3) : (f64, f64, f64, f64),
    (x4, y4, z4, w4) : (f64, f64, f64, f64)
    ) -> Matrix {

    Matrix {
        dim: Dimensions::X4,
        r1: Tuple4(x1, y1, z1, w1),
        r2: Tuple4(x2, y2, z2, w2),
        r3: Tuple4(x3, y3, z3, w3),
        r4: Tuple4(x4, y4, z4, w4),
    }
}


pub fn matrix2(
    (x1, y1) : (f64, f64),
    (x2, y2) : (f64, f64),
    ) -> Matrix {

    Matrix {
        dim: Dimensions::X2,
        r1: Tuple4(x1, y1, 0.0, 0.0),
        r2: Tuple4(x2, y2, 0.0, 0.0),
        r3: Tuple4(0.0, 0.0, 0.0, 0.0),
        r4: Tuple4(0.0, 0.0, 0.0, 0.0),
    }
}

pub fn matrix3(
    (x1, y1, z1) : (f64, f64, f64),
    (x2, y2, z2) : (f64, f64, f64),
    (x3, y3, z3) : (f64, f64, f64),
    ) -> Matrix {

    Matrix {
        dim: Dimensions::X3,
        r1: Tuple4(x1, y1, z1, 0.0),
        r2: Tuple4(x2, y2, z2, 0.0),
        r3: Tuple4(x3, y3, z3, 0.0),
        r4: Tuple4(0.0, 0.0, 0.0, 0.0),
    }
}
pub fn identity() -> Matrix {
    matrix((1.0, 0.0, 0.0, 0.0),
           (0.0, 1.0, 0.0, 0.0),
           (0.0, 0.0, 1.0, 0.0),
           (0.0, 0.0, 0.0, 1.0))
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

    pub fn transpose(&self) -> Matrix {
        matrix((self[0][0], self[1][0], self[2][0], self[3][0]),
               (self[0][1], self[1][1], self[2][1], self[3][1]),
               (self[0][2], self[1][2], self[2][2], self[3][2]),
               (self[0][3], self[1][3], self[2][3], self[3][3]))
    }

    pub fn det(&self) -> f64 {
        match self.dim {
            Dimensions::X2 => det_x2(self),
            Dimensions::X3 => det_x3(self),
            Dimensions::X4 => det_x4(self)
        }
    }

    pub fn submatrix(&self, row: usize, col: usize) -> Matrix {
        match self.dim {
            Dimensions::X3 => submatr_x3(self, row, col),
            Dimensions::X4 => submatr_x4(self, row, col),
            _ => panic!("no expected submatrix of a 2x2 matrix")
        }
    }

    pub fn minor(&self, row: usize, col: usize) -> f64 {
        self.submatrix(row, col).det()
    }

    pub fn cofactor(&self, row: usize, col: usize) -> f64 {
        if (row + col) % 2 ==  0 {
            self.minor(row, col)
        } else {
            - self.minor(row, col)
        }
    }

    pub fn cofactors(&self) -> Matrix {
        matrix((self.cofactor(0, 0), self.cofactor(0, 1), self.cofactor(0, 2), self.cofactor(0, 3)),
               (self.cofactor(1, 0), self.cofactor(1, 1), self.cofactor(1, 2), self.cofactor(1, 3)),
               (self.cofactor(2, 0), self.cofactor(2, 1), self.cofactor(2, 2), self.cofactor(2, 3)),
               (self.cofactor(3, 0), self.cofactor(3, 1), self.cofactor(3, 2), self.cofactor(3, 3)))
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
        self.cofactors().transpose().scale_elems(1.0/self.det())
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


fn det_x2(x: &Matrix) -> f64 {
    let a = x[0][0];
    let b = x[0][1];
    let c = x[1][0];
    let d = x[1][1];
    (a * d) - (b * c)
}

fn det_x3(x: &Matrix) -> f64 {
    x[0][0] * x.cofactor(0, 0) +
    x[0][1] * x.cofactor(0, 1) +
    x[0][2] * x.cofactor(0, 2)
}

fn det_x4(x: &Matrix) -> f64 {
    x[0][0] * x.cofactor(0, 0) +
    x[0][1] * x.cofactor(0, 1) +
    x[0][2] * x.cofactor(0, 2) +
    x[0][3] * x.cofactor(0, 3)
}

fn submatr_x3(x: &Matrix, row: usize, col: usize) -> Matrix {
    if row >= 3 {
        panic!("row for a 3x3 matrix expected to be 0, 1, or 2")
    }
    if col >= 3 {
        panic!("col for a 3x3 matrix expected to be 0, 1, or 2")
    }

    let mut collected = vec![];
    for r in 0..3 {
        for c in 0..3 {
            if r != row && c != col {
                collected.push(x[r][c]);
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

    let mut collected = vec![];
    for r in 0..4 {
        for c in 0..4 {
            if r != row && c != col {
                collected.push(x[r][c]);
            }
        }
    }

    matrix3(
        (collected[0], collected[1], collected[2]), 
        (collected[3], collected[4], collected[5]), 
        (collected[6], collected[7], collected[8]) 
        )
}


pub fn translation(x: f64, y: f64, z: f64) -> Matrix {
    matrix((1.0, 0.0, 0.0, x),
           (0.0, 1.0, 0.0, y),
           (0.0, 0.0, 1.0, z),
           (0.0, 0.0, 0.0, 1.0))
}

pub fn scaling(x: f64, y: f64, z: f64) -> Matrix {
    matrix((x  , 0.0, 0.0, 0.0),
           (0.0, y  , 0.0, 0.0),
           (0.0, 0.0, z  , 0.0),
           (0.0, 0.0, 0.0, 1.0))
}

pub fn rotation_x(r: f64) -> Matrix {
    matrix((1.0,    0.0,     0.0,     0.0),
           (0.0, cos(r), -sin(r),     0.0),
           (0.0, sin(r),  cos(r),     0.0),
           (0.0,    0.0,     0.0,     1.0))
}

pub fn rotation_y(r: f64) -> Matrix {
    matrix(( cos(r),    0.0,  sin(r),     0.0),
           (    0.0,    1.0,     0.0,     0.0),
           (-sin(r),    0.0,  cos(r),     0.0),
           (    0.0,    0.0,     0.0,     1.0))
}

pub fn rotation_z(r: f64) -> Matrix {
    matrix((cos(r), -sin(r),     0.0,     0.0),
           (sin(r),  cos(r),     0.0,     0.0),
           (   0.0,     0.0,     1.0,     0.0),
           (   0.0,     0.0,     0.0,     1.0))
}

// I don't like the call style of the trig functions in f64

fn sin(r: f64) -> f64 {
    r.sin()
}

fn cos(r: f64) -> f64 {
    r.cos()
}

#[derive(Debug, Clone)]
pub struct Ray {
    pub origin: Tuple4,
    pub direction: Tuple4
}

pub fn ray(o: Tuple4, d: Tuple4) -> Ray {
    Ray { origin: o, direction: d }
}

pub fn position(ray: Ray, t: f64) -> Tuple4 {
    ray.origin + (ray.direction.scale(t))
}

pub fn transform(r: &Ray, m: &Matrix) -> Ray {
    ray(m.mult(r.origin), m.mult(r.direction))
}


#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Sphere {
    pos: Tuple4,
    radius: f64,
    transform: Matrix,
    material: Material,
}

pub fn unit_sphere() -> Sphere {
    Sphere {
        pos: point(0.0, 0.0, 0.0),
        radius: 1.0,
        transform: identity(),
        material: Material::default(),
    }
}

impl Sphere {
    pub fn transform(&self) -> Matrix {
        self.transform
    }

    pub fn set_transform(&mut self, m: &Matrix) {
        self.transform = m.clone();
    }

    pub fn material(&self) -> Material {
        self.material
    }

    pub fn set_material(&mut self, m: &Material) {
        self.material = m.clone();
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Intersection {
    pub t_value: f64,
    pub intersected: Sphere
}

pub fn intersection(t: f64, s: &Sphere) -> Intersection {
    Intersection {
        t_value: t,
        intersected: s.clone()
    }
}

pub fn intersect(orig: &Ray, s: &Sphere) -> Vec<Intersection> {
    let inverted = s.transform().inverse();
    let r = transform(orig, &inverted);
    intersect_(&r, s)
}

fn intersect_(r: &Ray, s: &Sphere) -> Vec<Intersection> {
    // presume the sphere is centred at (0,0,0)
    let s_to_ray = r.origin - point(0.0, 0.0, 0.0);
    let a = r.direction.dot(r.direction);
    let b = 2.0 * r.direction.dot(s_to_ray);
    let c = s_to_ray.dot(s_to_ray) - 1.0;
    let discriminant = b.powf(2.0) - (4.0 * a * c);

    if discriminant < 0.0 {
        vec![]
    } else {
        let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b + discriminant.sqrt()) / (2.0 * a);
        vec![
            intersection(t1, s),
            intersection(t2, s)
        ]
    }
}

pub fn hit(intersects: Vec<Intersection>) -> Option<Intersection> {
    intersects
        .iter()
        .filter(|i| i.t_value >= 0.0)
        .fold(None, |least, x| {
            nearer_intersect(least, x)
        })
        .cloned()
}

fn nearer_intersect<'a>(nearest: Option<&'a Intersection>, x: &'a Intersection) -> Option<&'a Intersection> {
    match nearest {
        None => Some(x),
        Some(c) => {
            if x.t_value < c.t_value {
                Some(x)
            } else {
                nearest
            }
        }
    }
}

// ---- Camera related stuff
pub type Coord = (usize, usize);

#[derive(Debug)]
pub struct RadialLightSource {
    position: Tuple4,
    intensity: Tuple4, // a colour
}

pub fn point_light(p: Tuple4, i: Tuple4) -> RadialLightSource {
    RadialLightSource {
        position: p,
        intensity: i,
    }
}

impl RadialLightSource {
    pub fn position(self: &Self) -> Tuple4 {
        self.position
    }

    pub fn intensity(self: &Self) -> Tuple4 {
        self.intensity
    }
}

pub fn ray_to_point(origin: Tuple4, point: Tuple4) -> Ray {
    ray(origin, point.sub(origin))
}

fn to_f64(v: usize) -> f64 {
    let s = format!("{}", v);
    return f64::from_str(&s).unwrap();
}

#[derive(Debug)]
pub struct Camera {
    canvas: Canvas,
    plane: BoundedPlane,
}

impl Camera {
    pub fn new(canv: Canvas, pln: BoundedPlane) -> Camera {
        Camera {
            canvas: canv,
            plane: pln,
        }
    }

    pub fn canvas(self: &Self) -> &Canvas {
        &self.canvas
    }

    pub fn paint_colour_at(self: &mut Self, x: usize, y: usize, c: Tuple4) {
        self.canvas.set_colour_at(x, y, c);
    }

    pub fn pixel_to_point(self: &Self, x: usize, y: usize) -> Tuple4 {
        assert!(x < self.canvas.width);
        assert!(y < self.canvas.height);

        let world_width = self.plane.upper_right.sub(self.plane.lower_left);
        let x_factor : f64 = world_width.x() / to_f64(self.canvas.width);
        let y_factor : f64 = world_width.y() / to_f64(self.canvas.height);
        let x_float = to_f64(x);
        let y_float = to_f64(y);

        point(x_float * x_factor, y_float * y_factor, 0.0)
    }
}

#[derive(Debug)]
pub struct BoundedPlane {
    lower_left: Tuple4,
    upper_right: Tuple4,
    surface_normal: Tuple4,
}

type Triple = (f64, f64, f64);

pub fn camera(c: Canvas, l_left: Triple, u_right: Triple, normal: Triple) -> Camera {
    Camera::new(
        c,
        BoundedPlane {
            lower_left: point(l_left.0, l_left.1, l_left.2),
            upper_right: point(u_right.0, u_right.1, u_right.2),
            surface_normal: vector(normal.0, normal.1, normal.2),
        },
    )
}

// ----

pub fn normal_at(s: &Sphere, world_point: Tuple4) -> Tuple4 {
    let inversion_mat = s.transform.inverse();
    let object_point = inversion_mat.mult(world_point);
    let object_normal = object_point.sub(s.pos);
    let tmp = inversion_mat.transpose().mult(object_normal);

    tuple(tmp.x(), tmp.y(), tmp.z(), 0.0).normalize()
}

pub fn reflect(v: Tuple4, norm: Tuple4) -> Tuple4 {
    v - norm.scale(2.0).scale(v.dot(norm))
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Material {
    colour: Tuple4,
    ambient: f64,
    diffuse: f64,
    specular: f64,
    shininess: f64,
}

impl Material {
    pub fn default() -> Material {
        Material {
            colour: white(),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
        }
    }

    pub fn colour(self: &Self) -> Tuple4 {
        self.colour
    }
    pub fn set_colour(self: &mut Self, x: Tuple4) -> &mut Self {
        self.colour = x;
        self
    }

    pub fn ambient(self: &Self) -> f64 {
        self.ambient
    }
    pub fn set_ambient(self: &mut Self, x: f64) -> &mut Self {
        self.ambient = x;
        self
    }

    pub fn diffuse(self: &Self) -> f64 {
        self.diffuse
    }
    pub fn set_diffuse(self: &mut Self, x: f64) -> &mut Self {
        self.diffuse = x;
        self
    }

    pub fn specular(self: &Self) -> f64 {
        self.specular
    }
    pub fn set_specular(self: &mut Self, x: f64) -> &mut Self {
        self.specular = x;
        self
    }

    pub fn shininess(self: &Self) -> f64 {
        self.shininess
    }
    pub fn set_shininess(self: &mut Self, x: f64) -> &mut Self {
        self.shininess = x;
        self
    }

}

pub fn lighting(
    light: &RadialLightSource,
    pos: Tuple4,
    normalv: Tuple4,
    mat: &Material,
    eyev: Tuple4,
) -> Tuple4 {
    let effective_colour = mat.colour().mult_pairwise(light.intensity());
    let lightv = (light.position() - pos).normalize();
    let ambient = effective_colour.scale(mat.ambient());
    let light_dot_normal = lightv.dot(normalv);

    let black = colour(0.0, 0.0, 0.0);
    let (diffuse, specular) = if light_dot_normal < 0.0 {
        // the light is behind the surface
        (black, black)
    } else {
        let d = effective_colour.scale(mat.diffuse * light_dot_normal);
        let reflectv = reflect(-lightv, normalv);
        let reflect_dot_eye = reflectv.dot(eyev);

        if reflect_dot_eye <= 0.0 {
            (d, black)
        } else {
            let factor = reflect_dot_eye.powf(mat.shininess());
            let s = light.intensity().scale(mat.specular() * factor);
            (d, s)
        }
    };

    ambient + diffuse + specular
}


mod test {
    use crate::*;

    #[test]
    fn rays_to_lightsrc() {
        let orig1 = point(0.0, 0.0, 0.0);
        let orig2 = point(3.0, 3.0, 0.0);
        let target1 = point(0.0, 0.0, 1.0);
        let target2 = point(5.0, 6.0, 7.0);

        assert_eq!(ray_to_point(orig1, target1).origin.x(), 0.0);
        assert_eq!(ray_to_point(orig1, target1).origin.y(), 0.0);
        assert_eq!(ray_to_point(orig1, target1).origin.z(), 0.0);

        assert_eq!(ray_to_point(orig1, target1).direction.x(), 0.0);
        assert_eq!(ray_to_point(orig1, target1).direction.y(), 0.0);
        assert_eq!(ray_to_point(orig1, target1).direction.z(), 1.0);

        assert_eq!(ray_to_point(orig2, target2).origin.x(), 3.0);
        assert_eq!(ray_to_point(orig2, target2).origin.y(), 3.0);
        assert_eq!(ray_to_point(orig2, target2).origin.z(), 0.0);

        assert_eq!(ray_to_point(orig2, target2).direction.x(), 2.0);
        assert_eq!(ray_to_point(orig2, target2).direction.y(), 3.0);
        assert_eq!(ray_to_point(orig2, target2).direction.z(), 7.0);
    }

    #[test]
    fn pixel_index_to_point_in_space_xs() {
        let mut cam = camera(
            canvas(300, 300),
            (0.0, 0.0, 0.0),
            (60.0, 60.0, 0.0),
            (0.0, 0.0, 1.0),
        );

        assert_eq!(cam.pixel_to_point(0, 0), point(0.0, 0.0, 0.0));
        assert_eq!(cam.pixel_to_point(1, 0), point(0.2, 0.0, 0.0));
        assert_eq!(cam.pixel_to_point(2, 0), point(0.4, 0.0, 0.0));
        assert_eq!(cam.pixel_to_point(3, 0), point(0.6, 0.0, 0.0));
        assert_eq!(cam.pixel_to_point(4, 0), point(0.8, 0.0, 0.0));
        assert_eq!(cam.pixel_to_point(5, 0), point(1.0, 0.0, 0.0));
        assert_eq!(cam.pixel_to_point(299, 0), point(59.8, 0.0, 0.0));
    }

    #[test]
    fn pixel_index_to_point_in_space_ys() {
        let mut cam = camera(
            canvas(300, 300),
            (0.0, 0.0, 0.0),
            (60.0, 60.0, 0.0),
            (0.0, 0.0, 1.0),
        );

        assert_eq!(cam.pixel_to_point(0, 0), point(0.0, 0.0, 0.0));
        assert_eq!(cam.pixel_to_point(0, 1), point(0.0, 0.2, 0.0));
        assert_eq!(cam.pixel_to_point(0, 2), point(0.0, 0.4, 0.0));
        assert_eq!(cam.pixel_to_point(0, 3), point(0.0, 0.6, 0.0));
        assert_eq!(cam.pixel_to_point(0, 4), point(0.0, 0.8, 0.0));
        assert_eq!(cam.pixel_to_point(0, 5), point(0.0, 1.0, 0.0));
        assert_eq!(cam.pixel_to_point(0, 299), point(0.0, 59.8, 0.0));
    }

    #[test]
    fn pixel_index_to_point_in_space_zs() {
        let mut cam = camera(
            canvas(300, 300),
            (0.0, 0.0, 0.0),
            (60.0, 60.0, 0.0),
            (0.0, 0.0, 1.0),
        );

        assert_eq!(cam.pixel_to_point(0, 0), point(0.0, 0.0, 0.0));
        assert_eq!(cam.pixel_to_point(0, 1), point(0.0, 0.2, 0.0));
        assert_eq!(cam.pixel_to_point(0, 2), point(0.0, 0.4, 0.0));
        assert_eq!(cam.pixel_to_point(0, 3), point(0.0, 0.6, 0.0));
        assert_eq!(cam.pixel_to_point(0, 4), point(0.0, 0.8, 0.0));
        assert_eq!(cam.pixel_to_point(0, 5), point(0.0, 1.0, 0.0));
        assert_eq!(cam.pixel_to_point(0, 299), point(0.0, 59.8, 0.0));
    }

    #[test]
    fn test_ray_trace() {
        let mut s = unit_sphere();
        s.set_transform(&(translation(30.0, 30.0, 20.0) * scaling(7.5, 7.5, 7.5)));
        let r = ray(point(30.0, 30.0, 0.0), vector(0.0, 0.0, 1.0));
        let intersects = intersect(&r, &s);
        assert_eq!(2, intersects.len());
    }
}
