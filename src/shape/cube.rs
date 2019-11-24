use crate::*;
use crate::shape::bounds::Bounds;
use crate::shape::bounds::intersect_bounding_box;

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


pub fn normal_of_cube(pos: Tuple4) -> Tuple4 {

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


pub fn intersect_cube(r: &Ray, obj: &Object) -> Option<(Intersection, Intersection)> {
    match intersect_bounding_box(r, Bounds::unit()) {
        Some((tmin, tmax)) => {
            Some((intersection(tmin, obj), intersection(tmax, obj)))
        },
        _ => None,
    }
}
