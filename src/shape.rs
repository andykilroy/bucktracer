use crate::*;
use std::f64::INFINITY;

/// Creates a sphere of radius 1 centred at the origin.
pub fn unit_sphere() -> Object {
    Object {
        world_to_object_spc: identity(),
        material: Material::default(),
        shape: Shape::Sphere,
    }
}

/// Creates a transparent sphere of radius 1 centred at the origin.
pub fn glass_sphere() -> Object {
    let mut glass = Material::default();
    glass.set_transparency(1.0);
    glass.set_refractive_index(1.5);
    Object {
        world_to_object_spc: identity(),
        material: glass,
        shape: Shape::Sphere,
    }
}

/// Creates an x-z plane intersecting y=0.
pub fn plane() -> Object {
    Object {
        world_to_object_spc: identity(),
        material: Material::default(),
        shape: Shape::Plane,
    }
}

/// Creates a cube centred on the origin whose vertices are
/// 1 unit away from the nearest point on the x,y,z axis.
///
/// The length of each edge is 2.  The distance from the origin
/// to any vertex is sqrt(2).
pub fn cube() -> Object {
    Object {
        world_to_object_spc: identity(),
        material: Material::default(),
        shape: Shape::Cube,
    }
}

/// Creates an infinitely long open cylinder whose length extends along the y-axis,
/// with radius 1.
///
/// Imagine a circle of radius 1, centred at the origin
/// in the x-z plane, extruded along the y-axis.
pub fn inf_cylinder() -> Object {
    cylinder(CylKind::Open, std::f64::NEG_INFINITY, std::f64::INFINITY)
}

/// Creates an infinitely long open cylinder whose length extends along the y-axis,
/// with radius 1.
pub fn cylinder(kind: CylKind, lbound: f64, ubound: f64) -> Object {
    Object {
        world_to_object_spc: identity(),
        material: Material::default(),
        shape: Shape::Cylinder { kind, lbound, ubound },
    }
}

/// Creates a group of objects.
///
/// Intended to be used where a list of objects is to be treated as part of a whole.
/// E.g. 4 cylinders and one (flattened) cube can be placed into a group to
/// represent a table.
///
/// Groups can also be used to partition a scene, to help the ray tracer quickly
/// discard large numbers of objects that don't intersect the ray.
pub fn group(children: Vec<Object>) -> Object {
    let grp = Shape::Group { children };
    Object {
        world_to_object_spc: identity(),
        material: Material::default(),
        shape: grp,
    }
}

/// Determines what shape an object has.
///
/// Influences the calculation of surface normals and intersections.
#[derive(Debug, Clone, PartialEq)]
pub enum Shape {
    Sphere,
    Plane,
    Cube,
    Cylinder { kind: CylKind, lbound: f64, ubound: f64 },
    Group { children: Vec<Object> }
}

/// Dictates whether a cylinder is open ended
/// or has closed ends.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum CylKind {
    Open,
    Closed,
}

impl Shape {
    /// Calculates a normal appropriate for the object at the
    /// specified position.  The position is always in object
    /// co-ordinates.  The returned normal vector is also in
    /// object space.
    fn local_normal_at(&self, position: Tuple4) -> Tuple4 {
        match self {
            Shape::Sphere => {
                // presume the sphere is centred at (0, 0, 0)
                position - point(0.0, 0.0, 0.0)
            }
            Shape::Plane => vector(0.0, 1.0, 0.0),
            Shape::Cube => normal_of_cube(position),
            Shape::Cylinder { lbound, ubound, ..} => {
                normal_of_cylinder(*lbound, *ubound, position)
            },
            Shape::Group { children : _ } => {
                unimplemented!()
            }
        }
    }

