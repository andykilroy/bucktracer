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
fn bounds_of_empty_group___cannot_intersect_any_ray() {
    use std::f64::{MIN, MAX};
    let g = group(vec![]);
    // no good example of 'all' rays...
    let r1 = ray(point(MIN, 0.0, 0.0), vector(1.0, 0.0, 0.0));
    let r2 = ray(point(MAX, 0.0, 0.0), vector(-1.0, 0.0, 0.0));

    let mut v = vec![];
    append_intersects(&r1, &g, &mut v);
    append_intersects(&r2, &g, &mut v);
    assert_eq!(v.len(), 0);
}

#[allow(non_snake_case)]
#[test]
fn bounds_of_empty_group___cannot_contain_any_bound() {
    use std::f64::{MIN, MAX};
    use std::f64::{NEG_INFINITY, INFINITY};
    let largest_finite = Bounds::new(tuple(MIN, MIN, MIN, MIN), tuple(MAX, MAX, MAX, MAX));
    let largest_infinite = Bounds::new(tuple(NEG_INFINITY, NEG_INFINITY, NEG_INFINITY, NEG_INFINITY), tuple(INFINITY, INFINITY, INFINITY, INFINITY));
    let g = group(vec![]);

    assert_eq!(g.bounds().contains(&largest_finite), false);
    assert_eq!(g.bounds().contains(&largest_infinite), false);
    assert_eq!(largest_finite.contains(&g.bounds()), false);
    assert_eq!(largest_infinite.contains(&g.bounds()), false);
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

#[allow(non_snake_case)]
#[test]
fn enclose_two_shapes___produces_smallest_bounding_box_that_contains_them() {
    let c = cube().set_object_to_world_spc(translation(0.0, 5.0, 2.0)).clone();
    let s = unit_sphere();
    assert_bounds(Bounds::enclose(&[c, s]), (-1.0, -1.0, -1.0), (1.0, 6.0, 3.0));
}

#[allow(non_snake_case)]
#[test]
fn enclose_two_shapes___order_doesnt_matter() {
    let c = cube().set_object_to_world_spc(translation(0.0, 5.0, 2.0)).clone();
    let s = unit_sphere();
    assert_bounds(Bounds::enclose(&[s, c]), (-1.0, -1.0, -1.0), (1.0, 6.0, 3.0));
}

#[allow(non_snake_case)]
#[test]
fn contains___returns_true_if_rhs_within_lhs() {
    let lhs = Bounds::new(point(0.0, 0.0, 0.0), point(5.0, 5.0, 5.0));
    let rhs1 = Bounds::new(point(1.0, 1.0, 1.0), point(4.0, 4.0, 4.0));
    let rhs2 = Bounds::new(point(1.0, 1.0, 1.0), point(5.0, 5.0, 5.0));
    assert_eq!(lhs.contains(&rhs1), true);
    assert_eq!(lhs.contains(&rhs2), true);
}

#[allow(non_snake_case)]
#[test]
fn contains___a_bounds_contains_itself() {
    let lhs = Bounds::new(point(0.0, 0.0, 0.0), point(5.0, 5.0, 5.0));
    assert_eq!(lhs.contains(&lhs), true);
}

#[allow(non_snake_case)]
#[test]
fn contains___when_rhs_includes_space_outside_lhs___lhs_does_not_contain_rhs() {
    let lhs = Bounds::new(point(0.0, 0.0, 0.0), point(5.0, 5.0, 5.0));
    let rhs = Bounds::new(point(1.0, 1.0, 1.0), point(4.0, 4.0, 6.0));
    assert_eq!(lhs.contains(&rhs), false);
}

#[allow(non_snake_case)]
#[test]
fn contains___when_rhs_completely_outside_lhs___lhs_does_not_contain_rhs() {
    let lhs = Bounds::new(point(0.0, 0.0, 0.0), point(5.0, 5.0, 5.0));
    let rhs = Bounds::new(point(7.0, 1.0, 1.0), point(11.0, 4.0, 6.0));
    assert_eq!(lhs.contains(&rhs), false);
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
