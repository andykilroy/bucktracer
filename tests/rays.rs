use bucktracer::*;

#[test]
fn create_ray() {
    let r = ray(point(1.0, 2.0, 3.0), vector(4.0, 5.0, 6.0));
    assert_eq!(r.origin, point(1.0, 2.0, 3.0));
    assert_eq!(r.direction, vector(4.0, 5.0, 6.0));
}

#[test]
fn calc_position() {
    let p = point(2.0, 3.0, 4.0);
    let v = vector(1.0, 0.0, 0.0);
    let r = ray(p, v);

    assert_eq!(position(r.clone(),  0.0), point(2.0, 3.0, 4.0));
    assert_eq!(position(r.clone(),  1.0), point(3.0, 3.0, 4.0));
    assert_eq!(position(r.clone(), -1.0), point(1.0, 3.0, 4.0));
    assert_eq!(position(r.clone(),  2.5), point(4.5, 3.0, 4.0));
}