    fn bounds(&self) -> Bounds {
        match self {
            Shape::Sphere => {
                Bounds {
                    min : point(-1.0, -1.0, -1.0),
                    max : point( 1.0,  1.0,  1.0),
                }
            },
            Shape::Cube => {
                Bounds {
                    min : point(-1.0, -1.0, -1.0),
                    max : point( 1.0,  1.0,  1.0),
                }
            },
            Shape::Cylinder {lbound, ubound, ..} => {
                Bounds {
                    min : point(-1.0, *lbound, -1.0),
                    max : point( 1.0, *ubound,  1.0),
                }
            },
            Shape::Plane => {
                Bounds {
                    min : point(std::f64::NEG_INFINITY, std::f64::NEG_INFINITY, std::f64::NEG_INFINITY),
                    max : point(std::f64::    INFINITY, std::f64::    INFINITY, std::f64::    INFINITY),
                }
            },
            Shape::Group {children} => {
                Bounds {
                    min : min_point(children.as_slice()),
                    max : max_point(children.as_slice()),
                }
            },
        }
    }
}

fn min_point(arr: &[Object]) -> Tuple4 {
    fn minfunc(x:f64, y:f64) -> f64 {if x < y { x } else { y }};

    let inf = std::f64::INFINITY;
    let minx = arr.iter().map(|o| o.bounds().min().x()).fold(inf, minfunc);
    let miny = arr.iter().map(|o| o.bounds().min().y()).fold(inf, minfunc);
    let minz = arr.iter().map(|o| o.bounds().min().z()).fold(inf, minfunc);

    point(minx, miny, minz)
}

fn max_point(arr: &[Object]) -> Tuple4 {
    fn maxfunc(x:f64, y:f64) -> f64 {if x < y { y } else { x }};

    let inf = std::f64::NEG_INFINITY;
    let maxx = arr.iter().map(|o| o.bounds().max().x()).fold(inf, maxfunc);
    let maxy = arr.iter().map(|o| o.bounds().max().y()).fold(inf, maxfunc);
    let maxz = arr.iter().map(|o| o.bounds().max().z()).fold(inf, maxfunc);

    point(maxx, maxy, maxz)
}

fn normal_of_cylinder(lbound: f64, ubound: f64, pos: Tuple4) -> Tuple4 {
    let mag = pos.x().powi(2) + pos.z().powi(2);
    if mag < 1.0 {
        if pos.y() >= (ubound - crate::EPSILON) {
            vector(0.0, 1.0, 0.0)
        } else if pos.y() <= (lbound + crate::EPSILON) {
            vector(0.0, -1.0, 0.0)
        } else {
            vector(pos.x(), 0.0, pos.z())
        }
    } else {
        vector(pos.x(), 0.0, pos.z())
    }
}

fn normal_of_cube(pos: Tuple4) -> Tuple4 {

    let mut maxindex = 0;
    let mut prev = pos.x().abs();
    for (i, item) in [pos.y().abs(), pos.z().abs()].iter().enumerate() {
        if *item > prev {
            prev = *item;
            maxindex = i + 1;
        }
    }

    match maxindex {
        0 => vector(pos.x(), 0.0, 0.0),
        1 => vector(0.0, pos.y(), 0.0),
        _ => vector(0.0, 0.0, pos.z()),
    }
}

/// An object to be placed in the world; something to be rendered.
///
/// The object has a transform that dictates where it is placed in
/// the world, and also whether it is scaled or rotated in any way.
/// It also is associated with material dictating its surface
/// properties.
#[derive(Debug, Clone, PartialEq)]
pub struct Object {
    world_to_object_spc: Matrix,
    material: Material,
    shape: Shape,
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

    pub fn mut_material(&mut self) -> &mut Material {
        &mut self.material
    }

    pub fn normal_at(self: &Self, world_point: Tuple4) -> Tuple4 {
        let inversion_mat = self.world_to_object_spc();
        let object_point = inversion_mat.mult(world_point);
        let object_normal = self.shape.local_normal_at(object_point);
        let tmp = inversion_mat.transpose().mult(object_normal);

        vector(tmp.x(), tmp.y(), tmp.z()).normalize()
    }

