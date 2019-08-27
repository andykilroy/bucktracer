use std::vec;
use std::io::Result as IOResult;
use std::io::Write;
use std::str::FromStr;
use std::cmp::Ordering;

mod math;

pub use crate::math::*;

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

    pub fn set_transform(self: &mut Self, m: &Matrix) -> &mut Self {
        self.transform = m.clone();
        self
    }

    pub fn material(&self) -> Material {
        self.material
    }

    pub fn set_material(self: &mut Self, m: &Material) -> &mut Self {
        self.material = m.clone();
        self
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

#[derive(Debug, Copy, Clone)]
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
    ray(origin, point - origin)
}

fn to_f64(v: usize) -> f64 {
    let s = format!("{}", v);
    return f64::from_str(&s).unwrap();
}

#[derive(Debug)]
pub struct OldCamera {
    canvas: Canvas,
    plane: BoundedPlane,
}

impl OldCamera {
    pub fn new(canv: Canvas, pln: BoundedPlane) -> OldCamera {
        OldCamera {
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

        let world_width = self.plane.upper_right - self.plane.lower_left;
        let x_factor : f64 = world_width.x() / to_f64(self.canvas.width);
        let y_factor : f64 = world_width.y() / to_f64(self.canvas.height);
        let x_float = to_f64(x);
        let y_float = to_f64(y);

        point(x_float * x_factor, world_width.y() - ((y_float + 1.0) * y_factor), 0.0)
    }
}

#[derive(Debug)]
pub struct BoundedPlane {
    lower_left: Tuple4,
    upper_right: Tuple4,
    surface_normal: Tuple4,
}

type Triple = (f64, f64, f64);

pub fn camera(c: Canvas, l_left: Triple, u_right: Triple, normal: Triple) -> OldCamera {
    OldCamera::new(
        c,
        BoundedPlane {
            lower_left: point(l_left.0, l_left.1, l_left.2),
            upper_right: point(u_right.0, u_right.1, u_right.2),
            surface_normal: vector(normal.0, normal.1, normal.2),
        },
    )
}

// ---- END camera related stuff

pub fn normal_at(s: &Sphere, world_point: Tuple4) -> Tuple4 {
    let inversion_mat = s.transform.inverse();
    let object_point = inversion_mat.mult(world_point);
    let object_normal = object_point - s.pos;
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

#[derive(Debug)]
pub struct World {
    objects: Vec<Sphere>,
    lights: Vec<RadialLightSource>,
}

impl World {
    pub fn empty() -> World {
        World { objects: vec![], lights: vec![] }
    }

    pub fn default() -> World {
        let light = point_light(point(-10.0, 10.0, -10.0), white());
        let mut outer = unit_sphere();
        let mut inner = unit_sphere();

        let mut m = Material::default();
        m.set_colour(colour(0.8, 1.0, 0.6));
        m.set_diffuse(0.7);
        m.set_specular(0.2);
        outer.set_material(&m);

        inner.set_transform(&scaling(0.5, 0.5, 0.5));

        World {objects: vec![outer, inner], lights: vec![light]}
    }

    pub fn with(objects: Vec<Sphere>, lights: Vec<RadialLightSource>) -> World {
        World {objects, lights}
    }

    pub fn light_sources(self: &Self) -> Vec<RadialLightSource> {
        self.lights.clone()
    }

    pub fn objects(self: &Self) -> Vec<Sphere> {
        self.objects.clone()
    }

    pub fn set_objects(self: &mut Self, v: Vec<Sphere>) -> &mut Self {
        self.objects = v;
        self
    }

    pub fn intersect(self: &Self, r: &Ray) -> Vec<Intersection> {
        let mut v: Vec<Intersection> = vec![];
        for obj in self.objects.iter() {
            v.extend(intersect(r, obj).iter());
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

    pub fn colour_at_intersect(self: &Self, r: &Ray) -> Tuple4 {
        let ints = self.intersect(r);
        let black = colour(0.0, 0.0, 0.0);
        let poss_hit = hit(ints).and_then(|h| {
            let precomputed = precompute(&h, r);
            Some(shade_hit(self, &precomputed))
        });
        poss_hit.unwrap_or(black)
    }
}

#[derive(Debug)]
struct Precomputed {
    t_value: f64,
    object: Sphere,
    point: Tuple4,
    eyev: Tuple4,
    normalv: Tuple4,
    inside: bool,
}

fn precompute(i: &Intersection, r: &Ray) -> Precomputed {
    let pos = position(r.clone(), i.t_value);
    let n = normal_at(&i.intersected, pos);
    let e = -(r.direction);
    let inside = n.dot(e) < 0.0;
    let norm = if inside {
        -n
    } else {
        n
    };
    Precomputed {
        t_value: i.t_value,
        object: i.intersected,
        point: pos,
        eyev: e,
        normalv: norm,
        inside: inside,
    }
}

fn shade_hit(world: &World, comps: &Precomputed) -> Tuple4 {
    let mut c = colour(0.0, 0.0, 0.0);
    for light in world.lights.iter() {
        c = c + lighting(
            light,
            comps.point,
            comps.normalv,
            &comps.object.material,
            comps.eyev);
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

// A camera where the canvas is 1 unit in front of the camera's
// position (as given by from)
pub struct Camera {
    hsize: u32,
    vsize: u32,
    fov: f64,
    half_width: f64,
    half_height: f64,
    pixel_size: f64,
    transform: Matrix,
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
            transform: identity(),
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
    pub fn transform(self: &Self) -> Matrix {
        self.transform
    }
    pub fn set_transform(self: &mut Self, m: Matrix) -> &mut Self {
        self.transform = m;
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
        let inv_t = self.transform.inverse();

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

mod internal_rays {
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
        let cam = camera(
            canvas(300, 300),
            (0.0, 0.0, 0.0),
            (60.0, 60.0, 0.0),
            (0.0, 0.0, 1.0),
        );

        assert_eq!(cam.pixel_to_point(0, 299), point(0.0, 0.0, 0.0));
        assert_eq!(cam.pixel_to_point(1, 299), point(0.2, 0.0, 0.0));
        assert_eq!(cam.pixel_to_point(2, 299), point(0.4, 0.0, 0.0));
        assert_eq!(cam.pixel_to_point(3, 299), point(0.6, 0.0, 0.0));
        assert_eq!(cam.pixel_to_point(4, 299), point(0.8, 0.0, 0.0));
        assert_eq!(cam.pixel_to_point(5, 299), point(1.0, 0.0, 0.0));
        assert_eq!(cam.pixel_to_point(299, 299), point(59.8, 0.0, 0.0));
    }

    #[test]
    fn pixel_index_to_point_in_space_ys() {
        let mut cam = camera(
            canvas(300, 300),
            (0.0, 0.0, 0.0),
            (60.0, 60.0, 0.0),
            (0.0, 0.0, 1.0),
        );

        assert_eq!(cam.pixel_to_point(0, 299), point(0.0, 0.0, 0.0));
        assert_eq!(cam.pixel_to_point(0, 298), point(0.0, 0.2, 0.0));
        assert_eq!(cam.pixel_to_point(0, 297), point(0.0, 0.4, 0.0));
        assert_eq!(cam.pixel_to_point(0, 296), point(0.0, 0.6, 0.0));
        assert_eq!(cam.pixel_to_point(0, 295), point(0.0, 0.8, 0.0));
        assert_eq!(cam.pixel_to_point(0, 294), point(0.0, 1.0, 0.0));
        assert_eq!(cam.pixel_to_point(0, 0), point(0.0, 59.8, 0.0));
        assert_eq!(cam.pixel_to_point(0, 1), point(0.0, 59.6, 0.0));
        assert_eq!(cam.pixel_to_point(0, 2), point(0.0, 59.4, 0.0));
        assert_eq!(cam.pixel_to_point(0, 3), point(0.0, 59.2, 0.0));
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

// ----- Testing non-public shading functions
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
        let light = point_light(point(0.0, 0.25, 0.0), white());
        let w = World::with(World::default().objects(), vec![light]);
        let r = ray(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        let shape = w.objects()[1];
        let i = intersection(0.5, &shape);

        let comps = precompute(&i, &r);
        let c = shade_hit(&w, &comps);
        assert_eq!(c, colour(0.90498, 0.90498, 0.90498));
    }
}
