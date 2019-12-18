use crate::*;
use crate::shape::bounds;


/// Creates a group of objects.
///
/// Intended to be used where a list of objects is to be treated as part of a whole.
/// E.g. 4 cylinders and one (flattened) cube can be placed into a group to
/// represent a table.
///
/// Groups can also be used to partition a scene, to help the ray tracer quickly
/// discard large numbers of objects that don't intersect the ray.
pub fn group(children: Vec<Object>) -> Object {
    let bounds = Bounds::new(min_point(children.as_slice()), max_point(children.as_slice()));
    let grp = Shape::Group { children, bounds };

    Object {
        world_to_object_spc: identity(),
        material: Material::default(),
        shape: grp,
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

pub fn append_grp_intersects(r: &Ray, grp: &Object, vec: &mut Vec<Intersection>, children: &[Object]) {
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

