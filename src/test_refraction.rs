use crate::*;

#[test]
fn refractions1() {
    scenario_refractive_indices(0, 1.0, 1.5);
    scenario_refractive_indices(1, 1.5, 2.0);
    scenario_refractive_indices(2, 2.0, 2.5);
    scenario_refractive_indices(3, 2.5, 2.5);
    scenario_refractive_indices(4, 2.5, 1.5);
    scenario_refractive_indices(5, 1.5, 1.0);
}

fn scenario_refractive_indices(index: usize, n1: f64, n2: f64) {
    let mut a = glass_sphere();
    let mut b = glass_sphere();
    let mut c = glass_sphere();
    a.set_object_to_world_spc(scaling(2.0, 2.0, 2.0));
    a.mut_material().set_refractive_index(1.5);
    b.set_object_to_world_spc(translation(0.0, 0.0, -0.25));
    b.mut_material().set_refractive_index(2.0);
    c.set_object_to_world_spc(translation(0.0, 0.0, 0.25));
    c.mut_material().set_refractive_index(2.5);

    let r = ray(point(0.0, 0.0, -4.0), vector(0.0, 0.0, 1.0));
    let xs = vec![
        intersection(2.00, &a),
        intersection(2.75, &b),
        intersection(3.25, &c),
        intersection(4.75, &b),
        intersection(5.25, &c),
        intersection(6.00, &a),
    ];

    let comps = hit_data(&r, index, &xs);
    assert_eq!(comps.n1, n1);
    assert_eq!(comps.n2, n2);
}

#[test]
fn under_point_is_below_the_surface() {
    let mut shape = glass_sphere();
    shape.set_object_to_world_spc(translation(0.0, 0.0, 1.0));
    let i = intersection(5.0, &shape);
    let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
    let comps = singleton_hit_data(&r, &i);
    assert_eq!(comps.under_point.z() > (EPSILON / 2.0), true);
    assert_eq!(comps.point.z() < comps.under_point.z(), true);
}
