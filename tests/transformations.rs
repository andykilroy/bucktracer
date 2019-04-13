use bucktracer::*;
use std::f64::consts::*;

#[test]
fn translate() {
    let t = translation(5.0, -3.0, 2.0);
    let p = point(-3.0, 4.0, 5.0);

    assert_eq!(t.mult(p), point(2.0, 1.0, 7.0));
}

#[test]
fn mult_by_inverse_translation() {
    let t = translation(5.0, -3.0, 2.0);
    let i = t.clone().inverse();
    let p = point(-3.0, 4.0, 5.0);

    assert_eq!(i.mult(p), point(-8.0, 7.0, 3.0));
}

#[test]
fn translation_does_not_affect_vectors() {
    let t = translation(5.0, -3.0, 2.0);
    let v = vector(-3.0, 4.0, 5.0);

    assert_eq!(t.mult(v), v);
}

#[test]
fn scaling_a_point() {
    let t = scaling(2.0, 3.0, 4.0);
    let p = point(-4.0, 6.0, 8.0);

    assert_eq!(t.mult(p), point(-8.0, 18.0, 32.0));
}

#[test]
fn scaling_a_vector() {
    let t = scaling(2.0, 3.0, 4.0);
    let v = vector(-4.0, 6.0, 8.0);

    assert_eq!(t.mult(v), vector(-8.0, 18.0, 32.0));
}

#[test]
fn multiply_by_inverse_of_a_scaling_matrix() {
    let t = scaling(2.0, 3.0, 4.0);
    let i = t.clone().inverse();
    let v = vector(-4.0, 6.0, 8.0);

    assert_eq!(i.mult(v), vector(-2.0, 2.0, 2.0));
}

#[test]
fn reflection_is_scaling_by_a_negative_value() {
    let t = scaling(-1.0, 1.0, 1.0);
    let p = point(2.0, 3.0, 4.0);

    assert_eq!(t.mult(p), point(-2.0, 3.0, 4.0));
}

#[test]
fn rotate_around_x_axis() {
    let p = point(0.0, 1.0, 0.0);
    let eighth_turn = rotation_x(FRAC_PI_4);  // pi / 4
    let quarter_turn = rotation_x(FRAC_PI_2);  // pi / 2

    assert_eq!(eighth_turn.mult(p) , point(0.0, FRAC_1_SQRT_2, FRAC_1_SQRT_2));
    assert_eq!(quarter_turn.mult(p), point(0.0, 0.0, 1.0));
}


#[test]
fn rotate_around_y_axis() {
    let p = point(0.0, 0.0, 1.0);
    let eighth_turn = rotation_y(FRAC_PI_4);  // pi / 4
    let quarter_turn = rotation_y(FRAC_PI_2);  // pi / 2

    assert_eq!(eighth_turn.mult(p) , point(FRAC_1_SQRT_2, 0.0, FRAC_1_SQRT_2));
    assert_eq!(quarter_turn.mult(p), point(1.0, 0.0, 0.0));
}

#[test]
fn rotate_around_z_axis() {
    let p = point(1.0, 0.0, 0.0);
    let eighth_turn = rotation_z(FRAC_PI_4);  // pi / 4
    let quarter_turn = rotation_z(FRAC_PI_2);  // pi / 2

    assert_eq!(eighth_turn.mult(p) , point(FRAC_1_SQRT_2, FRAC_1_SQRT_2, 0.0));
    assert_eq!(quarter_turn.mult(p), point(0.0, 1.0, 0.0));
}



