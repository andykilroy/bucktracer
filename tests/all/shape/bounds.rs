use bucktracer::*;
use bucktracer::math::*;

#[allow(non_snake_case)]
#[test]
fn sphere_bounds() {
    let s = unit_sphere();
    assert_eq!(point(-1.0, -1.0, -1.0), s.bounds().min());
    assert_eq!(point(1.0, 1.0, 1.0), s.bounds().max());
}

#[allow(non_snake_case)]
#[test]
fn cube_bounds() {
    let c = cube();
    assert_eq!(point(-1.0, -1.0, -1.0), c.bounds().min());
    assert_eq!(point(1.0, 1.0, 1.0), c.bounds().max());
}


#[allow(non_snake_case)]
#[test]
fn cube_bounds_determined_by_transformation() {
    let root_2 = std::f64::consts::SQRT_2;
    let c1 = cube().set_object_to_world_spc(rotation_z(std::f64::consts::FRAC_PI_4)).clone();
    let b = c1.bounds();
    assert_eq!(b.min(), point(-root_2, -root_2, -1.0));
    assert_eq!(b.max(), point(root_2, root_2, 1.0));
}

#[allow(non_snake_case)]
#[test]
fn finite_cylinder_bounds() {
    scenario_inf_cylinder(CylKind::Closed, -2.0, 3.0);
    scenario_inf_cylinder(CylKind::Open, -2.0, 3.0);
}

#[allow(non_snake_case)]
#[test]
fn inf_cylinder_bounds() {
    let ninf = std::f64::NEG_INFINITY;
    let inf = std::f64::INFINITY;
    scenario_inf_cylinder(CylKind::Closed, ninf, 1.0);
    scenario_inf_cylinder(CylKind::Closed, 2.0, inf);
    scenario_inf_cylinder(CylKind::Open, ninf, 1.0);
    scenario_inf_cylinder(CylKind::Open, 2.0, inf);
}

fn scenario_inf_cylinder(kind: CylKind, lbound: f64, ubound: f64) {
    let c = cylinder(kind, lbound, ubound);
    assert_eq!(-1.0, c.bounds().min().x());
    assert_eq!(-1.0, c.bounds().min().z());
    assert_eq!(1.0, c.bounds().max().x());
    assert_eq!(1.0, c.bounds().max().z());
    assert_eq!(lbound, c.bounds().min().y());
    assert_eq!(ubound, c.bounds().max().y());
}

#[allow(non_snake_case)]
#[test]
fn plane_bounds() {
    let neg_inf = std::f64::NEG_INFINITY;
    let pos_inf = std::f64::INFINITY;

    let p = plane();
    assert_eq!(p.bounds().min().x(), neg_inf);
    assert_eq!(p.bounds().min().y(), neg_inf);
    assert_eq!(p.bounds().min().z(), neg_inf);
    assert_eq!(p.bounds().max().x(), pos_inf);
    assert_eq!(p.bounds().max().y(), pos_inf);
    assert_eq!(p.bounds().max().z(), pos_inf);
}

#[allow(non_snake_case)]
#[test]
fn group_bounds_is_origin_if_no_members() {
    let g = group(vec![]);
    let neg_inf = std::f64::NEG_INFINITY;
    let pos_inf = std::f64::INFINITY;

    assert_bounds(
        g.bounds(),
        (pos_inf, pos_inf, pos_inf),
        (neg_inf, neg_inf, neg_inf),
    );
}

#[allow(non_snake_case)]
#[test]
fn group_minima_and_maxima_are_dictated_by_its_members_bounds() {
    assert_bounds(
        group(vec![unit_sphere(), cylinder(CylKind::Open, -5.0, 2.0)]).bounds(),
        (-1.0, -5.0, -1.0),
        (1.0, 2.0, 1.0),
    );
    assert_bounds(
        group(vec![unit_sphere(), cube()]).bounds(),
        (-1.0, -1.0, -1.0),
        (1.0, 1.0, 1.0),
    );
}