    pub fn material_colour_at(self: &Self, world_point: Tuple4) -> RGB {
        let to_pattern_space = self.material().object_to_pattern_spc() * self.world_to_object_spc();
        let p = to_pattern_space.mult(world_point);
        self.material().pattern().colour_at(p)
    }

    pub fn children(&self) -> &[Object] {
        match &self.shape {
            Shape::Group { children } => &children,
            _ => &[]
        }
    }

    pub fn bounds(&self) -> Bounds {
        let bnds = self.shape.bounds();
        let combs = bnds.combinations();
        let ninf = std::f64::NEG_INFINITY;
        let pinf = std::f64::INFINITY;
        let mut minx = if bnds.min.x() == ninf { ninf } else { pinf };
        let mut miny = if bnds.min.y() == ninf { ninf } else { pinf };
        let mut minz = if bnds.min.z() == ninf { ninf } else { pinf };

        let to_world_spc = self.object_to_world_spc();
        for vertex in &combs {
            let p = to_world_spc.mult(*vertex);
            if p.x() < minx { minx = p.x() };
            if p.y() < miny { miny = p.y() };
            if p.z() < minz { minz = p.z() };
        }

        let mut maxx = if bnds.max.x() == pinf { pinf } else { ninf };
        let mut maxy = if bnds.max.y() == pinf { pinf } else { ninf };
        let mut maxz = if bnds.max.z() == pinf { pinf } else { ninf };

        for vertex in &combs {
            let p = to_world_spc.mult(*vertex);
            if p.x() > maxx { maxx = p.x() };
            if p.y() > maxy { maxy = p.y() };
            if p.z() > maxz { maxz = p.z() };
        }
        Bounds { min: point(minx, miny, minz), max: point(maxx, maxy, maxz) }
    }
}


// TODO this should be an internal function, not public.
pub fn append_intersects(orig: &Ray, s: &Object, vec: &mut Vec<Intersection>) {
    let to_object_space = s.world_to_object_spc();
    let r = orig.transform(&to_object_space);
    let shape = &s.shape;
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
        },
        Shape::Cube => {
            if let Some((a, b)) = intersect_cube(&r, s) {
                vec.push(a);
                vec.push(b);
            }
        },
        Shape::Cylinder { lbound, ubound, .. } => {
            append_cyl_intersects(&r, s, vec, *lbound, *ubound)
        },
        Shape::Group {children} => {
            append_grp_intersects(&r, s, vec, &children)
        }
    }
}

fn append_grp_intersects(r: &Ray, grp: &Object, vec: &mut Vec<Intersection>, children: &[Object]) {
    // TODO test the ray with this group's bounds, can we short circuit?
    let initial = vec.len();
    for obj in children {
        append_intersects(r, obj, vec);
    }

    let final_len = vec.len();
    for i in vec[initial..final_len].iter_mut() {
        let m = i.to_group_space() * grp.world_to_object_spc();
        i.set_to_group_space(m);
    }
}

fn intersect_sphere(r: &Ray, sphere: &Object) -> Option<(Intersection, Intersection)> {
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
        Some((intersection(t1, sphere), intersection(t2, sphere)))
    }
}

fn intersect_cube(r: &Ray, obj: &Object) -> Option<(Intersection, Intersection)> {

    let (x_tmin, x_tmax) = check_axis(r.origin.x(), r.direction.x());
    let (y_tmin, y_tmax) = check_axis(r.origin.y(), r.direction.y());
    let (z_tmin, z_tmax) = check_axis(r.origin.z(), r.direction.z());

    let tmin = [x_tmin, y_tmin, z_tmin].iter().cloned().fold(std::f64::MIN, f64::max);
    let tmax = [x_tmax, y_tmax, z_tmax].iter().cloned().fold(std::f64::MAX, f64::min);

    if tmin > tmax {
        None
    } else {
        Some((intersection(tmin, obj), intersection(tmax, obj)))
    }
}

