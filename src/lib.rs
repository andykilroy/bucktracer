use std::vec;
use std::io::Result as IOResult;
use std::io::Write;
use std::cmp::Ordering;

use serde::Deserialize;

mod math;

pub use crate::math::*;
use std::ops::Add;

const EPSILON: f64 = 1e-5;

/// A structure representing a colour in red, green, and blue components.
#[derive(Debug, Copy, Clone, PartialEq)]
#[derive(Deserialize)]
#[serde(from = "(f64, f64, f64)")]
pub struct RGB {
    inner: Tuple4
}

pub fn colour(red: f64, green: f64, blue: f64) -> RGB {
    RGB {inner: tuple(red, green, blue, 0.0)}
}

impl RGB {
    pub fn white() -> RGB {
        colour(1.0, 1.0, 1.0)
    }
    pub fn black() -> RGB {
        colour(0.0, 0.0, 0.0)
    }

    pub fn red(self: &Self) -> f64 {
        self.inner.x()
    }
    pub fn green(self: &Self) -> f64 {
        self.inner.y()
    }
    pub fn blue(self: &Self) -> f64 {
        self.inner.z()
    }
}

impl From<Tuple4> for RGB {
    fn from(x: Tuple4) -> Self {
        RGB { inner: x }
    }
}

impl Into<Tuple4> for RGB {
    fn into(self) -> Tuple4 {
        self.inner
    }
}

impl From<(f64, f64, f64)> for RGB {
    fn from((r, g, b): (f64, f64, f64)) -> Self {
        colour(r, g, b)
    }
}

impl Add for RGB {
    type Output = RGB;

    fn add(self, rhs: Self) -> Self::Output {
        RGB{ inner: (self.inner + rhs.inner) }
    }
}


/// A structure used to record pixel colour values
/// indexed by 2D coordinates.  (0,0) represents the
/// top-left pixel of the canvas.
#[derive(Debug)]
pub struct Canvas { 
    pub width: usize, 
    pub height: usize,
    pixels: Vec<RGB>
}

pub fn canvas(w: usize, h: usize) -> Canvas {
    let length = w * h;
    let arr = vec![colour(0.0, 0.0, 0.0); length];
    Canvas {width: w, height: h, pixels: arr}
}

impl Canvas {
    pub fn colour_at(&self, x: usize, y: usize) -> RGB {
        self.pixels[y * self.width + x]
    }
    pub fn set_colour_at(&mut self, x: usize, y: usize, c: RGB) {
        self.pixels[y * self.width + x] = c;
    }
}

pub fn encode_ppm(c: &Canvas, w: &mut dyn Write) -> IOResult<()> {
    writeln!(w, "P3")?;
    writeln!(w, "{} {}", c.width, c.height)?;
    writeln!(w, "255")?;
    encode_ppm_pixels(c, w, 70)?;
    Ok(())
}

