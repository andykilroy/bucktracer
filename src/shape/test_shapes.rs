use crate::*;
use std::f64::consts::FRAC_PI_2;

#[allow(non_snake_case)]
#[test]
fn shape___local_normal_of_a_plane_is_up() {
    let p: Shape = Shape::Plane;
    let n1 = p.local_normal_at(point(0.0, 0.0, 0.0));
    let n2 = p.local_normal_at(point(10.0, 0.0, -10.0));
    let n3 = p.local_normal_at(point(-5.0, 0.0, 150.0));

    assert_eq!(n1, vector(0.0, 1.0, 0.0));
    assert_eq!(n2, vector(0.0, 1.0, 0.0));
    assert_eq!(n3, vector(0.0, 1.0, 0.0));
}

#[allow(non_snake_case)]
#[test]
fn object___normal_of_a_plane_is_constant_everywhere() {
    let mut p: Object = plane();
    p.set_object_to_world_spc(rotation_z(FRAC_PI_2));
    let n1 = p.normal_at(point(0.0, 0.0, 0.0));
    let n2 = p.normal_at(point(10.0, 0.0, -10.0));
    let n3 = p.normal_at(point(-5.0, 0.0, 150.0));

    assert_eq!(n1, vector(-1.0, 0.0, 0.0));
    assert_eq!(n2, vector(-1.0, 0.0, 0.0));
    assert_eq!(n3, vector(-1.0, 0.0, 0.0));
}
