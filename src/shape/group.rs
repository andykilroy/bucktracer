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
    let grp = Shape::Group { children };
    Object {
        world_to_object_spc: identity(),
        material: Material::default(),
        shape: grp,
    }
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

