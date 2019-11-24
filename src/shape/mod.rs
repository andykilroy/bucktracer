use crate::*;

mod cube;
mod cylinder;
mod bounds;

pub use cube::cube;
pub use cylinder::CylKind;
pub use cylinder::cylinder;
pub use cylinder::inf_cylinder;
pub use bounds::Bounds;

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
            Shape::Cube => cube::normal_of_cube(position),
            Shape::Cylinder { lbound, ubound, ..} => {
                cylinder::normal_of_cylinder(*lbound, *ubound, position)
            },
            Shape::Group { children : _ } => {
                unimplemented!()
            }
        }
    }

    fn bounds(&self) -> Bounds {
        match self {
            Shape::Sphere => Bounds::new(point(-1.0, -1.0, -1.0), point(1.0, 1.0, 1.0)),
            Shape::Cube => Bounds::new(point(-1.0, -1.0, -1.0), point(1.0, 1.0, 1.0)),
            Shape::Cylinder {lbound, ubound, ..} =>
                Bounds::new(point(-1.0, *lbound, -1.0), point(1.0, *ubound, 1.0)),
            Shape::Plane => {
                Bounds::new(
                    point(std::f64::NEG_INFINITY, std::f64::NEG_INFINITY, std::f64::NEG_INFINITY),
                    point(std::f64::    INFINITY, std::f64::    INFINITY, std::f64::    INFINITY),
                )
            },
            Shape::Group {children} => {
                Bounds::new(min_point(children.as_slice()), max_point(children.as_slice()))
            },
        }
    }
}

fn min_point(arr: &[Object]) -> Tuple4 {
    let inf = std::f64::INFINITY;
    let inf_t = tuple(inf, inf, inf, inf);
    let p = arr.iter().map(|o| o.bounds().min()).fold(inf_t, Tuple4::min);
    p
}

fn max_point(arr: &[Object]) -> Tuple4 {
    let inf = std::f64::NEG_INFINITY;
    let inf_t = tuple(inf, inf, inf, inf);
    let p = arr.iter().map(|o| o.bounds().max()).fold(inf_t, Tuple4::max);
    p
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
        let corners = bnds.all_corners();
        let ninf = std::f64::NEG_INFINITY;
        let pinf = std::f64::INFINITY;
        let mut minx = if bnds.min().x() == ninf { ninf } else { pinf };
        let mut miny = if bnds.min().y() == ninf { ninf } else { pinf };
        let mut minz = if bnds.min().z() == ninf { ninf } else { pinf };

        let to_world_spc = self.object_to_world_spc();
        for vertex in &corners {
            let p = to_world_spc.mult(*vertex);
            if p.x() < minx { minx = p.x() };
            if p.y() < miny { miny = p.y() };
            if p.z() < minz { minz = p.z() };
        }

        let mut maxx = if bnds.max().x() == pinf { pinf } else { ninf };
        let mut maxy = if bnds.max().y() == pinf { pinf } else { ninf };
        let mut maxz = if bnds.max().z() == pinf { pinf } else { ninf };

        for vertex in &corners {
            let p = to_world_spc.mult(*vertex);
            if p.x() > maxx { maxx = p.x() };
            if p.y() > maxy { maxy = p.y() };
            if p.z() > maxz { maxz = p.z() };
        }

        Bounds::new(point(minx, miny, minz), point(maxx, maxy, maxz))
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
            if let Some((a, b)) = cube::intersect_cube(&r, s) {
                vec.push(a);
                vec.push(b);
            }
        },
        Shape::Cylinder { lbound, ubound, .. } => {
            cylinder::append_cyl_intersects(&r, s, vec, *lbound, *ubound)
        },
        Shape::Group {children} => {
            append_grp_intersects(&r, s, vec, &children)
        }
    }
}

fn append_grp_intersects(r: &Ray, grp: &Object, vec: &mut Vec<Intersection>, children: &[Object]) {
    if bounds::intersect_bounding_box(r, grp.shape.bounds()).is_none() {
        return;
    }

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


fn intersect_plane(r: &Ray, s: &Object) -> Option<Intersection> {
    if r.direction.y().abs() < EPSILON {
        None
    } else {
        Some(intersection(-r.origin.y() / r.direction.y(), s))
    }
}
