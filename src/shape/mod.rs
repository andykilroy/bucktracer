use crate::*;

mod sphere;
mod plane;
mod cube;
mod cylinder;
mod bounds;
mod group;
mod triangle;

pub use sphere::unit_sphere;
pub use sphere::glass_sphere;
pub use plane::plane;
pub use cube::cube;
pub use cylinder::CylKind;
pub use cylinder::cylinder;
pub use cylinder::inf_cylinder;
pub use group::group;
pub use triangle::triangle;
pub use triangle::smooth_triangle;
pub use bounds::Bounds;


/// Determines what shape an object has.
///
/// Influences the calculation of surface normals and intersections.
#[derive(Debug, Clone, PartialEq)]
pub enum Shape {
    Sphere,
    Plane,
    Cube,
    Cylinder { kind: CylKind, lbound: f64, ubound: f64 },
    Triangle { p1: Tuple4, p2: Tuple4, p3: Tuple4, e1: Tuple4, e2: Tuple4, normal: Tuple4},
    SmoothTri { p1: Tuple4, p2: Tuple4, p3: Tuple4, n1: Tuple4, n2: Tuple4, n3: Tuple4 },
    Group { children: Vec<Object>, bounds: Bounds },
}

impl Shape {
    /// Calculates a normal appropriate for the object at the
    /// specified position.  The position is always in object
    /// co-ordinates.  The returned normal vector is also in
    /// object space.
    fn local_normal_at(&self, position: Tuple4, hit: &Intersection) -> Tuple4 {
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
            Shape::Triangle { normal, .. } => *normal,
            Shape::SmoothTri { p1, p2, p3, n1, n2, n3 } => {
                triangle::normal_of_smooth_triangle(*p1, *p2, *p3, *n1, *n2, *n3, position, hit)
            },
            Shape::Group { .. } => {
                unimplemented!()
            },
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
            Shape::Triangle {p1, p2, p3, ..} => {
                Bounds::new(Tuple4::min_all(&[*p1, *p2, *p3]), Tuple4::max_all(&[*p1, *p2, *p3]))
            },
            Shape::SmoothTri {p1, p2, p3, ..} => {
                Bounds::new(Tuple4::min_all(&[*p1, *p2, *p3]), Tuple4::max_all(&[*p1, *p2, *p3]))
            },
            Shape::Group {children: _, bounds} => *bounds,
        }
    }
}

impl std::fmt::Display for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Shape::Sphere {..} => write!(f, "Sphere"),
            Shape::Plane {..} => write!(f, "Plane"),
            Shape::Cube {..} => write!(f, "Cube"),
            Shape::Triangle {..} => write!(f, "Triangle"),
            Shape::SmoothTri {..} => write!(f, "SmoothTri"),
            Shape::Cylinder {..} => write!(f, "Cylinder"),
            Shape::Group {children, ..} => write!(f, "Group ({})", children.len()),
        }
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
    shape: Shape,
    world_to_object_spc: Matrix,
    material: Material,
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

    pub fn normal_at(self: &Self, world_point: Tuple4, supplemental: &Intersection) -> Tuple4 {
        let inversion_mat = self.world_to_object_spc();
        let object_point = inversion_mat.mult(world_point);
        let object_normal = self.shape.local_normal_at(object_point, supplemental);
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
            Shape::Group { children, .. } => &children,
            _ => &[]
        }
    }

    pub fn bounds(&self) -> Bounds {
        let bnds = self.shape.bounds();
        let corners = bnds.all_corners();
        let ninf = std::f64::NEG_INFINITY;
        let pinf = std::f64::INFINITY;

        let mut minp = point(
            if bnds.min().x() == ninf { ninf } else { pinf },
            if bnds.min().y() == ninf { ninf } else { pinf },
            if bnds.min().z() == ninf { ninf } else { pinf },
        );

        let to_world_spc = self.object_to_world_spc();
        for vertex in &corners {
            let p = to_world_spc.mult(*vertex);
            minp = Tuple4::min(p, minp);
        }

        let mut maxp = point(
            if bnds.max().x() == pinf { pinf } else { ninf },
            if bnds.max().y() == pinf { pinf } else { ninf },
            if bnds.max().z() == pinf { pinf } else { ninf },
        );

        for vertex in &corners {
            let p = to_world_spc.mult(*vertex);
            maxp = Tuple4::max(p, maxp);
        }

        Bounds::new(minp, maxp)
    }
}

impl std::fmt::Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Object {{ {}, {} }}", self.shape, self.bounds())
    }
}

// TODO this should be an internal function, not public.
pub fn append_intersects(orig: &Ray, s: &Object, vec: &mut Vec<Intersection>) {
    let to_object_space = s.world_to_object_spc();
    let r = orig.transform(&to_object_space);
    let shape = &s.shape;
    match shape {
        Shape::Sphere => {
            if let Some((a, b)) = sphere::intersect_sphere(&r, s) {
                vec.push(a);
                vec.push(b);
            }
        },
        Shape::Plane => {
            if let Some(a) = plane::intersect_plane(&r, s) {
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
        Shape::Group {children, ..} => {
            group::append_grp_intersects(&r, s, vec, &children)
        },
        Shape::Triangle {p1, p2, p3, e1, e2, ..} => {
            triangle::append_tri_intersects(&r, s, vec, *p1, *p2, *p3, *e1, *e2)
        },
        Shape::SmoothTri {p1, p2, p3, ..} => {
            triangle::append_tri_intersects(&r, s, vec, *p1, *p2, *p3, *p2 - *p1, *p3 - *p1)
        },
    }
}

pub fn obj_at(root: &Object, path: &[usize]) -> Option<Object> {
    if path.len() == 0 { return Some(root.clone()); };
    let index = *(path.first().unwrap());
    match &root.shape {
        Shape::Group { children, .. } => {
            let next = children.get(index)?;
            obj_at(next, &path[1..])
        },
        _ => None
    }
}
