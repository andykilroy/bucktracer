use bucktracer::*;
use std::f64::consts::FRAC_PI_2;

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

    let s1 = world.objects()[0];
    let s2 = world.objects()[1];
    let p = Pattern::solid(colour(0.8, 1.0, 0.6));
    assert_eq!(s1.material().pattern(), p);
    assert_eq!(s1.material().diffuse(), 0.7);
    assert_eq!(s1.material().specular(), 0.2);

    assert_eq!(s2.transform_to_world(), scaling(0.5, 0.5, 0.5));

}

#[test]
fn colour_when_a_ray_misses() {
    let w = World::default();
    let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 1.0, 0.0));
    let c = w.colour_at_intersect(&r);
    assert_eq!(c, colour(0.0, 0.0, 0.0));
}

#[test]
fn colour_when_a_ray_hits() {
    let w = World::default();
    let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
    let c = w.colour_at_intersect(&r);
    assert_eq!(c, colour(0.38066, 0.47583, 0.2855));
}

#[test]
fn colour_with_an_intersection_behind_the_ray() {
    let mut w = World::default();
    let mut outer = w.objects()[0];
    let mut inner = w.objects()[1];

    let mut outer_mat = outer.material();
    outer.set_material(outer_mat.set_ambient(1.0).clone());
    let mut inner_mat = inner.material();
    inner.set_material(inner_mat.set_ambient(1.0).clone());

    w.set_objects(vec![outer, inner]);

    let r = ray(point(0.0, 0.0, 0.75), vector(0.0, 0.0, -1.0));
    let c = w.colour_at_intersect(&r);
    assert_eq!(c, RGB::white());
}

#[test]
fn point_not_in_shadow_when_nothing_colinear_with_point_and_light() {
    let w = World::default();
    let p = point(0.0, 10.0, 0.0);
    let l = w.light_sources()[0];
    assert_eq!(w.in_shadow(p, &l), false);
}

#[test]
fn in_shadow_when_object_between_light_and_point() {
    let w = World::default();
    let p = point(10.0, -10.0, 10.0);
    let l = w.light_sources()[0];
    assert_eq!(w.in_shadow(p, &l), true);
}

#[test]
fn point_not_in_shadow_when_light_source_between_point_and_object() {
    let w = World::default();
    let p = point(-20.0, 20.0, -20.0);
    let l = w.light_sources()[0];
    assert_eq!(w.in_shadow(p, &l), false);
}

#[test]
fn point_not_in_shadow_when_point_between_light_source_and_object() {
    let w = World::default();
    let p = point(-2.0, 2.0, -2.0);
    let l = w.light_sources()[0];
    assert_eq!(w.in_shadow(p, &l), false);
}

#[test]
fn render_world_with_camera() {
    let w = World::default();
    let mut c = Camera::new(11, 11, FRAC_PI_2);
    let t = view_transform(
        point(0.0, 0.0, -5.0),
        point(0.0, 0.0, 0.0),
        vector(0.0, 1.0, 0.0)
    );
    c.set_transform(t);
    let image = c.render(&w);
    assert_eq!(image.colour_at(5, 5), colour(0.38066, 0.47583, 0.2855))
}
