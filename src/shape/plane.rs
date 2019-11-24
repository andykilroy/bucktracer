use crate::*;

/// Creates an x-z plane intersecting y=0.
pub fn plane() -> Object {
    Object {
        world_to_object_spc: identity(),
        material: Material::default(),
        shape: Shape::Plane,
    }
}

pub fn intersect_plane(r: &Ray, s: &Object) -> Option<Intersection> {
    if r.direction.y().abs() < EPSILON {
        None
    } else {
        Some(intersection(-r.origin.y() / r.direction.y(), s))
    }
}
