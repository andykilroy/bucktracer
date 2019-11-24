use crate::*;


/// Dictates whether a cylinder is open ended
/// or has closed ends.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum CylKind {
    Open,
    Closed,
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


pub fn normal_of_cylinder(lbound: f64, ubound: f64, pos: Tuple4) -> Tuple4 {
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


pub fn append_cyl_intersects(
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
