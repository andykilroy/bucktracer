use bucktracer::*;
use bucktracer::math::*;

#[test]
fn create_a_world() {
    let world = World::empty();
    assert_eq!(0, world.objects().len());
    assert_eq!(0, world.light_sources().len());
}

#[test]
fn properties_of_default_world() {
    let world = World::default();
    let light = world.light_sources()[0];
    assert_eq!(light.position(), point(-10.0, 10.0, -10.0));
    assert_eq!(light.intensity(), colour(1.0, 1.0, 1.0));

    let s1 = &world.objects()[0];
    let s2 = &world.objects()[1];
    let p = Pattern::solid(colour(0.8, 1.0, 0.6));
    assert_eq!(s1.material().pattern(), p);
    assert_eq!(s1.material().diffuse(), 0.7);
    assert_eq!(s1.material().specular(), 0.2);

    assert_eq!(s2.object_to_world_spc(), scaling(0.5, 0.5, 0.5));
}

#[allow(non_snake_case)]
#[test]
fn colour_when_a_ray_misses___is_black() {
    let w = World::default();
    let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 1.0, 0.0));
    let c = w.colour_at_intersect(&r, 5);
    assert_eq!(c, colour(0.0, 0.0, 0.0));
}

#[allow(non_snake_case)]
#[test]
fn colour_when_a_ray_hits___is_consequence_of_material_and_light_colour() {
    let w = World::default();
    let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
    let c = w.colour_at_intersect(&r, 5);
    assert_eq!(c, colour(0.38066, 0.47583, 0.2855));
}

#[test]
fn colour_with_an_intersection_behind_the_ray() {
    // TODO Interesting test with mutable references; use of the references can't be reordered while they both exist.
    let mut objects = World::default().objects().to_vec();

    let outer: &mut Object = objects.get_mut(0).unwrap();
    outer.mut_material().set_ambient(1.0);
    let inner: &mut Object = objects.get_mut(1).unwrap();
    inner.mut_material().set_ambient(1.0);

    let w = World::with(World::default().light_sources().to_vec(), objects);

    let r = ray(point(0.0, 0.0, 0.75), vector(0.0, 0.0, -1.0));
    let c = w.colour_at_intersect(&r, 5);
    assert_eq!(c, RGB::white());
}

#[test]
fn point_not_in_shadow_when_nothing_colinear_with_point_and_light() {
    let w = World::default();
    let p = point(0.0, 10.0, 0.0);
    let l = w.light_sources()[0];
    assert_eq!(w.light_factor(p, &l), 1.0);
}

#[test]
fn in_shadow_when_object_between_light_and_point() {
    let w = World::default();
    let p = point(10.0, -10.0, 10.0);
    let l = w.light_sources()[0];
    assert_eq!(w.light_factor(p, &l), 0.0);
}

#[test]
fn an_intersection_in_shadow_returns_ambient_colour() {
    let l = point_light(point(0.0, 0.0, -10.0), RGB::white());
    let s1 = unit_sphere();
    let s2 = unit_sphere().set_object_to_world_spc(translation(0.0, 0.0, 10.0)).clone();

    let objects = vec![s1.clone(), s2.clone()];
    let w = World::with(vec![l], objects);
    let r = ray(point(0.0, 0.0, 5.0), vector(0.0, 0.0, 1.0));
    let c = w.colour_at_intersect(&r, 5);
    assert_eq!(c, colour(0.1, 0.1, 0.1));
}


#[test]
fn point_not_in_shadow_when_light_source_between_point_and_object() {
    let w = World::default();
    let p = point(-20.0, 20.0, -20.0);
    let l = w.light_sources()[0];
    assert_eq!(w.light_factor(p, &l), 1.0);
}

#[test]
fn point_not_in_shadow_when_point_between_light_source_and_object() {
    let w = World::default();
    let p = point(-2.0, 2.0, -2.0);
    let l = w.light_sources()[0];
    assert_eq!(w.light_factor(p, &l), 1.0);
}
