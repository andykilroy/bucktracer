use crate::*;

pub fn triangle(p1: Tuple4, p2: Tuple4, p3: Tuple4) -> Object {
    let e1 = p2 - p1;
    let e2 = p3 - p1;
    let normal = e2.cross(e1).normalize();
    Object {
        world_to_object_spc: identity(),
        material: Material::default(),
        shape: Shape::Triangle {p1, p2, p3, e1, e2, normal},
    }
}

pub fn append_tri_intersects(r: &Ray, obj: &Object, vec: &mut Vec<Intersection>) {
    unimplemented!()
}
