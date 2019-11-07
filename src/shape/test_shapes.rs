use crate::*;
use std::f64::consts::FRAC_PI_2;
use std::f64::{INFINITY, NEG_INFINITY};
use crate::CylKind::{Open, Closed};

#[allow(non_snake_case)]
#[test]
fn normal_of_untransformed_plane___is_up() {
    let mut p: Object = plane();
    p.set_object_to_world_spc(identity());
    let n1 = p.normal_at(point(0.0, 0.0, 0.0));
    let n2 = p.normal_at(point(10.0, 0.0, -10.0));
    let n3 = p.normal_at(point(-5.0, 0.0, 150.0));

    assert_eq!(n1, vector(0.0, 1.0, 0.0));
    assert_eq!(n2, vector(0.0, 1.0, 0.0));
    assert_eq!(n3, vector(0.0, 1.0, 0.0));
}

#[allow(non_snake_case)]
#[test]
fn after_transformation___normal_of_a_plane_is_constant_everywhere() {
    let mut p: Object = plane();
    p.set_object_to_world_spc(rotation_z(FRAC_PI_2));
    let n1 = p.normal_at(point(0.0, 0.0, 0.0));
    let n2 = p.normal_at(point(10.0, 0.0, -10.0));
    let n3 = p.normal_at(point(-5.0, 0.0, 150.0));

    assert_eq!(n1, vector(-1.0, 0.0, 0.0));
    assert_eq!(n2, vector(-1.0, 0.0, 0.0));
    assert_eq!(n3, vector(-1.0, 0.0, 0.0));
}

#[allow(non_snake_case)]
#[test]
fn ray_intersects_unit_cube___at_faces_1_unit_away_from_origin() {
    scenario_ray_intersects_cube("+x", point(5.0, 0.5, 0.0), vector(-1.0, 0.0, 0.0), 4.0, 6.0);
    scenario_ray_intersects_cube("+y", point(0.5, 5.0, 0.0), vector(0.0, -1.0, 0.0), 4.0, 6.0);
    scenario_ray_intersects_cube("+z", point(0.5, 0.0, 5.0), vector(0.0, 0.0, -1.0), 4.0, 6.0);

    scenario_ray_intersects_cube("-x", point(-5.0, 0.5, 0.0), vector(1.0, 0.0, 0.0), 4.0, 6.0);
    scenario_ray_intersects_cube("-y", point(0.5, -5.0, 0.0), vector(0.0, 1.0, 0.0), 4.0, 6.0);
    scenario_ray_intersects_cube("-z", point(0.5, 0.0, -5.0), vector(0.0, 0.0, 1.0), 4.0, 6.0);

    scenario_ray_intersects_cube("inside", point(0.0, 0.5, 0.0), vector(0.0, 0.0, 1.0), -1.0, 1.0);
}

fn scenario_ray_intersects_cube(_s: &str, pos: Tuple4, dir: Tuple4, t1: f64, t2: f64) {
    let c = cube();
    let r = ray(pos, dir);
    let mut xs: Vec<Intersection> = vec![];
    append_intersects(&r, &c, &mut xs);
    assert_eq!(2, xs.len());
    assert_eq!(t1, xs[0].t_value);
    assert_eq!(t2, xs[1].t_value);
}

fn scenario_ray_misses_cube(pos: Tuple4, dir: Tuple4) {
    let c = cube();
    let r = ray(pos, dir);
    let mut xs: Vec<Intersection> = vec![];
    append_intersects(&r, &c, &mut xs);
    assert_eq!(0, xs.len());
}


#[allow(non_snake_case)]
#[test]
fn when_rays_miss_a_cube___return_no_intersections() {
    scenario_ray_misses_cube(point(-2.0, 0.0, 0.0), vector(0.2673, 0.5345, 0.8018));
    scenario_ray_misses_cube(point( 0.0, -2.0, 0.0), vector(0.8018, 0.2673, 0.5345));
    scenario_ray_misses_cube(point( 0.0, 0.0, -2.0), vector(0.5345, 0.8018, 0.2673));
    scenario_ray_misses_cube(point( 2.0, 0.0, 2.0), vector(0.0, 0.0, -1.0));
    scenario_ray_misses_cube(point( 0.0, 2.0, 2.0), vector(0.0, -1.0, 0.0));
    scenario_ray_misses_cube(point( 2.0, 2.0, 0.0), vector(-1.0, 0.0, 0.0));
}

#[allow(non_snake_case)]
#[test]
fn normal_on_a_cube() {
    scenario_normal_on_a_cube(point(1.0, 0.5, -0.8), vector(1.0, 0.0, 0.0));
    scenario_normal_on_a_cube(point(-1.0, -0.2, 0.9), vector(-1.0, 0.0, 0.0));
    scenario_normal_on_a_cube(point(-0.4, 1.0, -0.1), vector(0.0, 1.0, 0.0));
    scenario_normal_on_a_cube(point(0.3, -1.0, -0.7), vector(0.0, -1.0, 0.0));
    scenario_normal_on_a_cube(point(-0.6, 0.3, 1.0), vector(0.0, 0.0, 1.0));
    scenario_normal_on_a_cube(point(0.4, 0.4, -1.0), vector(0.0, 0.0, -1.0));
    scenario_normal_on_a_cube(point(1.0, 1.0, 1.0), vector(1.0, 0.0, 0.0));
    scenario_normal_on_a_cube(point(-1.0, -1.0, -1.0), vector(-1.0, 0.0, 0.0));
}

fn scenario_normal_on_a_cube(pos: Tuple4, normal: Tuple4) {
    let c = cube();
    let n = c.normal_at(pos);
    assert_eq!(normal, n);
}

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

#[allow(non_snake_case)]
#[test]
fn group___create() {
    let grp = group(vec![]);
    assert_eq!(identity(), grp.object_to_world_spc());
}

#[allow(non_snake_case)]
#[test]
fn group___create_with_children() {
    let shapes = vec![unit_sphere(), inf_cylinder()];
    let grp = group(shapes.clone());
    assert_eq!(shapes, grp.children().to_vec());
}

#[allow(non_snake_case)]
#[test]
fn group___allow_nested_groups() {
    let childgroup = group(vec![cube()]);
    let shapes = vec![unit_sphere(), inf_cylinder(), childgroup];
    let grp = group(shapes.clone());
    assert_eq!(shapes, grp.children().to_vec());
}
