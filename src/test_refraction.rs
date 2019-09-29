use crate::*;
use std::f64::consts::SQRT_2;

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

#[test]
fn refracted_colour_of_opaque_surface_is_black() {
    let mut w = World::default();
    let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
    let shape = w.objects[0];
    let xs = vec![intersection(4.0, &shape), intersection(6.0, &shape)];
    let comps = hit_data(&r, 0, &xs);

    assert_eq!(w.refracted_colour(&comps, 5), RGB::black());
}

#[test]
fn refracted_colour_at_max_recursive_depth_is_black() {
    let mut w = World::default();
    let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
    let shape: &mut Object = &mut w.objects[0];
    shape.mut_material().set_transparency(1.0).set_refractive_index(1.5);

    let xs = vec![intersection(4.0, &shape), intersection(6.0, &shape)];
    let comps = hit_data(&r, 0, &xs);
    assert_eq!(w.refracted_colour(&comps, 0), RGB::black());
}

const ROOT2_BY_2: f64 = SQRT_2 / 2.0;

#[test]
fn refracted_colour_not_at_max_recursive_depth_is_not_black() {
    let mut w = World::default();
    let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
    let shape: &mut Object = &mut w.objects[0];
    shape.mut_material().set_transparency(1.0).set_refractive_index(1.5);

    let xs = vec![intersection(4.0, &shape), intersection(6.0, &shape)];
    let comps = hit_data(&r, 0, &xs);
    assert_ne!(w.refracted_colour(&comps, 5), RGB::black());
}

#[test]
fn refracted_colour_under_total_internal_reflection_is_black() {
    let mut w = World::default();
    let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
    let shape: &mut Object = &mut w.objects[0];
    shape.mut_material().set_transparency(1.0).set_refractive_index(1.5);

    let xs = vec![intersection(4.0, &shape), intersection(6.0, &shape)];
    let comps = hit_data(&r, 0, &xs);
    assert_ne!(w.refracted_colour(&comps, 5), RGB::black());

}

#[test]
fn refracted_colour_is_due_to_colour_of_refracted_ray() {
    let mut w = World::default();
    w.objects[0].mut_material()
        .set_ambient(1.0).set_pattern(Pattern::tester());
    w.objects[1].mut_material()
        .set_transparency(1.0).set_refractive_index(1.5);

    let r = ray(point(0.0, 0.0, 0.1), vector(0.0, 1.0, 0.0));
    let xs = vec![
        intersection(-0.9899, &w.objects[0]),
        intersection(-0.4899, &w.objects[1]),
        intersection( 0.4899, &w.objects[1]),
        intersection( 0.9899, &w.objects[0]),
    ];

    let comps = hit_data(&r, 2, &xs);
    assert_eq!(w.refracted_colour(&comps, 5), colour(0.0, 0.99888, 0.04722));
}
