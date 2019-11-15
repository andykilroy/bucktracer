use bucktracer::*;

#[allow(non_snake_case)]
#[test]
fn sphere_bounds() {
    let s = unit_sphere();
    assert_eq!(point(-1.0, -1.0, -1.0), s.bounds().min());
    assert_eq!(point( 1.0,  1.0,  1.0), s.bounds().max());
}

#[allow(non_snake_case)]
#[test]
fn cube_bounds() {
    let c = cube();
    assert_eq!(point(-1.0, -1.0, -1.0), c.bounds().min());
    assert_eq!(point( 1.0,  1.0,  1.0), c.bounds().max());
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
    let neg_inf = std::f64::NEG_INFINITY;
    let pos_inf = std::f64::INFINITY;
    scenario_inf_cylinder(CylKind::Closed, neg_inf, 1.0);
    scenario_inf_cylinder(CylKind::Closed,     2.0, pos_inf);
    scenario_inf_cylinder(CylKind::Open,   neg_inf, 1.0);
    scenario_inf_cylinder(CylKind::Open,       2.0, pos_inf);
}

fn scenario_inf_cylinder(kind: CylKind, lbound: f64, ubound: f64) {
    let c = cylinder(kind, lbound, ubound);
    assert_eq!(-1.0, c.bounds().min().x());
    assert_eq!(-1.0, c.bounds().min().z());
    assert_eq!( 1.0, c.bounds().max().x());
    assert_eq!( 1.0, c.bounds().max().z());
    assert_eq!(lbound, c.bounds().min().y());
    assert_eq!(ubound, c.bounds().max().y());
}