fn encode_ppm_pixels(c: &Canvas, w: &mut dyn Write, line_width: usize) -> IOResult<()> {
    for row in 0..(c.height) {
        let mut char_width = 0;
        for col in 0..(c.width) {
            let p = c.colour_at(col, row);
            let s = format!("{:.0} {:.0} {:.0} ", 
                            clamp(p.red(), 255),
                            clamp(p.green(), 255),
                            clamp(p.blue(), 255));

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

/// Represents a ray fired from a point in a particular
/// direction
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


pub fn unit_sphere() -> Object {
    Object {
        world_to_object_spc: identity(),
        material: Material::default(),
        shape: Shape::Sphere,
    }
}

pub fn plane() -> Object {
    Object {
        world_to_object_spc: identity(),
        material: Material::default(),
        shape: Shape::Plane,
    }
}

/// Determines what shape an object has.
///
/// Influences the calculation of surface normals and intersections.
#[derive(Debug, Copy, Clone, PartialEq, Deserialize)]
pub enum Shape {
    Sphere,
    Plane,
}

impl Shape {
    /// Calculates a normal appropriate for the object at the
    /// specified position.  The position is always in object
    /// co-ordinates.  The returned normal vector is also in
    /// object space.
    fn local_normal_at(self: &Self, position: Tuple4) -> Tuple4 {
        match *self {
            Shape::Sphere => {
                // presume the sphere is centred at (0, 0, 0)
                position - point(0.0, 0.0, 0.0)
            },
            Shape::Plane => {
                vector(0.0, 1.0, 0.0)
            }
        }
    }
}
/// An object to be placed in the world.
///
/// The object has a transform that dictates where it is placed in
/// the world, and also whether it is scaled or rotated in any way.
/// It also is associated with material dictating its reflective
/// properties.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Object {
    world_to_object_spc: Matrix,
    material: Material,
    shape: Shape
}

impl Object {
    /// The transformation matrix to convert co-ordinates from
    /// object space to world space.
    pub fn object_to_world_spc(&self) -> Matrix {
        self.world_to_object_spc.inverse()
    }

    pub fn set_object_to_world_spc(self: &mut Self, m: Matrix) -> &mut Self {
        // The given matrix describes the object -> world coordinate transform.
        // The transformation from world -> object is performed more frequently.
        // In the interests of performance, store the world -> object transform,
        // so the inverse does not have to be computed all the time.
        self.world_to_object_spc = m.inverse();
        self
    }

    /// The transformation matrix to convert co-ordinates from
    /// world space to object space.
    pub fn world_to_object_spc(&self) -> Matrix {
        self.world_to_object_spc
    }

    pub fn material(&self) -> Material {
        self.material
    }

    pub fn set_material(self: &mut Self, m: Material) -> &mut Self {
        self.material = m;
        self
    }

    pub fn normal_at(self: &Self, world_point: Tuple4) -> Tuple4 {
        let inversion_mat = self.world_to_object_spc();
        let object_point = inversion_mat.mult(world_point);
        let object_normal = self.shape.local_normal_at(object_point);
        let tmp = inversion_mat.transpose().mult(object_normal);

        tuple(tmp.x(), tmp.y(), tmp.z(), 0.0).normalize()
    }

    pub fn material_colour_at(self: &Self, world_point: Tuple4) -> RGB {
        let to_pattern_space =
            self.material().object_to_pattern_spc() *
            self.world_to_object_spc();
        let p = to_pattern_space.mult(world_point);
        self.material().pattern().colour_at(p)
    }

}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Intersection {
    pub t_value: f64,
    pub intersected: Object
}

pub fn intersection(t: f64, s: &Object) -> Intersection {
    Intersection {
        t_value: t,
        intersected: s.clone()
    }
}

pub fn append_intersects(
    orig: &Ray,
    s: &Object,
    vec: &mut Vec<Intersection>
){
    let to_object_space = s.world_to_object_spc();
    let r = transform(orig, &to_object_space);
    let shape = s.shape;
    match shape {
        Shape::Sphere => {
            if let Some((a, b)) = intersect_sphere(&r, s) {
                vec.push(a);
                vec.push(b);
            }
        },
        Shape::Plane => {
            if let Some(a) = intersect_plane(&r, s) {
                vec.push(a);
            }
        }
    }
}

fn intersect_sphere(r: &Ray, s: &Object) -> Option<(Intersection, Intersection)> {
    // presume the sphere is centred at (0,0,0)
    let s_to_ray = r.origin - point(0.0, 0.0, 0.0);
    let a = r.direction.dot(r.direction);
    let b = 2.0 * r.direction.dot(s_to_ray);
    let c = s_to_ray.dot(s_to_ray) - 1.0;
    let discriminant = b.powf(2.0) - (4.0 * a * c);

    if discriminant < 0.0 {
        None
    } else {
        let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b + discriminant.sqrt()) / (2.0 * a);
        Some((intersection(t1, s), intersection(t2, s)))
    }
}

fn intersect_plane(r: &Ray, s: &Object) -> Option<Intersection> {
    if r.direction.y().abs() < EPSILON {
        None
    } else {
        Some(intersection(-r.origin.y() / r.direction.y(), s))
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

/// A point that emits light in all directions.
#[derive(Debug, Copy, Clone)]
pub struct RadialLightSource {
    position: Tuple4,
    intensity: RGB,
}

pub fn point_light(p: Tuple4, i: RGB) -> RadialLightSource {
    RadialLightSource {
        position: p,
        intensity: i,
    }
}

impl RadialLightSource {
    pub fn position(self: &Self) -> Tuple4 {
        self.position
    }

    pub fn intensity(self: &Self) -> RGB {
        self.intensity
    }
}


pub fn reflect(v: Tuple4, norm: Tuple4) -> Tuple4 {
    v - norm.scale(2.0).scale(v.dot(norm))
}

/// Dictates the reflective properties of an object.
///
/// For example, colour and shininess.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Material {
    pattern: Pattern,
    object_to_pattern_spc: Matrix,
    ambient: f64,
    diffuse: f64,
    specular: f64,
    shininess: f64,
    reflective: f64,
}

impl Material {
    pub fn default() -> Material {
        Material {
            pattern: Pattern::solid(RGB::white()),
            object_to_pattern_spc: identity(),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
            reflective: 0.0,
        }
    }

    pub fn pattern(self: &Self) -> Pattern {
        self.pattern
    }
    pub fn set_pattern(self: &mut Self, p: Pattern) -> &mut Self {
        self.pattern = p;
        self
    }
    /// A transformation to transform from pattern space to
    /// object space co-ordinates.
    pub fn pattern_to_object_spc(self: &Self) -> Matrix {
        self.object_to_pattern_spc.inverse()
    }
    pub fn object_to_pattern_spc(self: &Self) -> Matrix {
        self.object_to_pattern_spc
    }
    pub fn set_pattern_to_object_spc(self: &mut Self, m: Matrix) -> &mut Self {
        self.object_to_pattern_spc = m.inverse();
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

    pub fn reflective(&self) -> f64 {
        self.reflective
    }
    pub fn set_reflective(&mut self, r: f64) -> &mut Self {
        self.reflective = r;
        self
    }
}

pub fn lighting(
    light: &RadialLightSource,
    pos: Tuple4,
    normalv: Tuple4,
    obj: &Object,
    eyev: Tuple4,
    in_shadow: bool
) -> RGB {

    let mat = obj.material;
    let matrl_colr: Tuple4 = obj.material_colour_at(pos).into();
    let light_intens: Tuple4 = light.intensity().into();
    let effective_colour: Tuple4 = matrl_colr.mult_pairwise(light_intens);
    let ambient = effective_colour.scale(mat.ambient());

    if in_shadow {
        return RGB::from(ambient);
    }

    let lightv = (light.position() - pos).normalize();
    let light_dot_normal = lightv.dot(normalv);

    let black: Tuple4 = colour(0.0, 0.0, 0.0).into();
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
            let s = light_intens.scale(mat.specular() * factor);
            (d, s)
        }
    };

    RGB::from(ambient + diffuse + specular)
}

/// Represents a scene to be rendered.  Contains lights and
/// objects
#[derive(Debug)]
pub struct World {
    lights: Vec<RadialLightSource>,
    objects: Vec<Object>,
}

impl World {
    pub fn empty() -> World {
        World { objects: vec![], lights: vec![] }
    }

    pub fn default() -> World {
        let light = point_light(point(-10.0, 10.0, -10.0), RGB::white());
        let mut outer = unit_sphere();
        let mut inner = unit_sphere();

        let mut m = Material::default();
        m.set_pattern(Pattern::solid(colour(0.8, 1.0, 0.6)));
        m.set_diffuse(0.7);
        m.set_specular(0.2);
        outer.set_material(m);

        inner.set_object_to_world_spc(scaling(0.5, 0.5, 0.5));

        World {objects: vec![outer, inner], lights: vec![light]}
    }

    pub fn with(lights: Vec<RadialLightSource>, objects: Vec<Object>) -> World {
        World {objects, lights}
    }

    pub fn light_sources(self: &Self) -> Vec<RadialLightSource> {
        self.lights.clone()
    }

    pub fn objects(self: &Self) -> Vec<Object> {
        self.objects.clone()
    }

    pub fn set_objects(self: &mut Self, v: Vec<Object>) -> &mut Self {
        self.objects = v;
        self
    }

    pub fn intersect(self: &Self, r: &Ray) -> Vec<Intersection> {
        let mut v: Vec<Intersection> = Vec::with_capacity(2 * self.objects.len());
        for obj in self.objects.iter() {
            append_intersects(r, obj, &mut v);
        }

        v.sort_by(|i1, i2| {
            let t1 = i1.t_value;
            let t2 = i2.t_value;
            if t1 < t2 {
                Ordering::Less
            } else if t1 > t2 {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        });

        v
    }

    pub fn colour_at_intersect(self: &Self, r: &Ray) -> RGB {
        let ints = self.intersect(r);
        let poss_hit = hit(ints).and_then(|h| {
            let precomputed = precompute(&h, r);
            Some(shade_hit(self, &precomputed))
        });
        poss_hit.unwrap_or(RGB::black())
    }

    pub fn in_shadow(self: &Self, point: Tuple4, light: &RadialLightSource) -> bool {
        let lgt_to_point = light.position() - point;
        let mag = lgt_to_point.magnitude();
        let r = ray(point, lgt_to_point.normalize());

        let h = hit(self.intersect(&r));
        match h {
            Some(i) => i.t_value < mag,
            _ => false
        }
    }

    fn reflected_colour(self: &Self, comps: &Precomputed) -> RGB {
        if comps.object.material.reflective == 0.0 {
            RGB::black()
        } else {
            let reflected_ray = ray(comps.over_point, comps.reflectv);
            let c : Tuple4 = self.colour_at_intersect(&reflected_ray).into();
            RGB::from(c.scale(comps.object.material.reflective))
        }
    }
}


#[derive(Debug)]
struct Precomputed {
    t_value: f64,
    object: Object,
    point: Tuple4,
    eyev: Tuple4,
    normalv: Tuple4,
    inside: bool,
    over_point: Tuple4,
    reflectv: Tuple4,
}

fn precompute(i: &Intersection, r: &Ray) -> Precomputed {
    let pos = position(r.clone(), i.t_value);
    let n = i.intersected.normal_at(pos);
    let e = -(r.direction);
    let inside = n.dot(e) < 0.0;
    let norm = if inside {
        -n
    } else {
        n
    };
    let r = reflect(r.direction, norm);
    Precomputed {
        t_value: i.t_value,
        object: i.intersected,
        point: pos,
        eyev: e,
        normalv: norm,
        inside: inside,
        over_point: pos + (norm.scale(1e-5)),
        reflectv: r,
    }
}

fn shade_hit(world: &World, comps: &Precomputed) -> RGB {
    let mut c = colour(0.0, 0.0, 0.0);
    for light in world.lights.iter() {
        let l = lighting(
            light,
            comps.over_point,
            comps.normalv,
            &comps.object,
            comps.eyev,
            world.in_shadow(comps.over_point, light),
            );
        let r = world.reflected_colour(&comps);
        c = c + l + r;
    }
    c
}

pub fn view_transform(from: Tuple4, to: Tuple4, up: Tuple4) -> Matrix {
    let forward = (to - from).normalize();
    let left = forward.cross(up.normalize());
    let trueup = left.cross(forward);
    let m = matrix((    left.x(),     left.y(),     left.z(), 0.0),
                   (  trueup.x(),   trueup.y(),   trueup.z(), 0.0),
                   (-forward.x(), -forward.y(), -forward.z(), 0.0),
                   (         0.0,          0.0,          0.0, 1.0));
    m * translation(-from.x(), -from.y(), -from.z())
}

/// The configuration of a Camera sets up how a world will be
/// viewed.  It sets up what portion of the scene is visible
/// and what will be rendered in the final image.
///
/// Attributes influencing the final image are the number of
/// horizontal and vertical pixels, the field of view angle,
/// where the camera is positioned (the <em>from</em> position),
/// where it is directed (the <em>to</em> position) and which
/// direction is up (the <em>up</em> vector)
///
/// When the scene is rendered, an imaginary canvas is
/// positioned 1 unit in front of the camera's
/// <em>from</em> position towards the <em>to</em> position.
#[derive(Debug)]
pub struct Camera {
    hsize: u32,
    vsize: u32,
    fov: f64,
    half_width: f64,
    half_height: f64,
    pixel_size: f64,
    inverse_view_t: Matrix,
}

impl Camera {
    pub fn new(hsize: u32, vsize: u32, fov: f64) -> Camera {
        let half_view = (fov / 2.0).tan();
        let aspect = f64::from(hsize) / f64::from(vsize);
        let half_width = if aspect >= 1.0 {
            half_view
        } else {
            half_view * aspect
        };
        let half_height = if aspect >= 1.0 {
            half_view / aspect
        } else {
            half_view
        };

        let pixel_size = (half_width * 2.0) / f64::from(hsize);

        Camera {
            hsize,
            vsize,
            fov,
            half_width,
            half_height,
            pixel_size,
            inverse_view_t: identity(),
        }
    }
    pub fn hsize(self: &Self) -> u32 {
        self.hsize
    }
    pub fn vsize(self: &Self) -> u32 {
        self.vsize
    }
    pub fn field_of_view(self: &Self) -> f64 {
        self.fov
    }
    pub fn view_transform(self: &Self) -> Matrix {
        self.inverse_view_t.inverse()
    }
    pub fn inverse_view_t(self: &Self) -> Matrix {
        self.inverse_view_t
    }
    pub fn set_view_transform(self: &mut Self, m: Matrix) -> &mut Self {
        self.inverse_view_t = m.inverse();
        self
    }
    pub fn pixel_size(self: &Self) -> f64 {
        self.pixel_size
    }

    pub fn ray_for_pixel(self: &Self, px: u32, py: u32) -> Ray {
        assert!(px < self.hsize);
        assert!(py < self.vsize);

        let xoffset = (f64::from(px) + 0.5) * self.pixel_size;
        let yoffset = (f64::from(py) + 0.5) * self.pixel_size;
        let worldx = self.half_width - xoffset;
        let worldy = self.half_height - yoffset;
        let inv_t = self.inverse_view_t();

        let pixel = inv_t.mult(point(worldx, worldy, -1.0));
        let origin = inv_t.mult(point(0.0, 0.0, 0.0));
        let dir = (pixel - origin).normalize();
        ray(origin, dir)
    }

    pub fn render(self: &Self, w: &World) -> Canvas {
        let mut canv = canvas(self.hsize as usize, self.vsize as usize);
        for y in 0..self.vsize {
            for x in 0..self.hsize {
                let r = self.ray_for_pixel(x, y);
                let c = w.colour_at_intersect(&r);
                canv.set_colour_at(x as usize, y as usize, c);
            }
        }
        canv
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
#[derive(Deserialize)]
pub enum Pattern {
    Solid(RGB),
    Stripes {a: RGB, b: RGB},
    Gradient { from: RGB, to: RGB},
    Ring {a: RGB, b: RGB},
    Checkers {a: RGB, b: RGB},
}

impl Pattern {
    pub fn solid(c: RGB) -> Pattern {
        Pattern::Solid(c)
    }
    pub fn stripes(c1: RGB, c2: RGB) -> Pattern {
        Pattern::Stripes {a: c1, b: c2}
    }
    pub fn gradient(a: RGB, b: RGB) -> Pattern {
        Pattern::Gradient { from: a, to: b }
    }
    pub fn ring(a: RGB, b: RGB) -> Pattern {
        Pattern::Ring { a, b }
    }
    pub fn checkers(a: RGB, b: RGB) -> Pattern {
        Pattern::Checkers { a, b }
    }

    pub fn colour_at(self: &Self, pattern_space_pos: Tuple4) -> RGB {
        match *self {
            Pattern::Solid(c) => c,
            Pattern::Stripes {a, b} =>
                stripe_colour(a, b, pattern_space_pos),
            Pattern::Gradient {from, to} =>
                gradient_colour(from, to, pattern_space_pos),
            Pattern::Ring {a, b} =>
                ring_colour(a, b, pattern_space_pos),
            Pattern::Checkers {a, b} =>
                checkers_colour(a, b, pattern_space_pos),
        }
    }
}

fn gradient_colour(
    RGB {inner: a}: RGB,
    RGB {inner: b}: RGB,
    pos: Tuple4)
    -> RGB {
    // the x component of pos is expected to be int the range [0,1]
    let distance = b - a;
    let frac = if pos.x() >= 1.0 {
        1.0
    } else if pos.x() < 0.0{
        0.0
    } else {
        pos.x() - pos.x().floor()
    };

    RGB::from(a + (distance.scale(frac)))
}
fn stripe_colour(a: RGB, b: RGB, pos: Tuple4) -> RGB {
    // pos must be in object co-ordinates not world...

    if rem_euclid(pos.x(), 2.0) >= 1.0 {
        b
    } else {
        a
    }
}

fn ring_colour(a: RGB, b: RGB, p: Tuple4) -> RGB {
    let s = (p.x().powi(2) + p.z().powi(2)).sqrt();
    if (s % 2.0).floor() == 0.0 {
        a
    } else {
        b
    }
}

fn checkers_colour(a: RGB, b: RGB, p: Tuple4) -> RGB {
    let s = p.x().floor() + p.y().floor() + p.z().floor();
    if rem_euclid(s, 2.0) == 0.0 {
        a
    } else {
        b
    }
}

fn rem_euclid(x: f64, y: f64) -> f64 {
    let d = x / y;
    let r = x % y;
    if d < 0.0 {
        if r == 0.0 {
            r
        } else {
            y + r
        }
    } else {
        r
    }
}

// ----- Testing non-public shading functions
#[cfg(test)]
mod internal_shading {
    use crate::*;


    #[test]
    fn intersect_a_world_with_a_ray() {
        let world = World::default();
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let intersects = world.intersect(&r);
        assert_eq!(intersects.len(), 4);

        assert_eq!(intersects[0].t_value, 4.0);
        assert_eq!(intersects[1].t_value, 4.5);
        assert_eq!(intersects[2].t_value, 5.5);
        assert_eq!(intersects[3].t_value, 6.0);
    }

    #[test]
    fn precompute_state_of_intersection() {
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let shape = unit_sphere();
        let i = intersection(4.0, &shape);
        let comps = precompute(&i, &r);
        assert_eq!(comps.t_value, 4.0);
        assert_eq!(comps.object, shape);
        assert_eq!(comps.point, point(0.0, 0.0, -1.0));
        assert_eq!(comps.eyev, vector(0.0, 0.0, -1.0));
        assert_eq!(comps.normalv, vector(0.0, 0.0, -1.0));
    }

    #[test]
    fn the_hit_when_intersection_occurs_on_outside() {
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let shape = unit_sphere();
        let i = intersection(4.0, &shape);
        let comps = precompute(&i, &r);
        assert_eq!(comps.inside, false);
    }

    #[test]
    fn the_hit_when_an_intersection_occurs_on_inside() {
        let r = ray(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        let shape = unit_sphere();
        let i = intersection(1.0, &shape);
        let comps = precompute(&i, &r);
        assert_eq!(comps.t_value, 1.0);
        assert_eq!(comps.object, shape);
        assert_eq!(comps.point, point(0.0, 0.0, 1.0));
        assert_eq!(comps.eyev, vector(0.0, 0.0, -1.0));
        assert_eq!(comps.normalv, vector(0.0, 0.0, -1.0));
        assert_eq!(comps.inside, true);
    }

    #[test]
    fn shade_an_intersection_point() {
        let w = World::default();
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let shape = w.objects()[0];
        let i = intersection(4.0, &shape);
        let comps = precompute(&i, &r);
        let c = shade_hit(&w, &comps);
        assert_eq!(c, colour(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn shade_an_intersection_point_from_inside() {
        let light = point_light(point(0.0, 0.25, 0.0), RGB::white());
        let w = World::with(vec![light], World::default().objects());
        let r = ray(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        let shape = w.objects()[1];
        let i = intersection(0.5, &shape);

        let comps = precompute(&i, &r);
        let c = shade_hit(&w, &comps);
        assert_eq!(c, colour(0.90498, 0.90498, 0.90498));
    }
}

// ----- Test computing shadows
#[cfg(test)]
mod shadows {
    use crate::*;

    #[test]
    fn an_intersection_in_shadow_returns_ambient_colour() {
        let l = point_light(point(0.0, 0.0, -10.0), RGB::white());
        let s1 = unit_sphere();
        let s2 = *(unit_sphere().set_object_to_world_spc(translation(0.0, 0.0, 10.0)));

        let objects = vec![s1, s2];
        let w = World::with(vec![l], objects);
        let r = ray(point(0.0, 0.0, 5.0), vector(0.0, 0.0, 1.0));
        let i = intersection(4.0, &s2);
        let c = shade_hit(&w, &precompute(&i, &r));
        assert_eq!(c, colour(0.1, 0.1, 0.1));
    }

    #[test]
    fn the_hit_should_bump_the_point_slightly_in_the_direction_of_normalv() {
        let shape = *(unit_sphere().set_object_to_world_spc(translation(0.0, 0.0, 1.0)));

        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let i = intersection(5.0, &shape);
        let precomputed = precompute(&i, &r);
        assert_eq!(true, precomputed.point.z() > precomputed.over_point.z())
    }
}

#[cfg(test)]
mod planes {
    use crate::*;
    use std::f64::consts::FRAC_PI_2;

    #[test]
    fn normal_of_a_plane_is_up() {
        let p: Shape = Shape::Plane;
        let n1 = p.local_normal_at(point(0.0, 0.0, 0.0));
        let n2 = p.local_normal_at(point(10.0, 0.0, -10.0));
        let n3 = p.local_normal_at(point(-5.0, 0.0, 150.0));

        assert_eq!(n1, vector(0.0, 1.0, 0.0));
        assert_eq!(n2, vector(0.0, 1.0, 0.0));
        assert_eq!(n3, vector(0.0, 1.0, 0.0));
    }

    #[test]
    fn normal_of_a_plane_object_is_constant_everywhere() {
        let mut p: Object = plane();
        p.set_object_to_world_spc(rotation_z(FRAC_PI_2));
        let n1 = p.normal_at(point(0.0, 0.0, 0.0));
        let n2 = p.normal_at(point(10.0, 0.0, -10.0));
        let n3 = p.normal_at(point(-5.0, 0.0, 150.0));

        assert_eq!(n1, vector(-1.0, 0.0, 0.0));
        assert_eq!(n2, vector(-1.0, 0.0, 0.0));
        assert_eq!(n3, vector(-1.0, 0.0, 0.0));
    }
}

#[cfg(test)]
mod euclid {
    use crate::*;

    #[test]
    fn positive_numerator_positive_denominator() {
        assert_eq!(rem_euclid(0.0, 2.0), 0.0);
        assert_eq!(rem_euclid(0.5, 2.0), 0.5);
        assert_eq!(rem_euclid(1.0, 2.0), 1.0);
        assert_eq!(rem_euclid(1.5, 2.0), 1.5);
        assert_eq!(rem_euclid(2.0, 2.0), 0.0);
        assert_eq!(rem_euclid(2.5, 2.0), 0.5);
        assert_eq!(rem_euclid(3.0, 2.0), 1.0);
    }

    #[test]
    fn negative_numerator_positive_denominator() {
        assert_eq!(rem_euclid(0.0, 2.0), 0.0);
        assert_eq!(rem_euclid(-0.5, 2.0), 1.5);
        assert_eq!(rem_euclid(-1.0, 2.0), 1.0);
        assert_eq!(rem_euclid(-1.5, 2.0), 0.5);
        assert_eq!(rem_euclid(-2.0, 2.0), 0.0);
        assert_eq!(rem_euclid(-2.5, 2.0), 1.5);
        assert_eq!(rem_euclid(-3.0, 2.0), 1.0);
    }
}

#[cfg(test)]
mod reflection {
    use crate::*;
    use std::f64::consts::*;

    const ROOT2_BY_2: f64 = SQRT_2 / 2.0;

    #[test]
    fn compute_reflective_vector() {
        let p = plane();
        let r = ray(point(0.0, 1.0, -1.0), vector(0.0, -ROOT2_BY_2, ROOT2_BY_2));
        let inter = intersection(SQRT_2, &p);
        let comps = precompute(&inter, &r);

        assert_eq!(comps.reflectv, vector(0.0, ROOT2_BY_2, ROOT2_BY_2));
    }

    #[test]
    fn reflected_colour_for_non_reflective_material_is_black() {
        let mut w = World::default();
        w.objects[1].material.set_reflective(0.0);
        w.objects[1].material.set_ambient(1.0);
        let r = ray(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        let obj: Object = w.objects[1];
        let i = intersection(1.0, &obj);

        assert_eq!(w.reflected_colour(&precompute(&i, &r)), RGB::black());
    }

    #[test]
    fn reflected_colour_for_reflective_material() {
        let mut w = World::default();
        let mut p = plane();
        p.material.reflective = 0.5;
        p.set_object_to_world_spc(translation(0.0, -1.0, 0.0));
        w.objects.push(p);

        let r = ray(point(0.0, 0.0, -3.0), vector(0.0, -ROOT2_BY_2, ROOT2_BY_2));
        let i = intersection(SQRT_2, &p);
        let rgb = w.reflected_colour(&precompute(&i, &r));

        assert_eq!(rgb, colour(0.19033, 0.23791, 0.14274));
    }

    #[test]
    fn shade_hit_for_reflective_material() {
        let mut w = World::default();
        let mut p = plane();
        p.material.reflective = 0.5;
        p.set_object_to_world_spc(translation(0.0, -1.0, 0.0));
        w.objects.push(p);

        let r = ray(point(0.0, 0.0, -3.0), vector(0.0, -ROOT2_BY_2, ROOT2_BY_2));
        let i = intersection(SQRT_2, &p);
        let rgb = shade_hit(&w, &precompute(&i, &r));

        assert_eq!(rgb, colour(0.87675, 0.92434, 0.82918));
    }

    fn almost_eq(x1: f64, x2: f64) -> bool {
        f64::abs(x1 - x2) < EPSILON
    }
}


