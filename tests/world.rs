use bucktracer::*;

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
    assert_eq!(s1.material().colour(), colour(0.8, 1.0, 0.6));
    assert_eq!(s1.material().diffuse(), 0.7);
    assert_eq!(s1.material().specular(), 0.2);

    assert_eq!(s2.transform(), scaling(0.5, 0.5, 0.5));

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
    assert_eq!(c, inner_mat.colour());

}
