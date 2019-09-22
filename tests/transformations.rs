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
    let eighth_turn = rotation_x(FRAC_PI_4); // pi / 4
    let quarter_turn = rotation_x(FRAC_PI_2); // pi / 2

    assert_eq!(
        eighth_turn.mult(p),
        point(0.0, FRAC_1_SQRT_2, FRAC_1_SQRT_2)
    );
    assert_eq!(quarter_turn.mult(p), point(0.0, 0.0, 1.0));
}

#[test]
fn rotate_around_y_axis() {
    let p = point(0.0, 0.0, 1.0);
    let eighth_turn = rotation_y(FRAC_PI_4); // pi / 4
    let quarter_turn = rotation_y(FRAC_PI_2); // pi / 2

    assert_eq!(
        eighth_turn.mult(p),
        point(FRAC_1_SQRT_2, 0.0, FRAC_1_SQRT_2)
    );
    assert_eq!(quarter_turn.mult(p), point(1.0, 0.0, 0.0));
}

#[test]
fn rotate_around_z_axis() {
    let p = point(1.0, 0.0, 0.0);
    let eighth_turn = rotation_z(FRAC_PI_4); // pi / 4
    let quarter_turn = rotation_z(FRAC_PI_2); // pi / 2

    assert_eq!(
        eighth_turn.mult(p),
        point(FRAC_1_SQRT_2, FRAC_1_SQRT_2, 0.0)
    );
    assert_eq!(quarter_turn.mult(p), point(0.0, 1.0, 0.0));
}

#[test]
fn compose_transforms() {
    let pi_by_2 = FRAC_PI_2;
    let p = point(1.0, 0.0, 1.0);
    let t = identity()
        .rotate_x(pi_by_2)
        .scale(5.0, 5.0, 5.0)
        .translate(10.0, 5.0, 7.0);
    assert_eq!(t.mult(p), point(15.0, 0.0, 7.0));
}

#[test]
fn transformation_matrix_for_default_orientation() {
    let from = point(0.0, 0.0, 0.0);
    let to = point(0.0, 0.0, -1.0);
    let up = vector(0.0, 1.0, 0.0);
    let t = view_transform(from, to, up);
    assert_eq!(t, identity());
}

#[test]
fn a_view_towards_the_positive_z_direction() {
    let from = point(0.0, 0.0, 0.0);
    let to = point(0.0, 0.0, 1.0);
    let up = vector(0.0, 1.0, 0.0);
    let t = view_transform(from, to, up);
    assert_eq!(t, scaling(-1.0, 1.0, -1.0));
}

#[test]
fn view_transform_moves_the_world() {
    let from = point(0.0, 0.0, 8.0);
    let to = point(0.0, 0.0, 0.0);
    let up = vector(0.0, 1.0, 0.0);
    let t = view_transform(from, to, up);
    assert_eq!(t, translation(0.0, 0.0, -8.0));
}

#[test]
fn arbitrary_view_transform() {
    let from = point(1.0, 3.0, 2.0);
    let to = point(4.0, -2.0, 8.0);
    let up = vector(1.0, 1.0, 0.0);
    let t = view_transform(from, to, up);
    assert_eq!(
        t,
        matrix(
            (-0.50709, 0.50709, 0.67612, -2.36643),
            (0.76772, 0.60609, 0.12122, -2.82843),
            (-0.35857, 0.59761, -0.71714, 0.00000),
            (0.00000, 0.00000, 0.00000, 1.00000)
        )
    );
}

#[test]
fn construct_camera() {
    let hsize = 160;
    let vsize = 120;
    let fov = PI / 2.0;
    let c = Camera::new(hsize, vsize, fov);
    assert_eq!(c.hsize(), 160);
    assert_eq!(c.vsize(), 120);
    assert_eq!(c.field_of_view(), fov);
    assert_eq!(c.view_transform(), identity());
}

fn almost_eq(x1: f64, x2: f64) -> bool {
    f64::abs(x1 - x2) < 0.000001
}

#[test]
fn pixel_size_for_horizontal_canvas() {
    let c = Camera::new(200, 125, FRAC_PI_2);
    assert_eq!(true, almost_eq(c.pixel_size(), 0.01));
}

#[test]
fn pixel_size_for_vertical_canvas() {
    let c = Camera::new(125, 200, FRAC_PI_2);
    assert_eq!(true, almost_eq(c.pixel_size(), 0.01));
}

#[test]
fn ray_thru_centre_of_canvas() {
    let c = Camera::new(201, 101, FRAC_PI_2);
    let r = c.ray_for_pixel(100, 50);
    assert_eq!(r.origin, point(0.0, 0.0, 0.0));
    assert_eq!(r.direction, vector(0.0, 0.0, -1.0));
}

#[test]
fn ray_thru_corner_of_canvas() {
    let c = Camera::new(201, 101, FRAC_PI_2);
    let r = c.ray_for_pixel(0, 0);
    assert_eq!(r.origin, point(0.0, 0.0, 0.0));
    assert_eq!(r.direction, vector(0.66519, 0.33259, -0.66851));
}

#[test]
fn ray_when_camera_transformed() {
    let mut c = Camera::new(201, 101, FRAC_PI_2);
    c.set_view_transform(rotation_y(FRAC_PI_4) * translation(0.0, -2.0, 5.0));
    let r = c.ray_for_pixel(100, 50);
    assert_eq!(r.origin, point(0.0, 2.0, -5.0));
    assert_eq!(r.direction, vector(SQRT_2 / 2.0, 0.0, -SQRT_2 / 2.0));
}
