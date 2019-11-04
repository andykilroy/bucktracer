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

    assert_eq!(r.position(0.0), point(2.0, 3.0, 4.0));
    assert_eq!(r.position(1.0), point(3.0, 3.0, 4.0));
    assert_eq!(r.position(-1.0), point(1.0, 3.0, 4.0));
    assert_eq!(r.position(2.5), point(4.5, 3.0, 4.0));
}

#[test]
fn ray_intersects_sphere() {
    let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
    let s = unit_sphere();
    let mut xs = vec![];
    append_intersects(&r, &s, &mut xs);
    assert_eq!(xs[0].t_value, 4.0);
    assert_eq!(xs[1].t_value, 6.0);
}

#[test]
fn ray_intersects_sphere_at_tangent() {
    let r = ray(point(0.0, 1.0, -5.0), vector(0.0, 0.0, 1.0));
    let s = unit_sphere();
    let mut xs = vec![];
    append_intersects(&r, &s, &mut xs);
    assert_eq!(xs[0].t_value, 5.0);
    assert_eq!(xs[1].t_value, 5.0);
}

#[test]
fn ray_misses_sphere() {
    let r = ray(point(0.0, 2.0, -5.0), vector(0.0, 0.0, 1.0));
    let s = unit_sphere();
    let mut xs = vec![];
    append_intersects(&r, &s, &mut xs);
    assert_eq!(xs.len(), 0);
}

#[test]
fn ray_originates_inside_sphere() {
    let r = ray(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
    let s = unit_sphere();
    let mut xs = vec![];
    append_intersects(&r, &s, &mut xs);
    assert_eq!(xs[0].t_value, -1.0);
    assert_eq!(xs[1].t_value, 1.0);
}

#[test]
fn ray_originates_in_front_of_sphere() {
    let r = ray(point(0.0, 0.0, 5.0), vector(0.0, 0.0, 1.0));
    let s = unit_sphere();
    let mut xs = vec![];
    append_intersects(&r, &s, &mut xs);
    assert_eq!(xs[0].t_value, -6.0);
    assert_eq!(xs[1].t_value, -4.0);
}

#[test]
fn create_intersection() {
    let s = unit_sphere();
    let i = intersection(3.5, &s);
    assert_eq!(s, i.intersected);
    assert_eq!(3.5, i.t_value);
}

#[test]
fn finding_hits_when_t_values_are_positive() {
    let s = unit_sphere();
    let i1 = intersection(1.0, &s);
    let i2 = intersection(2.0, &s);
    let xs = vec![i1.clone(), i2.clone()];
    assert_eq!(Some(i1), hit(xs));
}

#[test]
fn finding_hits_when_some_intersections_have_negative_t() {
    let s = unit_sphere();
    let i1 = intersection(-1.0, &s);
    let i2 = intersection(1.0, &s);
    let xs = vec![i1.clone(), i2.clone()];
    assert_eq!(Some(i2), hit(xs));
}

#[test]
fn finding_hits_when_all_intersections_have_negative_t() {
    let s = unit_sphere();
    let i1 = intersection(-2.0, &s);
    let i2 = intersection(-1.0, &s);
    let xs = vec![i1, i2];
    assert_eq!(None, hit(xs));
}

#[test]
fn finding_hits_always_the_lowest_non_negative_t() {
    let s = unit_sphere();
    let i1 = intersection(5.0, &s);
    let i2 = intersection(7.0, &s);
    let i3 = intersection(-3.0, &s);
    let i4 = intersection(2.0, &s);
    let xs = vec![i1, i2, i3, i4.clone()];
    assert_eq!(Some(i4), hit(xs));
}

#[test]
fn translating_a_ray() {
    let r = ray(point(1.0, 2.0, 3.0), vector(0.0, 1.0, 0.0));
    let m = translation(3.0, 4.0, 5.0);
    let r2 = r.transform(&m);

    assert_eq!(r2.origin, point(4.0, 6.0, 8.0));
    assert_eq!(r2.direction, vector(0.0, 1.0, 0.0));
}

#[test]
fn scaling_a_ray() {
    let r = ray(point(1.0, 2.0, 3.0), vector(0.0, 1.0, 0.0));
    let m = scaling(2.0, 3.0, 4.0);
    let r2 = r.transform(&m);

    assert_eq!(r2.origin, point(2.0, 6.0, 12.0));
    assert_eq!(r2.direction, vector(0.0, 3.0, 0.0));
}

#[test]
fn intersect_scaled_sphere_with_a_ray() {
    let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
    let mut s = unit_sphere();
    s.set_object_to_world_spc(scaling(2.0, 2.0, 2.0));
    let mut xs = vec![];
    append_intersects(&r, &s, &mut xs);
    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0].t_value, 3.0);
    assert_eq!(xs[1].t_value, 7.0);
}

#[test]
fn intersect_translated_sphere_with_a_ray_miss() {
    let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
    let mut s = unit_sphere();
    s.set_object_to_world_spc(translation(5.0, 0.0, 0.0));
    let mut xs = vec![];
    append_intersects(&r, &s, &mut xs);
    assert_eq!(xs.len(), 0);
}

#[test]
fn intersect_translated_sphere_with_a_ray_hit() {
    let r = ray(point(5.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
    let mut s = unit_sphere();
    s.set_object_to_world_spc(translation(5.0, 0.0, 0.0));
    let mut xs = vec![];
    append_intersects(&r, &s, &mut xs);
    assert_eq!(xs.len(), 2);
}

// Intersections with planes

#[test]
fn a_ray_parallel_to_the_plane_produces_no_intersects() {
    let p = plane();
    let r = ray(point(0.0, 10.0, 0.0), vector(0.0, 0.0, 1.0));
    let mut xs = vec![];
    append_intersects(&r, &p, &mut xs);
    assert_eq!(0, xs.len());
}

#[test]
fn a_coplanar_ray_produces_no_intersects() {
    let p = plane();
    let r = ray(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
    let mut xs = vec![];
    append_intersects(&r, &p, &mut xs);
    assert_eq!(xs.len(), 0);
}

#[test]
fn a_ray_intersects_a_plane_from_above() {
    let p = plane();
    let r = ray(point(0.0, 1.0, 0.0), vector(0.0, -1.0, 0.0));
    let mut xs = vec![];
    append_intersects(&r, &p, &mut xs);
    assert_eq!(xs.len(), 1);
    assert_eq!(xs[0].t_value, 1.0);
    assert_eq!(xs[0].intersected, p);
}

#[test]
fn a_ray_intersects_a_plane_from_below() {
    let p = plane();
    let r = ray(point(0.0, -1.0, 0.0), vector(0.0, 1.0, 0.0));
    let mut xs = vec![];
    append_intersects(&r, &p, &mut xs);
    assert_eq!(xs.len(), 1);
    assert_eq!(xs[0].t_value, 1.0);
    assert_eq!(xs[0].intersected, p);
}
