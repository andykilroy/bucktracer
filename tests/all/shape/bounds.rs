use bucktracer::*;

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
    assert_eq!(neg_inf, p.bounds().min().x());
    assert_eq!(neg_inf, p.bounds().min().z());
    assert_eq!(pos_inf, p.bounds().max().x());
    assert_eq!(pos_inf, p.bounds().max().z());
    assert_eq!(0.0, p.bounds().min().y());
    assert_eq!(0.0, p.bounds().max().y());
}

#[allow(non_snake_case)]
#[test]
fn group_bounds_is_origin_if_no_members() {
    let g = group(vec![]);
    let neg_inf = std::f64::NEG_INFINITY;
    let pos_inf = std::f64::INFINITY;

    assert_bounds(
        g.clone(),
        (pos_inf, pos_inf, pos_inf),
        (neg_inf, neg_inf, neg_inf),
    );
}

#[allow(non_snake_case)]
#[test]
fn group_minima_and_maxima_are_dictated_by_its_members_bounds() {
    assert_bounds(
        group(vec![unit_sphere(), cylinder(CylKind::Open, -5.0, 2.0)]),
        (-1.0, -5.0, -1.0),
        (1.0, 2.0, 1.0),
    );
    assert_bounds(
        group(vec![unit_sphere(), cube()]),
        (-1.0, -1.0, -1.0),
        (1.0, 1.0, 1.0),
    );
}

fn assert_bounds(obj: Object, min: (f64, f64, f64), max: (f64, f64, f64)) {
    assert_eq!(min.0, obj.bounds().min().x());
    assert_eq!(min.1, obj.bounds().min().y());
    assert_eq!(min.2, obj.bounds().min().z());

    assert_eq!(max.0, obj.bounds().max().x());
    assert_eq!(max.1, obj.bounds().max().y());
    assert_eq!(max.2, obj.bounds().max().z());
}

fn two_groups_and_intersect_them() {
    let c1 = cube().set_object_to_world_spc(translation(4.0, 0.0, 0.0) * rotation_z(std::f64::consts::FRAC_PI_4)).clone();
    let c2 = cube().set_object_to_world_spc(translation(0.0, 4.0, 0.0) * rotation_z(std::f64::consts::FRAC_PI_4)).clone();

    let g1 = group(vec![c1.clone()]).set_object_to_world_spc(translation(0.0, -3.0, 0.0)).clone();
    let g2 = group(vec![c2.clone()]).set_object_to_world_spc(translation(-3.0, 0.0, 0.0)).clone();;

    let w = World::with(vec![], vec![g1, g2]);
    let ints = w.intersect(&ray(point(0.0, -3.0, 0.0), vector(1.0, 0.0, 0.0)));

    assert_eq!(ints.len(), 2);
    assert_eq!(ints[0].t_value(), 4.0 - std::f64::consts::SQRT_2);
    assert_eq!(ints[0].intersected(), c1.clone());

    assert_eq!(ints[1].t_value(), 4.0 + std::f64::consts::SQRT_2);
    assert_eq!(ints[1].intersected(), c1.clone());
}
