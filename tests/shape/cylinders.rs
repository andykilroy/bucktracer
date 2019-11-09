use bucktracer::*;
use bucktracer::CylKind::{Open, Closed};
use std::f64::{INFINITY, NEG_INFINITY};
use crate::almost_eq;

#[allow(non_snake_case)]
#[test]
fn rays_miss_a_cylinder() {
    scenario_rays_miss_a_cylinder(point(1.0, 0.0, 0.0), vector(0.0, 1.0, 0.0));
    scenario_rays_miss_a_cylinder(point(0.0, 0.0, 0.0), vector(0.0, 1.0, 0.0));
    scenario_rays_miss_a_cylinder(point(0.0, 0.0, -5.0), vector(1.0, 1.0, 1.0));
}

fn scenario_rays_miss_a_cylinder(origin: Tuple4, direction: Tuple4) {
    let c = inf_cylinder();
    let r = ray(origin, direction.normalize());
    let mut v = vec![];
    append_intersects(&r, &c, &mut v);
    assert_eq!(0, v.len());
}

#[allow(non_snake_case)]
#[test]
fn ray_hits_a_cylinder() {
    scenario_ray_hits_a_cylinder(point(1.0, 0.0, -5.0), vector(0.0, 0.0, 1.0), 5.0, 5.0);
    scenario_ray_hits_a_cylinder(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0), 4.0, 6.0);
    scenario_ray_hits_a_cylinder(point(0.5, 0.0, -5.0), vector(0.1, 1.0, 1.0), 6.80798, 7.08872);
}

fn scenario_ray_hits_a_cylinder(origin: Tuple4, direction: Tuple4, t0: f64, t1:f64) {
    let c = inf_cylinder();
    let r = ray(origin, direction.normalize());
    let mut v = vec![];
    append_intersects(&r, &c, &mut v);
    assert_eq!(2, v.len());
    assert_eq!(true, almost_eq(t0, v[0].t_value));
    assert_eq!(true, almost_eq(t1, v[1].t_value));
}

#[allow(non_snake_case)]
#[test]
fn normal_on_a_cylinder() {
    scenario_normal_on_an_inf_cylinder(point(1.0, 0.0, 0.0), vector(1.0, 0.0, 0.0));
    scenario_normal_on_an_inf_cylinder(point(0.0, 5.0, -1.0), vector(0.0, 0.0, -1.0));
    scenario_normal_on_an_inf_cylinder(point(0.0, -2.0, 1.0), vector(0.0, 0.0, 1.0));
    scenario_normal_on_an_inf_cylinder(point(-1.0, 1.0, 0.0), vector(-1.0, 0.0, 0.0));
}

fn scenario_normal_on_an_inf_cylinder(origin: Tuple4, expected: Tuple4) {
    let c = inf_cylinder();
    let n = c.normal_at(origin);
    assert_eq!(n, expected);
}

fn scenario_normal_on_a_closed_cylinder(origin: Tuple4, expected: Tuple4) {
    let c = cylinder(Closed, 1.0, 2.0);
    let n = c.normal_at(origin);
    assert_eq!(n, expected);
}

#[allow(non_snake_case)]
#[test]
fn cylinder___default_extents_are_infinite() {
    assert_eq!(inf_cylinder(), cylinder(Open, NEG_INFINITY, INFINITY));
    assert_ne!(inf_cylinder(), cylinder(Open, NEG_INFINITY, 6.0));
    assert_ne!(inf_cylinder(), cylinder(Open, -1.0, INFINITY));
}

#[allow(non_snake_case)]
#[test]
fn cylinder___infinite_cylinders_are_not_closed() {
    assert_ne!(inf_cylinder(), cylinder(Closed, NEG_INFINITY, INFINITY));
}

