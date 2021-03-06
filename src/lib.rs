use std::cmp::Ordering;
use std::ops::{Add, Mul};
use std::vec;
#[macro_use]
extern crate lazy_static;

use serde::Deserialize;

pub mod math;
pub mod png;
pub mod ppm;
mod shape;
pub mod wavefront;
mod partition;

use crate::math::*;
pub use crate::shape::*;

pub use partition::binary_partition;
pub use partition::flatten;
// TODO for testing.  Remove once debugged
pub use partition::bbox_map;

const EPSILON: f64 = 1e-5;

fn almost_eq(x1: f64, x2: f64) -> bool {
    f64::abs(x1 - x2) < EPSILON
}

/// A structure representing a colour in red, green, and blue components.
#[derive(Debug, Copy, Clone, PartialEq, Deserialize)]
#[serde(from = "(f64, f64, f64)")]
pub struct RGB {
    inner: Tuple4,
}

/// Create a colour using red, green, and blue values, each a number in the range
/// [0.0, 1.0]
pub fn colour(red: f64, green: f64, blue: f64) -> RGB {
    RGB {
        inner: vector(red, green, blue),
    }
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
        RGB {
            inner: (self.inner + rhs.inner),
        }
    }
}
impl Mul<f64> for RGB {
    type Output = RGB;

    fn mul(self, rhs: f64) -> Self::Output {
        RGB::from(self.inner.scale(rhs))
    }
}

/// A structure used to record pixel colour values
/// indexed by 2D coordinates.  (0,0) represents the
/// top-left pixel of the canvas.
#[derive(Debug)]
pub struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<RGB>,
}

/// Create a structure onto which an image will be rendered.
/// Initially, every pixel is black.
pub fn canvas(w: usize, h: usize) -> Canvas {
    let length = w * h;
    let arr = vec![colour(0.0, 0.0, 0.0); length];
    Canvas {
        width: w,
        height: h,
        pixels: arr,
    }
}

impl Canvas {
    pub fn colour_at(&self, x: usize, y: usize) -> RGB {
        self.pixels[y * self.width + x]
    }
    pub fn set_colour_at(&mut self, x: usize, y: usize, c: RGB) {
        self.pixels[y * self.width + x] = c;
    }
    pub fn width(&self) -> usize {
        self.width
    }
    pub fn height(&self) -> usize {
        self.height
    }
}

/// Represents a ray fired from a point in a particular
/// direction
#[derive(Debug, Clone)]
pub struct Ray {
    origin: Tuple4,
    direction: Tuple4,
}

/// Create a ray pointing in a particular direction, rooted at a point.
pub fn ray(origin: Tuple4, direction: Tuple4) -> Ray {
    Ray { origin, direction }
}

