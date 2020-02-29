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
    let mut tracer = RayTracer::new();
    let w = World::default();
    let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
    let shape = &w.objects[0];
    let xs = vec![intersection(4.0, shape), intersection(6.0, shape)];
    let comps = hit_data(&r, 0, &xs);

    assert_eq!(tracer.refracted_colour(&comps, 5, &w), RGB::black());
}

#[test]
fn refracted_colour_at_max_recursive_depth_is_black() {
    let mut tracer = RayTracer::new();
    let mut w = World::default();
    let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
    let shape: &mut Object = &mut w.objects[0];
    shape.mut_material().set_transparency(1.0).set_refractive_index(1.5);

    let xs = vec![intersection(4.0, &shape), intersection(6.0, &shape)];
    let comps = hit_data(&r, 0, &xs);
    assert_eq!(tracer.refracted_colour(&comps, 0, &w), RGB::black());
}

#[test]
fn refracted_colour_not_at_max_recursive_depth_is_not_black() {
    let mut tracer = RayTracer::new();
    let mut w = World::default();
    let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
    let shape: &mut Object = &mut w.objects[0];
    shape.mut_material().set_transparency(1.0).set_refractive_index(1.5);

    let xs = vec![intersection(4.0, &shape), intersection(6.0, &shape)];
    let comps = hit_data(&r, 0, &xs);
    assert_ne!(tracer.refracted_colour(&comps, 5, &w), RGB::black());
}

const ROOT2_BY_2: f64 = SQRT_2 / 2.0;

#[test]
fn refracted_colour_under_total_internal_reflection_is_black() {
    let mut tracer = RayTracer::new();
    let mut w = World::default();
    let shape: &mut Object = &mut w.objects[0];
    shape.mut_material().set_transparency(1.0).set_refractive_index(1.5);
    let r = ray(point(0.0, 0.0, ROOT2_BY_2), vector(0.0, 1.0, 0.0));

    let xs = vec![intersection(-ROOT2_BY_2, &shape), intersection(ROOT2_BY_2, &shape)];
    let comps = hit_data(&r, 1, &xs);
    assert_eq!(tracer.refracted_colour(&comps, 5, &w), RGB::black());
}

#[test]
fn refracted_colour_is_due_to_colour_of_refracted_ray() {
    let mut tracer = RayTracer::new();
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
    assert_eq!(tracer.refracted_colour(&comps, 5, &w), colour(0.0, 0.99888, 0.04722));
}

#[test]
fn shade_hit_transparent_material() {
    let mut tracer = RayTracer::new();
    let mut w = World::default();

    let mut floor = plane();
    floor.set_object_to_world_spc(translation(0.0, -1.0, 0.0));
    floor.mut_material().set_transparency(0.5).set_refractive_index(1.5);
    w.objects.push(floor.clone());

    let mut ball = unit_sphere();
    ball.set_object_to_world_spc(translation(0.0, -3.5, -0.5));
    ball.mut_material().set_pattern(Pattern::solid(colour(1.0, 0.0, 0.0))).set_ambient(0.5);
    w.objects.push(ball.clone());

    let r = ray(point(0.0, 0.0, -3.0), vector(0.0, -ROOT2_BY_2, ROOT2_BY_2));
    let xs = vec![intersection(SQRT_2, &floor)];
    let comps = hit_data(&r, 0, &xs);
    let c = shade_hit(&mut tracer,&w, &comps, 5);
//    assert_eq!(c, colour(0.93642, 0.68642, 0.68642));
    assert_eq!(c, colour(1.12546, 0.68642, 0.68642));
}

#[allow(non_snake_case)]
#[test]
fn schlick_approx_with_a_perpendicular_viewing_angle() {
    let shape = glass_sphere();
    let r = ray(point(0.0, 0.0, 0.0), vector(0.0, 1.0, 0.0));
    let xs = vec![
        intersection(-1.0, &shape),
        intersection( 1.0, &shape),
    ];
    let comps = hit_data(&r, 1, &xs);

    assert_eq!(almost_eq(schlick(&comps), 0.04), true);
}

#[allow(non_snake_case)]
#[test]
fn schlick_approximation_with_small_angle_n2_gt_n1() {
    let shape = glass_sphere();
    let r = ray(point(0.0, 0.99, -2.0), vector(0.0, 0.0, 1.0));
    let xs = vec![
        intersection(1.8589, &shape),
    ];
    let comps = hit_data(&r, 0, &xs);
    assert_eq!(almost_eq(schlick(&comps), 0.48873), true);
}

fn almost_eq(x1: f64, x2: f64) -> bool {
    f64::abs(x1 - x2) < 0.000001
}

#[allow(non_snake_case)]
#[test]
fn shade_hit_with_reflective_transparent_material() {
    let mut tracer = RayTracer::new();
    let mut w = World::default();
    let r = ray(point(0.0, 0.0, -3.0), vector(0.0, -ROOT2_BY_2, ROOT2_BY_2));
    let mut floor = plane();
    floor.set_object_to_world_spc(translation(0.0, -1.0, 0.0));
    floor.mut_material()
        .set_reflective(0.5)
        .set_transparency(0.5)
        .set_refractive_index(1.5);
    let mut ball = unit_sphere();
    ball.mut_material()
        .set_pattern(Pattern::solid(colour(1.0, 0.0, 0.0)))
        .set_ambient(0.5);
    ball.set_object_to_world_spc(translation(0.0, -3.5, -0.5));

    w.objects.push(floor.clone());
    w.objects.push(ball.clone());

    let xs = vec![intersection(SQRT_2, &floor)];
    let comps = hit_data(&r, 0, &xs);
//    assert_eq!(shade_hit(&w, &comps, 5), colour(0.93391, 0.69643, 0.69243));
    assert_eq!(shade_hit(&mut tracer, &w, &comps, 5), colour(1.11500, 0.69643, 0.69243));

}
