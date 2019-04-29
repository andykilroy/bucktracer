use bucktracer::*;

#[test]
fn default_transformation_is_identity() {
    let s = unit_sphere();
    assert_eq!(identity(), s.transform());
}

#[test]
fn change_a_spheres_transform() {
    let mut s = unit_sphere();
    let t = translation(2.0, 3.0, 4.0);
    s.set_transform(&t);

    assert_eq!(t, s.transform());
}
