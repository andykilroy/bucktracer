use bucktracer::*;
use bucktracer::math::*;
use std::io::Read;

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
fn objects_placed_in_one_cell___depth2() {
    let root = unit_sphere();
    let s0 = unit_sphere().set_object_to_world_spc( translation(-0.125, -0.125, -0.125) * scaling(0.20, 0.20, 0.20)).clone();
    let s1 = unit_sphere().set_object_to_world_spc( translation(-0.125, -0.125, -0.125) * scaling(0.20, 0.20, 0.20)).clone();
    let s2 = unit_sphere().set_object_to_world_spc( translation(-0.125, -0.125, -0.125) * scaling(0.20, 0.20, 0.20)).clone();
    let s3 = unit_sphere().set_object_to_world_spc( translation(-0.125, -0.125, -0.125) * scaling(0.20, 0.20, 0.20)).clone();
    let s4 = unit_sphere().set_object_to_world_spc( translation(-0.125, -0.125, -0.125) * scaling(0.20, 0.20, 0.20)).clone();

    let groups = binary_partition(2, vec![
        root.clone(), s0.clone(), s1.clone(), s2.clone(), s3.clone(), s4.clone()
    ]);

    let expected = group(vec![ // 0th level
        group(vec![root]),      // member of 0th level
        group(vec![            // 1st level
            group(vec![        // 2nd level
                group(vec![    // 0th cell
                    s0, s1, s2, s3, s4
                ])
            ])
        ])
    ]);
}

#[allow(non_snake_case)]
#[test]
fn objects_at_multiple_levels() {
    let root = unit_sphere();
    let s10 = unit_sphere().set_object_to_world_spc( translation(-0.25, -0.25, -0.25) * scaling(0.25, 0.25, 0.25)).clone();
    let s11 = unit_sphere().set_object_to_world_spc( translation(-0.25, -0.25,  0.25) * scaling(0.25, 0.25, 0.25)).clone();
    let s12 = unit_sphere().set_object_to_world_spc( translation(-0.25,  0.25, -0.25) * scaling(0.25, 0.25, 0.25)).clone();
    let s13 = unit_sphere().set_object_to_world_spc( translation(-0.25,  0.25,  0.25) * scaling(0.25, 0.25, 0.25)).clone();
    let s14 = unit_sphere().set_object_to_world_spc( translation( 0.25, -0.25, -0.25) * scaling(0.25, 0.25, 0.25)).clone();
    let s15 = unit_sphere().set_object_to_world_spc( translation( 0.25, -0.25,  0.25) * scaling(0.25, 0.25, 0.25)).clone();
    let s16 = unit_sphere().set_object_to_world_spc( translation( 0.25,  0.25, -0.25) * scaling(0.25, 0.25, 0.25)).clone();
    let s17 = unit_sphere().set_object_to_world_spc( translation( 0.25,  0.25,  0.25) * scaling(0.25, 0.25, 0.25)).clone();

    let s20 = unit_sphere().set_object_to_world_spc( translation(-0.125, -0.125, -0.125) * scaling(0.20, 0.20, 0.20)).clone();
    let s21 = unit_sphere().set_object_to_world_spc( translation(-0.125, -0.125, -0.125) * scaling(0.20, 0.20, 0.20)).clone();
    let s22 = unit_sphere().set_object_to_world_spc( translation(-0.125, -0.125, -0.125) * scaling(0.20, 0.20, 0.20)).clone();
    let s23 = unit_sphere().set_object_to_world_spc( translation(-0.125, -0.125, -0.125) * scaling(0.20, 0.20, 0.20)).clone();
    let s24 = unit_sphere().set_object_to_world_spc( translation(-0.125, -0.125, -0.125) * scaling(0.20, 0.20, 0.20)).clone();

    let groups = binary_partition(2, vec![
        s10.clone(), s11.clone(), s12.clone(), s13.clone(), s14.clone(),
        s20.clone(), s21.clone(), s22.clone(), s23.clone(), s24.clone(),
        s15.clone(), s16.clone(), s17.clone(), root.clone()
    ]);

    let expected = group(vec![ // 0th level
        group(vec![root]),      // member of 0th level
        group(vec![            // 0th cell, 1st level
            group(vec![s10]),  // member of 0th cell on 1st level
            group(vec![        // 0th cell, 2nd level
                group(vec![    // members of 0th cell, 2nd level
                    s20, s21, s22, s23, s24
                ])
            ])
        ]),
        group(vec![            // 1st cell, 1st level
            group(vec![s11]),  // member of 1st cell on 1st level
        ]),
        group(vec![            // 1st cell, 1st level
            group(vec![s12]),  // member of 1st cell on 1st level
        ]),
        group(vec![            // 1st cell, 1st level
            group(vec![s13]),  // member of 1st cell on 1st level
        ]),
        group(vec![            // 1st cell, 1st level
            group(vec![s14]),  // member of 1st cell on 1st level
        ]),
        group(vec![            // 1st cell, 1st level
            group(vec![s15]),  // member of 1st cell on 1st level
        ]),
        group(vec![            // 1st cell, 1st level
            group(vec![s16]),  // member of 1st cell on 1st level
        ]),
        group(vec![            // 1st cell, 1st level
            group(vec![s17]),  // member of 1st cell on 1st level
        ]),
    ]);

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

#[allow(non_snake_case)]
#[test]
fn flatten___produces_flat_vector() {
    let mut bytes = r#"v 0 0 0
v 0 0 1
v 0 1 0
v 0 1 1
v 1 0 0
v 1 0 1
v 1 1 0
v 1 1 1

g HalfOne
f 1 2 4 3
f 5 6 8 7
f 1 2 6 5

g HalfTwo
f 3 4 8 7
f 1 3 7 5
f 2 4 8 6
"#.as_bytes();
    let input = wavefront::read_object_vec(&mut bytes).unwrap();

    let v = flatten(&input);
    assert_eq!(v, vec![
        triangle(point(0.0, 0.0, 0.0), point(0.0, 0.0, 1.0), point(0.0, 1.0, 1.0)),
        triangle(point(0.0, 0.0, 0.0), point(0.0, 1.0, 1.0), point(0.0, 1.0, 0.0)),
        triangle(point(1.0, 0.0, 0.0), point(1.0, 0.0, 1.0), point(1.0, 1.0, 1.0)),
        triangle(point(1.0, 0.0, 0.0), point(1.0, 1.0, 1.0), point(1.0, 1.0, 0.0)),
        triangle(point(0.0, 0.0, 0.0), point(0.0, 0.0, 1.0), point(1.0, 0.0, 1.0)),
        triangle(point(0.0, 0.0, 0.0), point(1.0, 0.0, 1.0), point(1.0, 0.0, 0.0)),
        triangle(point(0.0, 1.0, 0.0), point(0.0, 1.0, 1.0), point(1.0, 1.0, 1.0)),
        triangle(point(0.0, 1.0, 0.0), point(1.0, 1.0, 1.0), point(1.0, 1.0, 0.0)),
        triangle(point(0.0, 0.0, 0.0), point(0.0, 1.0, 0.0), point(1.0, 1.0, 0.0)),
        triangle(point(0.0, 0.0, 0.0), point(1.0, 1.0, 0.0), point(1.0, 0.0, 0.0)),
        triangle(point(0.0, 0.0, 1.0), point(0.0, 1.0, 1.0), point(1.0, 1.0, 1.0)),
        triangle(point(0.0, 0.0, 1.0), point(1.0, 1.0, 1.0), point(1.0, 0.0, 1.0)),
    ]);
}

// TODO test when the shapes are in one plane (one of the axes has exactly one value)
