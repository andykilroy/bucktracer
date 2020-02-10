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
    let group = binary_partition(1, vec![]);
    assert_eq!(group.children().len(), 0);
}

#[allow(non_snake_case)]
#[test]
fn one_shape___generates_one_node___because_it_defines_outermost_bounding_box() {
    let tri = triangle(point(1.0, 0.0, 0.0), point(0.0, 1.0, 0.0), point(0.0, 0.0, 1.0));
    let groups = binary_partition(1, vec![tri.clone()]);

    assert_eq!(obj_at(&groups, &[0].clone()).unwrap().children().len(), 1);
    assert_eq!(obj_at(&groups, &[0, 0].clone()), Some(tri));
}

#[allow(non_snake_case)]
#[test]
fn a_shape_placed_in_each_eighth() {
    let root = unit_sphere();
    let s0 = unit_sphere().set_object_to_world_spc( translation(-0.25, -0.25, -0.25) * scaling(0.25, 0.25, 0.25)).clone();
    let s1 = unit_sphere().set_object_to_world_spc( translation(-0.25, -0.25,  0.25) * scaling(0.25, 0.25, 0.25)).clone();
    let s2 = unit_sphere().set_object_to_world_spc( translation(-0.25,  0.25, -0.25) * scaling(0.25, 0.25, 0.25)).clone();
    let s3 = unit_sphere().set_object_to_world_spc( translation(-0.25,  0.25,  0.25) * scaling(0.25, 0.25, 0.25)).clone();
    let s4 = unit_sphere().set_object_to_world_spc( translation( 0.25, -0.25, -0.25) * scaling(0.25, 0.25, 0.25)).clone();
    let s5 = unit_sphere().set_object_to_world_spc( translation( 0.25, -0.25,  0.25) * scaling(0.25, 0.25, 0.25)).clone();
    let s6 = unit_sphere().set_object_to_world_spc( translation( 0.25,  0.25, -0.25) * scaling(0.25, 0.25, 0.25)).clone();
    let s7 = unit_sphere().set_object_to_world_spc( translation( 0.25,  0.25,  0.25) * scaling(0.25, 0.25, 0.25)).clone();

    let groups = binary_partition(1, vec![
        s0.clone(), s1.clone(), s2.clone(), s3.clone(), s4.clone(),
        s5.clone(), s6.clone(), s7.clone(), root.clone()
    ]);

    assert_eq!(obj_at(&groups, &[0, 0].clone()), Some(root));
    assert_eq!(obj_at(&groups, &[1, 0, 0].clone()), Some(s0));
    assert_eq!(obj_at(&groups, &[2, 0, 0].clone()), Some(s1));
    assert_eq!(obj_at(&groups, &[3, 0, 0].clone()), Some(s2));
    assert_eq!(obj_at(&groups, &[4, 0, 0].clone()), Some(s3));
    assert_eq!(obj_at(&groups, &[5, 0, 0].clone()), Some(s4));
    assert_eq!(obj_at(&groups, &[6, 0, 0].clone()), Some(s5));
    assert_eq!(obj_at(&groups, &[7, 0, 0].clone()), Some(s6));
    assert_eq!(obj_at(&groups, &[8, 0, 0].clone()), Some(s7));
}

#[allow(non_snake_case)]
#[test]
fn do_not_add_groups_which_are_empty() {
    let root = unit_sphere();
    let s0 = unit_sphere().set_object_to_world_spc( translation(-0.25, -0.25, -0.25) * scaling(0.25, 0.25, 0.25)).clone();
    let s1 = unit_sphere().set_object_to_world_spc( translation(-0.25, -0.25,  0.25) * scaling(0.25, 0.25, 0.25)).clone();
    let s2 = unit_sphere().set_object_to_world_spc( translation(-0.25,  0.25, -0.25) * scaling(0.25, 0.25, 0.25)).clone();
    let s3 = unit_sphere().set_object_to_world_spc( translation(-0.25,  0.25,  0.25) * scaling(0.25, 0.25, 0.25)).clone();
    let s4 = unit_sphere().set_object_to_world_spc( translation( 0.25, -0.25, -0.25) * scaling(0.25, 0.25, 0.25)).clone();
    let s5 = unit_sphere().set_object_to_world_spc( translation( 0.25, -0.25,  0.25) * scaling(0.25, 0.25, 0.25)).clone();
    let s6 = unit_sphere().set_object_to_world_spc( translation( 0.25,  0.25, -0.25) * scaling(0.25, 0.25, 0.25)).clone();
    let s7 = unit_sphere().set_object_to_world_spc( translation( 0.25,  0.25,  0.25) * scaling(0.25, 0.25, 0.25)).clone();

    let groups = binary_partition(1, vec![
        s0.clone(), s1.clone(), s2.clone(), s3.clone(),
        // s4.clone(),  omit s4
        s5.clone(), s6.clone(), s7.clone(), root.clone()
    ]);

    assert_eq!(obj_at(&groups, &[0, 0].clone()), Some(root));
    assert_eq!(obj_at(&groups, &[1, 0, 0].clone()), Some(s0));
    assert_eq!(obj_at(&groups, &[2, 0, 0].clone()), Some(s1));
    assert_eq!(obj_at(&groups, &[3, 0, 0].clone()), Some(s2));
    assert_eq!(obj_at(&groups, &[4, 0, 0].clone()), Some(s3));
    assert_eq!(obj_at(&groups, &[5, 0, 0].clone()), Some(s5));
    assert_eq!(obj_at(&groups, &[6, 0, 0].clone()), Some(s6));
    assert_eq!(obj_at(&groups, &[7, 0, 0].clone()), Some(s7));
    assert_eq!(obj_at(&groups, &[8, 0, 0].clone()), None);
}

#[allow(non_snake_case)]
#[test]
fn flatten___empty_list___produces_empty_list() {
    let input = vec![];
    let v = flatten(&input);
    assert_eq!(v.len(), 0);
}

#[allow(non_snake_case)]
#[test]
fn flatten___flattened_list_produces_same_result() {
    let input = vec![unit_sphere(), cube(), glass_sphere()];
    let v = flatten(&input);
    assert_eq!(v, input);
}

#[allow(non_snake_case)]
#[test]
fn flatten___children_of_group_are_promoted_to_top_level() {
    let p1 = point(0.0, 0.0, 0.0);
    let p2 = point(0.0, 0.0, 1.0);
    let p3 = point(1.0, 0.0, 0.0);
    let input = vec![
        unit_sphere(),
        cube(),
        group(vec![triangle(p1, p2, p3)]),
        glass_sphere()];

    let v = flatten(&input);
    assert_eq!(v, vec![unit_sphere(), cube(), triangle(p1, p2, p3), glass_sphere()]);
}


// TODO test when the shapes are in one plane (one of the axes has exactly one value)