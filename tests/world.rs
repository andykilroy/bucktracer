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
fn intersect_a_world_with_a_ray() {
    let world = World::default();
    let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
    let intersects = world.intersect(r);
    assert_eq!(intersects.len(), 4);

    assert_eq!(intersects[0].t_value, 4.0);
    assert_eq!(intersects[1].t_value, 4.5);
    assert_eq!(intersects[2].t_value, 5.5);
    assert_eq!(intersects[3].t_value, 6.0);
}

#[test]
fn precompute_state_of_intersection() {
    let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
    let shape = unit_sphere();
    let i = intersection(4.0, &shape);
    let comps = precompute(&i, &r);
    assert_eq!(comps.t_value, 4.0);
    assert_eq!(comps.object, shape);
    assert_eq!(comps.point, point(0.0, 0.0, -1.0));
    assert_eq!(comps.eyev, vector(0.0, 0.0, -1.0));
    assert_eq!(comps.normalv, vector(0.0, 0.0, -1.0));
}

#[test]
fn the_hit_when_intersection_occurs_on_outside() {
    let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
    let shape = unit_sphere();
    let i = intersection(4.0, &shape);
    let comps = precompute(&i, &r);
    assert_eq!(comps.inside, false);
}

#[test]
fn the_hit_when_an_intersection_occurs_on_inside() {
    let r = ray(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
    let shape = unit_sphere();
    let i = intersection(1.0, &shape);
    let comps = precompute(&i, &r);
    assert_eq!(comps.t_value, 1.0);
    assert_eq!(comps.object, shape);
    assert_eq!(comps.point, point(0.0, 0.0, 1.0));
    assert_eq!(comps.eyev, vector(0.0, 0.0, -1.0));
    assert_eq!(comps.normalv, vector(0.0, 0.0, -1.0));
    assert_eq!(comps.inside, true);
}

#[test]
fn shade_an_intersection_point() {
    let w = World::default();
    let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
    let shape = w.objects()[0];
    let i = intersection(4.0, &shape);
    let comps = precompute(&i, &r);
    let c = shade_hit(&w, &comps);
    assert_eq!(c, colour(0.38066, 0.47583, 0.2855));
}

#[test]
fn shade_an_intersection_point_from_inside() {
    let light = point_light(point(0.0, 0.25, 0.0), white());
    let w = World::with(World::default().objects(), vec![light]);
    let r = ray(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
    let shape = w.objects()[1];
    let i = intersection(0.5, &shape);

    let comps = precompute(&i, &r);
    let c = shade_hit(&w, &comps);
    assert_eq!(c, colour(0.90498, 0.90498, 0.90498));
}
