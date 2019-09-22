use crate::*;

#[test]
fn an_intersection_in_shadow_returns_ambient_colour() {
    let l = point_light(point(0.0, 0.0, -10.0), RGB::white());
    let s1 = unit_sphere();
    let s2 = *(unit_sphere().set_object_to_world_spc(translation(0.0, 0.0, 10.0)));

    let objects = vec![s1, s2];
    let w = World::with(vec![l], objects);
    let r = ray(point(0.0, 0.0, 5.0), vector(0.0, 0.0, 1.0));
    let i = intersection(4.0, &s2);
    let c = shade_hit(&w, &precompute(&i, &r), RECURSION_LIMIT);
    assert_eq!(c, colour(0.1, 0.1, 0.1));
}

#[test]
fn the_hit_should_bump_the_point_slightly_in_the_direction_of_normalv() {
    let shape = *(unit_sphere().set_object_to_world_spc(translation(0.0, 0.0, 1.0)));

    let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
    let i = intersection(5.0, &shape);
    let precomputed = precompute(&i, &r);
    assert_eq!(true, precomputed.point.z() > precomputed.over_point.z())
}