#[allow(non_snake_case)]
#[test]
fn group_bounds_encompasses_all_members() {
    let mut all = vec![unit_sphere()];
    assert_bounds(group(all.clone()).bounds(), (-1.0, -1.0, -1.0), (1.0, 1.0, 1.0));

    all.push(cylinder(CylKind::Open, -5.0, 2.0));
    assert_bounds(group(all.clone()).bounds(), (-1.0, -5.0, -1.0), (1.0, 2.0, 1.0));

    all.push(cube().set_object_to_world_spc(scaling(8.0, 1.0, 1.0)).clone());
    assert_bounds(group(all.clone()).bounds(), (-8.0, -5.0, -1.0), (8.0, 2.0, 1.0));

    all.push(cube().set_object_to_world_spc(translation(0.0, 0.0, 200.0)).clone());
    assert_bounds(group(all.clone()).bounds(), (-8.0, -5.0, -1.0), (8.0, 2.0, 201.0));
}

#[allow(non_snake_case)]
#[test]
fn bounds_in_group_affected_by_nested_group_transform() {
    let root_2 = std::f64::consts::SQRT_2;
    let pi_4 = std::f64::consts::FRAC_PI_4;
    let pi_2 = std::f64::consts::FRAC_PI_2;
    let c1 = cube().set_object_to_world_spc(rotation_z(pi_4)).clone();
    let g2 = group(vec![c1.clone()]).set_object_to_world_spc(translation(0.0, 5.0, 0.0)).clone();
    let g1 = group(vec![g2.clone()]).set_object_to_world_spc(rotation_z(pi_2)).clone();

    approx_bounds(c1.bounds(), (-root_2, -root_2, -1.0), (root_2, root_2, 1.0));
    approx_bounds(g2.bounds(), (-root_2, -root_2 + 5.0, -1.0), (root_2, root_2 + 5.0, 1.0));
    approx_bounds(g1.bounds(), (-root_2 - 5.0, -root_2, -1.0), (root_2 - 5.0, root_2, 1.0));
}

#[allow(non_snake_case)]
#[test]
fn two_groups_and_intersect_them() {
    let c1 = cube().set_object_to_world_spc(translation(4.0, 0.0, 0.0) * rotation_z(std::f64::consts::FRAC_PI_4)).clone();
    let c2 = cube().set_object_to_world_spc(translation(0.0, 4.0, 0.0) * rotation_z(std::f64::consts::FRAC_PI_4)).clone();

    let g1 = group(vec![c1.clone()]).set_object_to_world_spc(translation(0.0, -3.0, 0.0)).clone();
    let g2 = group(vec![c2.clone()]).set_object_to_world_spc(translation(-3.0, 0.0, 0.0)).clone();

    let w = World::with(vec![], vec![g1, g2]);
    let ints = w.intersect(&ray(point(0.0, -3.0, 0.0), vector(1.0, 0.0, 0.0)));

    assert_eq!(ints.len(), 2);
    assert_eq!(ints[0].t_value(), 4.0 - std::f64::consts::SQRT_2);
    assert_eq!(ints[0].intersected(), c1.clone());

    assert_eq!(ints[1].t_value(), 4.0 + std::f64::consts::SQRT_2);
    assert_eq!(ints[1].intersected(), c1.clone());
}


fn assert_bounds(b: Bounds, min: (f64, f64, f64), max: (f64, f64, f64)) {
    assert_eq!(b.min().x(), min.0);
    assert_eq!(b.min().y(), min.1);
    assert_eq!(b.min().z(), min.2);

    assert_eq!(b.max().x(), max.0);
    assert_eq!(b.max().y(), max.1);
    assert_eq!(b.max().z(), max.2);
}

fn approx_bounds(b: Bounds, min: (f64, f64, f64), max: (f64, f64, f64)) {
    assert_eq!(b.min(), point(min.0, min.1, min.2));
    assert_eq!(b.max(), point(max.0, max.1, max.2));
}
