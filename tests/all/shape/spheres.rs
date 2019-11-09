use bucktracer::*;
use std::f64::consts::PI;

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



#[test]
fn what_is_the_normal_on_a_sphere_on_x_axis() {
    let s = unit_sphere();
    let n = s.normal_at(point(1.0, 0.0, 0.0));
    assert_eq!(n, vector(1.0, 0.0, 0.0));
}

#[test]
fn what_is_the_normal_on_a_sphere_on_y_axis() {
    let s = unit_sphere();
    let n = s.normal_at(point(0.0, 1.0, 0.0));
    assert_eq!(n, vector(0.0, 1.0, 0.0));
}

#[test]
fn what_is_the_normal_on_a_sphere_on_z_axis() {
    let s = unit_sphere();
    let n = s.normal_at(point(0.0, 0.0, 1.0));
    assert_eq!(n, vector(0.0, 0.0, 1.0));
}

#[test]
fn normal_on_non_axial_point() {
    let sqrt3by3 = 3.0f64.sqrt() / 3.0;
    let s = unit_sphere();
    let n = s.normal_at(point(sqrt3by3, sqrt3by3, sqrt3by3));

    assert_eq!(n, vector(sqrt3by3, sqrt3by3, sqrt3by3));
}

#[test]
fn a_normal_is_normalised() {
    let sqrt3by3 = 3.0f64.sqrt() / 3.0;
    let s = unit_sphere();
    let n = s.normal_at(point(sqrt3by3, sqrt3by3, sqrt3by3));

    assert_eq!(n, n.normalize());
}

#[test]
fn normal_on_translated_sphere() {
    let mut s = unit_sphere();
    s.set_object_to_world_spc(translation(0.0, 1.0, 0.0));
    let n = s.normal_at(point(0.0, 1.70711, -0.70711));

    assert_eq!(n, vector(0.0, 0.70711, -0.70711));
}

#[test]
fn normal_on_transformed_sphere() {
    let mut s = unit_sphere();
    let pi_by_5 = PI / 5.0;
    let rt2by2 = 2.0f64.sqrt() / 2.0;

    s.set_object_to_world_spc(scaling(1.0, 0.5, 1.0) * rotation_z(pi_by_5));
    let n = s.normal_at(point(0.0, rt2by2, -rt2by2));
    assert_eq!(n, vector(0.0, 0.97014, -0.24254));
}

#[test]
fn reflect_a_vector_approaching_at_45_deg() {
    let v = vector(1.0, -1.0, 0.0);
    let n = vector(0.0, 1.0, 0.0);
    let r = reflect(v, n);
    assert_eq!(r, vector(1.0, 1.0, 0.0));
}

#[test]
fn reflect_vector_off_slanted_surface() {
    let rt2by2 = 2.0f64.sqrt() / 2.0;

    let v = vector(0.0, -1.0, 0.0);
    let n = vector(rt2by2, rt2by2, 0.0);
    let r = reflect(v, n);
    assert_eq!(r, vector(1.0, 0.0, 0.0));
}
