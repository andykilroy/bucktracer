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

#[test]
fn ray_intersects_sphere() {
    let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
    let s = unit_sphere();
    let xs = intersect(&r, &s);
    assert_eq!(xs[0], 4.0);
    assert_eq!(xs[1], 6.0);
}

#[test]
fn ray_intersects_sphere_at_tangent() {
    let r = ray(point(0.0, 1.0, -5.0), vector(0.0, 0.0, 1.0));
    let s = unit_sphere();
    let xs = intersect(&r, &s);
    assert_eq!(xs[0], 5.0);
    assert_eq!(xs[1], 5.0);
}

#[test]
fn ray_misses_sphere() {
    let r = ray(point(0.0, 2.0, -5.0), vector(0.0, 0.0, 1.0));
    let s = unit_sphere();
    let xs = intersect(&r, &s);
    assert_eq!(xs.len(), 0);
}

#[test]
fn ray_originates_inside_sphere() {
    let r = ray(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
    let s = unit_sphere();
    let xs = intersect(&r, &s);
    assert_eq!(xs[0], -1.0);
    assert_eq!(xs[1],  1.0);
}

#[test]
fn ray_originates_in_front_of_sphere() {
    let r = ray(point(0.0, 0.0, 5.0), vector(0.0, 0.0, 1.0));
    let s = unit_sphere();
    let xs = intersect(&r, &s);
    assert_eq!(xs[0], -6.0);
    assert_eq!(xs[1], -4.0);
}


