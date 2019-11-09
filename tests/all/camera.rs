use bucktracer::*;
use std::f64::consts::*;
use crate::almost_eq;

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

