use bucktracer::*;

#[test]
fn default_transformation_is_identity() {
    let s = unit_sphere();
    assert_eq!(identity(), s.object_to_world_spc());
}

#[test]
fn change_a_spheres_transform() {
    let mut s = unit_sphere();
    let t = translation(2.0, 3.0, 4.0);
    s.set_object_to_world_spc(t);

    assert_eq!(t, s.object_to_world_spc());
}

#[test]
fn create_glass_sphere() {
    let s: Object = glass_sphere();
    assert_eq!(s.object_to_world_spc(), identity());
    assert_eq!(s.material().transparency(), 1.0);
    assert_eq!(s.material().refractive_index(), 1.5);
}
