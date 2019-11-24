use crate::*;

pub fn triangle(p1: Tuple4, p2: Tuple4, p3: Tuple4) -> Object {
    Object {
        world_to_object_spc: identity(),
        material: Material::default(),
        shape: Shape::Triangle {p1, p2, p3},
    }
}

pub fn append_tri_intersects(r: &Ray, obj: &Object, vec: &mut Vec<Intersection>) {
    unimplemented!()
}