fn check_axis(origin: f64, direction: f64) -> (f64, f64) {
    let tmin_numerator = -1.0 - origin;
    let tmax_numerator = 1.0 - origin;

    let (tmin, tmax) = if direction.abs() >= EPSILON {
        (tmin_numerator / direction,
         tmax_numerator / direction)
    } else {
        (tmin_numerator * INFINITY,
         tmax_numerator * INFINITY)
    };

    if tmin > tmax { (tmax, tmin) }
    else {(tmin, tmax)}
}


fn intersect_plane(r: &Ray, s: &Object) -> Option<Intersection> {
    if r.direction.y().abs() < EPSILON {
        None
    } else {
        Some(intersection(-r.origin.y() / r.direction.y(), s))
    }
}

fn append_cyl_intersects(
    r: &Ray,
    cyl: &Object,
    vec: &mut Vec<Intersection>,
    lower: f64,
    upper: f64)
{
    if let Some((a, b)) = intersect_cylinder(&r, cyl) {
        let ya = (r.origin + (r.direction.scale(a.t_value))).y();
        let yb = (r.origin + (r.direction.scale(b.t_value))).y();

        if lower < ya && ya < upper {
            vec.push(a);
        }

        if lower < yb && yb < upper {
            vec.push(b);
        }
    }

    intersect_caps(cyl, r, vec);
}

fn intersect_caps(cyl: &Object, r: &Ray, vec: &mut Vec<Intersection>) {

    if let Shape::Cylinder { kind, lbound, ubound } = cyl.shape {
        if kind == CylKind::Open || almost_eq(r.direction.y().abs(), 0.0) {
            return;
        }

        let t0 = (lbound - r.origin.y()) / r.direction.y() ;
        let t1 = (ubound - r.origin.y()) / r.direction.y() ;

        if check_cap(r, t0) {
            vec.push(intersection(t0, cyl))
        }

        if check_cap(r, t1) {
            vec.push(intersection(t1, cyl))
        }
    }
}

fn check_cap(ray: &Ray, t: f64) -> bool {
    let x = ray.origin.x() + (t * ray.direction.x());
    let z = ray.origin.z() + (t * ray.direction.z());
    x.powi(2) + z.powi(2) <= 1.0
}

fn intersect_cylinder(ray: &Ray, obj: &Object) -> Option<(Intersection, Intersection)> {
    let a = ray.direction.x().powi(2) + ray.direction.z().powi(2);

    if almost_eq(a, 0.0) { return None; }

    let b =
        (2.0 * ray.origin.x() * ray.direction.x()) +
        (2.0 * ray.origin.z() * ray.direction.z());

    let c = ray.origin.x().powi(2) + ray.origin.z().powi(2) - 1.0;
    let disc = b.powi(2) - (4.0 * a * c);
    if disc < 0.0 { return None ; }

    let t0 = ( -b - disc.sqrt()) / (2.0*a);
    let t1 = ( -b + disc.sqrt()) / (2.0*a);

    Some((intersection(t0, obj), intersection(t1, obj)))
}

/// Describes an axis aligned bounding box
#[derive(Debug, Copy, Clone)]
pub struct Bounds {
    min: Tuple4,
    max: Tuple4,
}

impl Bounds {
    pub fn min(&self) -> Tuple4 {
        self.min
    }
    pub fn max(&self) -> Tuple4 {
        self.max
    }
    pub fn combinations(&self) -> [Tuple4 ; 8] {
        [
            point(self.min.x(), self.min.y(), self.min.z()),
            point(self.min.x(), self.min.y(), self.max.z()),
            point(self.min.x(), self.max.y(), self.min.z()),
            point(self.min.x(), self.max.y(), self.max.z()),
            point(self.max.x(), self.min.y(), self.min.z()),
            point(self.max.x(), self.min.y(), self.max.z()),
            point(self.max.x(), self.max.y(), self.min.z()),
            point(self.max.x(), self.max.y(), self.max.z()),
        ]
    }
}
