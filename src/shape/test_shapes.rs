use crate::*;
use std::f64::consts::FRAC_PI_2;

#[allow(non_snake_case)]
#[test]
fn shape___local_normal_of_a_plane_is_up() {
    let p: Shape = Shape::Plane;
    let n1 = p.local_normal_at(point(0.0, 0.0, 0.0));
    let n2 = p.local_normal_at(point(10.0, 0.0, -10.0));
    let n3 = p.local_normal_at(point(-5.0, 0.0, 150.0));

    assert_eq!(n1, vector(0.0, 1.0, 0.0));
    assert_eq!(n2, vector(0.0, 1.0, 0.0));
    assert_eq!(n3, vector(0.0, 1.0, 0.0));
}

#[allow(non_snake_case)]
#[test]
fn object___normal_of_a_plane_is_constant_everywhere() {
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
fn ray_intersects_a_cube() {
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
fn ray_misses_a_cube() {
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
    scenario_normal_of_a_cube(point(1.0, 0.5, -0.8), vector(1.0, 0.0, 0.0));
    scenario_normal_of_a_cube(point(-1.0, -0.2, 0.9), vector(-1.0, 0.0, 0.0));
    scenario_normal_of_a_cube(point(-0.4, 1.0, -0.1), vector(0.0, 1.0, 0.0));
    scenario_normal_of_a_cube(point(0.3, -1.0, -0.7), vector(0.0, -1.0, 0.0));
    scenario_normal_of_a_cube(point(-0.6, 0.3, 1.0), vector(0.0, 0.0, 1.0));
    scenario_normal_of_a_cube(point(0.4, 0.4, -1.0), vector(0.0, 0.0, -1.0));
    scenario_normal_of_a_cube(point(1.0, 1.0, 1.0), vector(1.0, 0.0, 0.0));
    scenario_normal_of_a_cube(point(-1.0, -1.0, -1.0), vector(-1.0, 0.0, 0.0));
}

fn scenario_normal_of_a_cube(pos: Tuple4, normal: Tuple4)
{
    let c = cube();
    let n = c.normal_at(pos);
    assert_eq!(normal, n);
}

