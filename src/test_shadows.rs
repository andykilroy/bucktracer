use crate::*;

#[test]
fn the_hit_should_bump_the_point_slightly_in_the_direction_of_normalv() {
    let shape = unit_sphere().set_object_to_world_spc(translation(0.0, 0.0, 1.0)).clone();

    let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
    let i = intersection(5.0, &shape);
    let precomputed = hit_data(&r, 0, &[i.clone()]);

    assert_eq!(true, precomputed.point.z() > precomputed.over_point.z())
}
