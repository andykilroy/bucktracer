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

pub fn append_tri_intersects(
    r: &Ray, obj: &Object, vec: &mut Vec<Intersection>,
    p1: Tuple4, _p2: Tuple4, _p3: Tuple4,
    e1: Tuple4, e2: Tuple4)
{
    let c = r.direction.cross(e2);
    let det = e1.dot(c);
    if det.abs() >= EPSILON {
        let f = 1.0 / det;
        let p1_to_origin = r.origin - p1;
        let u = f * p1_to_origin.dot(c);

        if u < 0.0 || u > 1.0 {
            return;
        }

        let origin_cross_e1 = p1_to_origin.cross(e1);
        let v = f * r.direction.dot(origin_cross_e1);

        if v < 0.0 || (u + v) > 1.0 {
            return;
        }

        let t = f * e2.dot(origin_cross_e1);
        vec.push(intersection_with_uv(t, obj, u, v));
    }
}


pub fn smooth_triangle(p1: Tuple4, p2: Tuple4, p3: Tuple4, n1: Tuple4, n2: Tuple4, n3: Tuple4) -> Object {
    Object {
        world_to_object_spc: identity(),
        material: Material::default(),
        shape: Shape::SmoothTri {
            p1, p2, p3, n1, n2, n3,
        }
    }
}