impl Ray {
    pub fn position(&self, t: f64) -> Tuple4 {
        self.origin + (self.direction.scale(t))
    }
    pub fn transform(&self, m: &Matrix) -> Ray {
        ray(m.mult(self.origin), m.mult(self.direction))
    }
    pub fn origin(&self) -> Tuple4 {
        self.origin
    }
    pub fn direction(&self) -> Tuple4 {
        self.direction
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Intersection {
    t_value: f64,
    intersected: Object,
    to_group_spc: Matrix,
    uv: Option<(f64, f64)>,
}

impl Intersection {
    pub fn t_value(&self) -> f64 {
        self.t_value
    }

    pub fn intersected(&self) -> Object {
        self.intersected.clone()
    }

    pub fn normal_at(&self, world_point: Tuple4) -> Tuple4 {
        // TODO can we opt out of the matrix multiplications?
        let p = self.to_group_spc.mult(world_point);
        let tmp = self.intersected.normal_at(p, self);
        self.to_group_spc.inverse().mult(tmp)
    }

    fn to_group_space(&self) -> Matrix {
        self.to_group_spc
    }

    fn set_to_group_space(&mut self, matr: Matrix) -> &mut Self {
        self.to_group_spc = matr;
        self
    }

    // TODO we want this to be private, it only has relevance for smooth triangles.
    pub fn u(&self) -> Option<f64> {
        match self.uv {
            Some((u, _v)) => Some(u),
            None => None,
        }
    }

    // TODO we want this to be private, it only has relevance for smooth triangles.
    pub fn v(&self) -> Option<f64> {
        match self.uv {
            Some((_u, v)) => Some(v),
            None => None,
        }
    }
}

pub fn intersection(t: f64, s: &Object) -> Intersection {
    Intersection {
        t_value: t,
        intersected: s.clone(),
        to_group_spc: identity(),
        uv: None,
    }
}

pub fn intersection_with_uv(t: f64, s: &Object, u: f64, v: f64) -> Intersection {
    Intersection {
        t_value: t,
        intersected: s.clone(),
        to_group_spc: identity(),
        uv: Some((u, v)),
    }
}

pub fn index_of_hit(intersects: &[Intersection]) -> Option<usize> {
    intersects
        .iter()
        .enumerate()
        .filter(|(_, i)| i.t_value >= 0.0)
        .fold(None, nearer_intersect)
        .map(|(ind, _)| ind)
}

fn nearer_intersect<'a>(
    nearest: Option<(usize, &'a Intersection)>,
    x: (usize, &'a Intersection),
) -> Option<(usize, &'a Intersection)> {
    match nearest {
        None => Some(x),
        Some((_, c)) => {
            if x.1.t_value < c.t_value {
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
    transparency: f64,
    refractive_index: f64,
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
            transparency: 0.0,
            refractive_index: 1.0,
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

    pub fn transparency(&self) -> f64 {
        self.transparency
    }
    pub fn set_transparency(&mut self, t: f64) -> &mut Self {
        self.transparency = t;
        self
    }

    pub fn refractive_index(&self) -> f64 {
        self.refractive_index
    }
    pub fn set_refractive_index(&mut self, ri: f64) -> &mut Self {
        self.refractive_index = ri;
        self
    }
}

pub fn lighting(
    light: &RadialLightSource,
    pos: Tuple4,
    normalv: Tuple4,
    obj: &Object,
    eyev: Tuple4,
    light_allowance: f64,
) -> RGB {
    let mat = obj.material();
    let matrl_colr: Tuple4 = obj.material_colour_at(pos).into();
    let light_intens: Tuple4 = light.intensity().into();
    let effective_colour: Tuple4 = matrl_colr.mult_pairwise(light_intens);
    let ambient = effective_colour.scale(mat.ambient());

    if light_allowance == 0.0 {
        return RGB::from(ambient);
    }

    let lightv = (light.position() - pos).normalize();
    let light_dot_normal = lightv.dot(normalv);

    let black: Tuple4 = colour(0.0, 0.0, 0.0).into();
    let (diffuse, specular) = if light_dot_normal < 0.0 {
        // the light is behind the surface
        (black, black)
    } else {
        let d = effective_colour.scale(mat.diffuse() * light_dot_normal);
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

    let l: Tuple4 = (diffuse + specular).scale(light_allowance);
    RGB::from(ambient + l)
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
        World {
            objects: vec![],
            lights: vec![],
        }
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

        World {
            objects: vec![outer, inner],
            lights: vec![light],
        }
    }

    pub fn with(lights: Vec<RadialLightSource>, objects: Vec<Object>) -> World {
        World { objects, lights }
    }

    pub fn light_sources(self: &Self) -> &[RadialLightSource] {
        &self.lights
    }

    pub fn objects(self: &Self) -> &[Object] {
        &self.objects
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

    pub fn colour_at_intersect(self: &Self, r: &Ray, rlimit: u32) -> RGB {
        let ints = self.intersect(r);
        let poss_hit = index_of_hit(&ints).and_then(|hit_index| {
            let precomputed = hit_data(r, hit_index, &ints);
            Some(shade_hit(self, &precomputed, rlimit))
        });
        poss_hit.unwrap_or_else(RGB::black)
    }

    fn reflected_colour(self: &Self, comps: &HitCalculations, rlimit: u32) -> RGB {
        if rlimit == 0 {
            return RGB::black();
        }
        if comps.object.material().reflective() == 0.0 {
            RGB::black()
        } else {
            let reflected_ray = ray(comps.over_point, comps.reflectv);
            let c = self.colour_at_intersect(&reflected_ray, rlimit - 1);
            c * comps.object.material().reflective()
        }
    }

    fn refracted_colour(&self, comps: &HitCalculations, rlimit: u32) -> RGB {
        if rlimit == 0 {
            return RGB::black();
        }
        if comps.object.material().transparency() == 0.0 {
            RGB::black()
        } else {
            let ratio = comps.n1 / comps.n2;
            let cos_i = comps.eyev.dot(comps.normalv);
            let sin2_t = ratio.powi(2) * (1.0 - cos_i.powi(2));

            if sin2_t > 1.0 {
                // total internal reflection occurs therefore no
                // colour contributes.
                RGB::black()
            } else {
                // Snell's law: for incoming ray i and refracted ray t,
                // and angles theta_i and theta_t of i and t made respectively
                // with the normal of the surface, the following relationship holds:
                //
                //    sin(theta_i)     n2
                //    ------------  =  --
                //    sin(theta_t)     n1

                let cos_t = (1.0 - sin2_t).sqrt();
                let direction =
                    comps.normalv.scale((ratio * cos_i) - cos_t) - (comps.eyev.scale(ratio));
                let refract_ray = ray(comps.under_point, direction);
                let c = self.colour_at_intersect(&refract_ray, rlimit - 1);
                c * comps.object.material().transparency()
            }
        }
    }

    pub fn light_factor(&self, point: Tuple4, light: &RadialLightSource) -> f64 {
        let point_to_light = light.position() - point;
        let mag = point_to_light.magnitude();
        let r = ray(point, point_to_light.normalize());

        let accumulatd: f64 = self
            .intersect(&r)
            .iter()
            .filter(|i| i.t_value >= 0.0 && i.t_value < mag)
            .map(|h| h.intersected.material().transparency())
            .fold(1.0, |x, y| x * y);
        accumulatd
    }
}

#[derive(Debug)]
struct HitCalculations {
    t_value: f64,
    object: Object,
    point: Tuple4,
    eyev: Tuple4,
    normalv: Tuple4,
    inside: bool,
    over_point: Tuple4,
    under_point: Tuple4,
    reflectv: Tuple4,
    n1: f64,
    n2: f64,
}

fn hit_data(r: &Ray, hit_index: usize, intersects: &[Intersection]) -> HitCalculations {
    let hit: &Intersection = &intersects[hit_index];
    let pos = r.position(hit.t_value());

    // TODO think of a test to assert that the hit's normal takes into
    // account the group transformations.
    let n = hit.normal_at(pos);
    let e = -(r.direction);
    let is_inside = n.dot(e) < 0.0;
    let norm = if is_inside { -n } else { n };
    let r = reflect(r.direction, norm);
    let (n1, n2) = refractive_indices(hit_index, intersects);

    HitCalculations {
        t_value: hit.t_value(),
        object: hit.intersected(),
        point: pos,
        eyev: e,
        normalv: norm,
        inside: is_inside,
        over_point: pos + (norm.scale(1e-5)),
        under_point: pos - (norm.scale(1e-5)),
        reflectv: r,
        n1,
        n2,
    }
}

fn refractive_indices(hit_index: usize, intersects: &[Intersection]) -> (f64, f64) {
    let mut containers: Vec<&Object> = Vec::with_capacity(intersects.len());
    let mut n1 = 1.0;
    let mut n2 = 1.0;
    for (i, current) in intersects.iter().enumerate() {
        if i == hit_index {
            if !containers.is_empty() {
                n1 = containers.last().unwrap().material().refractive_index();
            }
        }
        let object: &Object = &current.intersected;

        match find(&containers, object) {
            Some(obj_index) => containers.remove(obj_index),
            None => {
                containers.push(object);
                object
            }
        };

        if i == hit_index {
            if !containers.is_empty() {
                n2 = containers.last().unwrap().material().refractive_index();
            }
            break;
        }
    }

    (n1, n2)
}

fn find(objects: &[&Object], obj: &Object) -> Option<usize> {
    for (i, item) in objects.iter().enumerate() {
        if *item == obj {
            return Some(i);
        }
    }
    None
}

fn shade_hit(world: &World, comps: &HitCalculations, rlimit: u32) -> RGB {
    world.lights.iter().fold(RGB::black(), |prev_colour, light| {
        let surface = lighting(
            light,
            comps.over_point,
            comps.normalv,
            &comps.object,
            comps.eyev,
            world.light_factor(comps.over_point, light),
        );
        let reflected = world.reflected_colour(&comps, rlimit);
        let refracted = world.refracted_colour(&comps, rlimit);

        let c = if comps.object.material().reflective() > 0.0
            && comps.object.material().transparency() > 0.0
        {
            let reflectance = schlick(comps);
            surface + (reflected * reflectance) + (refracted * (1.0 - reflectance))
        } else {
            surface + reflected + refracted
        };
        prev_colour + c
    })
}

fn schlick(comps: &HitCalculations) -> f64 {
    fn calc(comps: &HitCalculations, cos: f64) -> f64 {
        let r0 = ((comps.n1 - comps.n2) / (comps.n1 + comps.n2)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cos).powi(5)
    }

    let cos = comps.eyev.dot(comps.normalv);
    if comps.n1 > comps.n2 {
        let n = comps.n1 / comps.n2;
        let sin2_t = n.powi(2) * (1.0 - cos.powi(2));
        if sin2_t > 1.0 {
            1.0
        } else {
            let cos_t = (1.0 - sin2_t).sqrt();
            calc(comps, cos_t)
        }
    } else {
        calc(comps, cos)
    }
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

const RECURSION_LIMIT: u32 = 5;

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
    pub fn orient(&mut self, from: Tuple4, to: Tuple4, up: Tuple4) {
        self.set_view_transform(view_transform(from, to, up));
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

    pub fn render<F>(&self, w: &World, mut progress: F) -> Canvas
        where F: FnMut(u32, u32) -> ()
    {
        let mut canv = canvas(self.hsize as usize, self.vsize as usize);
        for y in 0..self.vsize {
            for x in 0..self.hsize {
                let r = self.ray_for_pixel(x, y);
                let c = w.colour_at_intersect(&r, RECURSION_LIMIT);
                canv.set_colour_at(x as usize, y as usize, c);
            }
            progress(y + 1, self.vsize);
        }
        canv
    }
}

/// Describes how to colour the surface of an object.
#[derive(Debug, Copy, Clone, PartialEq, Deserialize)]
pub enum Pattern {
    Solid(RGB),
    Stripes { a: RGB, b: RGB },
    Gradient { from: RGB, to: RGB },
    Ring { a: RGB, b: RGB },
    Checkers { a: RGB, b: RGB },
    Test,
}

impl Pattern {
    pub fn solid(c: RGB) -> Pattern {
        Pattern::Solid(c)
    }
    pub fn stripes(c1: RGB, c2: RGB) -> Pattern {
        Pattern::Stripes { a: c1, b: c2 }
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
    pub fn tester() -> Pattern {
        Pattern::Test
    }

    pub fn colour_at(self: &Self, pattern_space_pos: Tuple4) -> RGB {
        match *self {
            Pattern::Solid(c) => c,
            Pattern::Stripes { a, b } => stripe_colour(a, b, pattern_space_pos),
            Pattern::Gradient { from, to } => gradient_colour(from, to, pattern_space_pos),
            Pattern::Ring { a, b } => ring_colour(a, b, pattern_space_pos),
            Pattern::Checkers { a, b } => checkers_colour(a, b, pattern_space_pos),
            Pattern::Test => no_op_colour(pattern_space_pos),
        }
    }
}

fn no_op_colour(pattern_space_pos: Tuple4) -> RGB {
    RGB::from(pattern_space_pos)
}

fn gradient_colour(RGB { inner: a }: RGB, RGB { inner: b }: RGB, pos: Tuple4) -> RGB {
    // the x component of pos is expected to be int the range [0,1]
    let distance = b - a;
    let frac = if pos.x() >= 1.0 {
        1.0
    } else if pos.x() < 0.0 {
        0.0
    } else {
        pos.x() - pos.x().floor()
    };

    RGB::from(a + (distance.scale(frac)))
}

fn stripe_colour(a: RGB, b: RGB, pos: Tuple4) -> RGB {
    // pos must be in object co-ordinates not world...

    if pos.x().rem_euclid(2.0) >= 1.0 {
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
    if s.rem_euclid(2.0) == 0.0 {
        a
    } else {
        b
    }
}

// A helper for the following test modules
#[cfg(test)]
fn singleton_hit_data(r: &Ray, hit: &Intersection) -> HitCalculations {
    hit_data(r, 0, &[hit.clone()])
}

#[cfg(test)]
mod test_shading;

#[cfg(test)]
mod test_shadows;

#[cfg(test)]
mod test_reflection;

#[cfg(test)]
mod test_refraction;
