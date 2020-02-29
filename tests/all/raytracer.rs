use bucktracer::*;
use bucktracer::math::*;

#[allow(non_snake_case)]
#[test]
fn colour_when_a_ray_misses___is_black() {
    let mut tracer = RayTracer::new();
    let w = World::default();
    let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 1.0, 0.0));
    let c = tracer.colour_at_intersect(&r, 5, &w);
    assert_eq!(c, colour(0.0, 0.0, 0.0));
}

#[allow(non_snake_case)]
#[test]
fn colour_when_a_ray_hits___the_colour_is_consequence_of_material_and_light_colour() {
    let mut tracer = RayTracer::new();
    let w = World::default();
    let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
    let c = tracer.colour_at_intersect(&r, 5, &w);
    assert_eq!(c, colour(0.38066, 0.47583, 0.2855));
}

#[allow(non_snake_case)]
#[test]
fn when_determining_colour___determine_colour_of_first_hit_along_the_ray() {
    // TODO Interesting test with mutable references; use of the references can't be reordered while they both exist.
    let mut objects = World::default().objects().to_vec();
    let outers_colour = colour(1.0, 0.0, 0.0);
    let inners_colour = colour(0.0, 1.0, 0.0);
    let outer: &mut Object = objects.get_mut(0).unwrap();
    outer.mut_material().set_ambient(1.0);
    outer.mut_material().set_pattern(Pattern::Solid(outers_colour));
    let inner: &mut Object = objects.get_mut(1).unwrap();
    inner.mut_material().set_ambient(1.0);
    inner.mut_material().set_pattern(Pattern::Solid(inners_colour));

    let w = World::with(World::default().light_sources().to_vec(), objects);
    let mut tracer = RayTracer::new();

    // position the origin of the ray between inner and outer concentric spheres, directed at the inner.
    let r = ray(point(0.0, 0.0, 0.75), vector(0.0, 0.0, -1.0));
    let c = tracer.colour_at_intersect(&r, 5, &w);
    assert_eq!(c, inners_colour);
}

#[test]
fn point_not_in_shadow_when_nothing_colinear_with_point_and_light() {
    let mut tracer = RayTracer::new();
    let w = World::default();
    let p = point(0.0, 10.0, 0.0);
    let l = w.light_sources()[0];

    assert_eq!(tracer.light_factor(p, &l, &w), 1.0);
}

#[test]
fn in_shadow_when_object_between_light_and_point() {
    let mut tracer = RayTracer::new();
    let w = World::default();
    let p = point(10.0, -10.0, 10.0);
    let l = w.light_sources()[0];
    assert_eq!(tracer.light_factor(p, &l, &w), 0.0);
}

#[test]
fn an_intersection_in_shadow_returns_ambient_colour() {
    let mut tracer = RayTracer::new();
    let l = point_light(point(0.0, 0.0, -10.0), RGB::white());
    let s1 = unit_sphere();
    let s2 = unit_sphere().set_object_to_world_spc(translation(0.0, 0.0, 10.0)).clone();

    let objects = vec![s1.clone(), s2.clone()];
    let w = World::with(vec![l], objects);
    let r = ray(point(0.0, 0.0, 5.0), vector(0.0, 0.0, 1.0));
    let c = tracer.colour_at_intersect(&r, 5, &w);
    assert_eq!(c, colour(0.1, 0.1, 0.1));
}

#[test]
fn point_not_in_shadow_when_light_source_between_point_and_object() {
    let mut tracer = RayTracer::new();
    let w = World::default();
    let p = point(-20.0, 20.0, -20.0);
    let l = w.light_sources()[0];
    assert_eq!(tracer.light_factor(p, &l, &w), 1.0);
}

#[test]
fn point_not_in_shadow_when_point_between_light_source_and_object() {
    let mut tracer = RayTracer::new();
    let w = World::default();
    let p = point(-2.0, 2.0, -2.0);
    let l = w.light_sources()[0];
    assert_eq!(tracer.light_factor(p, &l, &w), 1.0);
}