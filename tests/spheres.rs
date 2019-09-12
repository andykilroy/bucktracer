use bucktracer::*;

#[test]
fn default_transformation_is_identity() {
    let s = unit_sphere();
    assert_eq!(identity(), s.transform_to_world());
}

#[test]
fn change_a_spheres_transform() {
    let mut s = unit_sphere();
    let t = translation(2.0, 3.0, 4.0);
    s.set_transform_to_world(t);

    assert_eq!(t, s.transform_to_world());
}