#[allow(non_snake_case)]
#[test]
fn cylinder___with_same_limits___compare_equal() {
    let ninf = NEG_INFINITY;
    let inf = INFINITY;
    assert_eq!(cylinder(Open, ninf, inf), cylinder(Open, ninf, inf));
    assert_eq!(cylinder(Open,  6.0, 7.0), cylinder(Open,  6.0, 7.0));
    assert_eq!(cylinder(Open,  6.0, inf), cylinder(Open,  6.0, inf));
    assert_eq!(cylinder(Open, -9.0, 1.0), cylinder(Open, -9.0, 1.0));

    assert_eq!(cylinder(Closed, ninf, inf), cylinder(Closed, ninf, inf));
    assert_eq!(cylinder(Closed,  6.0, 7.0), cylinder(Closed,  6.0, 7.0));
    assert_eq!(cylinder(Closed,  6.0, inf), cylinder(Closed,  6.0, inf));
    assert_eq!(cylinder(Closed, -9.0, 1.0), cylinder(Closed, -9.0, 1.0));
}

#[allow(non_snake_case)]
#[test]
fn cylinder___differ_only_by_closed_are_not_equal() {
    assert_ne!(cylinder(Open,  6.0, 7.0), cylinder(Closed,  6.0, 7.0));
}

#[allow(non_snake_case)]
#[test]
fn cylinder___with_different_limits___are_not_equal() {
    assert_ne!(cylinder(Open, 6.0, 7.0), cylinder(Open, 6.0, 8.0));
    assert_ne!(cylinder(Open, 6.0, 7.0), cylinder(Open, -10.0, 1.0));
    assert_ne!(cylinder(Open, 6.0, 7.0), cylinder(Open, 0.0, 7.0));
}

#[allow(non_snake_case)]
#[test]
fn cylinder___when_ray_hits_between_limits___is_intersected() {
    count_intersects(Open, point(0.0, 1.5, 0.0),  vector(0.1, 1.0, 0.0), 0);
    count_intersects(Open, point(0.0, 3.0, -5.0), vector(0.0, 0.0, 1.0), 0);
    count_intersects(Open, point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0), 0);
    count_intersects(Open, point(0.0, 2.0, -5.0), vector(0.0, 0.0, 1.0), 0);
    count_intersects(Open, point(0.0, 1.0, -5.0), vector(0.0, 0.0, 1.0), 0);
    count_intersects(Open, point(0.0, 1.5, -2.0), vector(0.0, 0.0, 1.0), 2);
}

fn count_intersects(closed: CylKind, pos: Tuple4, direction: Tuple4, count: usize) {
    let c = cylinder(closed, 1.0, 2.0);
    let dir = direction.normalize();
    let r = ray(pos, dir);
    let mut xs = vec![];
    append_intersects(&r, &c, &mut xs);
    assert_eq!(count, xs.len())
}

#[allow(non_snake_case)]
#[test]
fn cylinder___intersect_end_caps() {
    count_intersects(Closed, point(0.0, 3.0, 0.0), vector(0.0,-1.0, 0.0), 2);
    count_intersects(Closed, point(0.0, 3.0,-2.0), vector(0.0,-1.0, 2.0), 2);
    count_intersects(Closed, point(0.0, 4.0,-2.0), vector(0.0,-1.0, 1.0), 2);
    count_intersects(Closed, point(0.0, 0.0,-2.0), vector(0.0, 1.0, 2.0), 2);
    count_intersects(Closed, point(0.0,-1.0,-2.0), vector(0.0, 1.0, 1.0), 2);
}

#[allow(non_snake_case)]
#[test]
fn cylinder___normal_on_end_caps() {
    scenario_normal_on_a_closed_cylinder(point(0.0, 1.0, 0.0), vector(0.0,-1.0, 0.0));
    scenario_normal_on_a_closed_cylinder(point(0.5, 1.0, 0.0), vector(0.0,-1.0, 0.0));
    scenario_normal_on_a_closed_cylinder(point(0.0, 1.0, 0.5), vector(0.0,-1.0, 0.0));
    scenario_normal_on_a_closed_cylinder(point(0.0, 2.0, 0.0), vector(0.0, 1.0, 0.0));
    scenario_normal_on_a_closed_cylinder(point(0.5, 2.0, 0.0), vector(0.0, 1.0, 0.0));
    scenario_normal_on_a_closed_cylinder(point(0.0, 2.0, 0.5), vector(0.0, 1.0, 0.0));
}
