use bucktracer::*;
use bucktracer::math::*;
use std::f64::consts::FRAC_PI_2;

#[allow(non_snake_case)]
#[test]
fn can_create_empty_group() {
    let grp = group(vec![]);
    assert_eq!(identity(), grp.object_to_world_spc());
}

#[allow(non_snake_case)]
#[test]
fn can_create_group_with_various_objects_as_children() {
    let shapes = vec![unit_sphere(), inf_cylinder()];
    let grp = group(shapes.clone());
    assert_eq!(shapes, grp.children().to_vec());
}

#[allow(non_snake_case)]
#[test]
fn create_with_nested_groups() {
    let childgroup = group(vec![cube()]);
    let shapes = vec![unit_sphere(), inf_cylinder(), childgroup];
    let grp = group(shapes.clone());
    assert_eq!(shapes, grp.children().to_vec());
}

#[allow(non_snake_case)]
#[test]
fn when_intersecting_with_empty_group___produces_no_intersections() {
    let grp = group(vec![]);
    let r = ray(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
    let mut result: Vec<Intersection> = vec![];
    let expected: Vec<Intersection>  = vec![];

    append_intersects(&r, &grp, &mut result);
    assert_eq!(expected, result);
}

#[allow(non_snake_case)]
#[test]
fn when_intersect_with_non_empty_group___can_produce_no_intersections() {
    let s1 = unit_sphere();
    let mut s2 = unit_sphere();
    let mut s3 = unit_sphere();
    s2.set_object_to_world_spc(translation(0.0, 0.0, -3.0));
    s3.set_object_to_world_spc(translation(5.0, 0.0, 0.0));

    let grp = group(vec![s1.clone(), s2.clone(), s3.clone()]);
    let r = ray(point(1.0, 0.0, 1.5), vector(1.0, 0.0, 0.0));

    let w = World::with(vec![], vec![grp.clone()]);
    let result = w.intersect(&r);

    assert_intersections(result, vec![]);
}

fn assert_intersections(result: Vec<Intersection>, expected: Vec<Object>) {
    let objects: Vec<Object> = result.iter().map(|i| i.intersected()).collect();
    assert_eq!(objects, expected);
}

#[allow(non_snake_case)]
#[test]
fn when_intersect_with_non_empty_group___can_produce_intersections() {
    let s1 = unit_sphere();
    let mut s2 = unit_sphere();
    let mut s3 = unit_sphere();
    s2.set_object_to_world_spc(translation(0.0, 0.0, -3.0));
    s3.set_object_to_world_spc(translation(5.0, 0.0, 0.0));

    let grp = group(vec![s1.clone(), s2.clone(), s3.clone()]);
    let r = ray(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));

    let w = World::with(vec![], vec![grp.clone()]);
    let result = w.intersect(&r);

    assert_intersections(
        result,
        vec![s2.clone(), s2.clone(), s1.clone(), s1.clone()]
    );
}

#[allow(non_snake_case)]
#[test]
fn intersect_group___must_account_for_groups_transformation() {
    let mut s1 = unit_sphere();
    s1.set_object_to_world_spc(translation(5.0, 0.0, 0.0));
    let mut grp = group(vec![s1.clone()]);
    grp.set_object_to_world_spc(scaling(2.0, 2.0, 2.0));

    let r = ray(point(10.0, 0.0, -10.0), vector(0.0, 0.0, 1.0));
    let w = World::with(vec![], vec![grp.clone()]);
    let result = w.intersect(&r);

    assert_eq!(2, result.len());
}

#[allow(non_snake_case)]
#[test]
fn intersect_transforms_world_point_to_object_point() {
    let mut sphere = unit_sphere();
    sphere.set_object_to_world_spc(translation(5.0, 0.0, 0.0));
    let mut g2 = group(vec![sphere.clone()]);
    g2.set_object_to_world_spc(rotation_y(FRAC_PI_2));
    let mut g1 = group(vec![g2]);
    g1.set_object_to_world_spc(rotation_x(FRAC_PI_2));

    let w = World::with(vec![], vec![g1]);
    assert_eq!(0, w.intersect(&ray(point(-5.0, 0.0,  0.0), vector(1.0, 0.0, 0.0))).len());
    assert_eq!(0, w.intersect(&ray(point(-5.0, 0.0, -5.0), vector(1.0, 0.0, 0.0))).len());

    // transform to object space must be S G2 G1 p where S, G2 ,G1 are world to object transforms
    let intersections = w.intersect(&ray(point(-5.0, 5.0,  0.0), vector(1.0, 0.0, 0.0)));
    let p = point(0.0, 6.0, 0.0);
    assert_eq!(4.0, intersections[0].t_value());
    assert_eq!(6.0, intersections[1].t_value());
    assert_eq!(sphere, intersections[0].intersected());

    assert_eq!(vector(0.0, 1.0, 0.0), intersections[0].normal_at(p));
}


