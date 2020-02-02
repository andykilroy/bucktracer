use bucktracer::*;
use bucktracer::math::*;

#[allow(non_snake_case)]
#[test]
fn leaf_object___but_path_non_empty___return_none() {
    let path = vec![1, 2];
    let c = cube();

    assert_eq!(obj_at(&c, &path), None);
}


#[allow(non_snake_case)]
#[test]
fn non_empty_group___path_indicates_existing_object___return_object() {
    let path = vec![2];
    let target = triangle(point(-1.0, 0.0, -1.0), point(0.0, 1.0, 0.0), point(0.0, 0.0, 1.0));
    let level1 = vec![cube(), cube(), target.clone()];
    let group = group(level1);

    assert_eq!(obj_at(&group, &path), Some(target));
}

#[allow(non_snake_case)]
#[test]
fn path_to_existing_object___multiple_levels___returns_object() {
    let path = vec![0,3,1];
    let path_to_group = vec![0,1];
    let target = triangle(point(-1.0, 0.0, -1.0), point(0.0, 1.0, 0.0), point(0.0, 0.0, 1.0));
    let target_group = group(vec![cube(), cube(), unit_sphere()]);
    let group = group(vec![
        group(vec![
            cube(),
            target_group.clone(),
            glass_sphere(),
            group(vec![
                glass_sphere(),
                target.clone(),
                cube()
            ])
        ]),
        cube(),
        group(vec![inf_cylinder()]),
    ]);

    assert_eq!(obj_at(&group, &path), Some(target));
    assert_eq!(obj_at(&group, &path_to_group), Some(target_group));
}


#[allow(non_snake_case)]
#[test]
fn path_goes_out_of_bounds___returns_none() {
    let non_existent = 5;
    let path = vec![0,non_existent,1];
    let target = triangle(point(-1.0, 0.0, -1.0), point(0.0, 1.0, 0.0), point(0.0, 0.0, 1.0));
    let group = group(vec![
        group(vec![
            cube(),
            glass_sphere(),
            group(vec![
                glass_sphere(),
                target.clone(),
                cube()
            ])
        ]),
        cube(),
        group(vec![inf_cylinder()]),
    ]);

    assert_eq!(obj_at(&group, &path), None);
}

#[allow(non_snake_case)]
#[test]
fn no_objects___generates_empty_group() {
    let group = partition(vec![]);
    let path = [0];

    assert_eq!(obj_at(&group, &path).unwrap().children().len(), 0);
}

#[allow(non_snake_case)]
#[test]
fn one_shape___generates_one_group___because_it_defines_outermost_bounding_box() {
    let tri = triangle(point(1.0, 0.0, 0.0), point(0.0, 1.0, 0.0), point(0.0, 0.0, 1.0));
    let groups = partition(vec![tri.clone()]);

    assert_eq!(obj_at(&groups, &[0].clone()).unwrap().children().len(), 1);
    assert_eq!(obj_at(&groups, &[0, 0].clone()), Some(tri));
}

// TODO test when the shapes are in one plane (one of the axes has exactly one value)
