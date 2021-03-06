use bucktracer::*;
use bucktracer::math::*;
use bucktracer::wavefront;
use bucktracer::wavefront::ParseError;


#[allow(non_snake_case)]
#[test]
fn read_empty_file___produces_no_objects() {
    let mut input = "".as_bytes();
    let out: Vec<Object> = wavefront::read_object_vec(&mut input).unwrap();
    assert_eq!(out.len(), 0);
}

#[allow(non_snake_case)]
#[test]
fn unknown_instructions___produces_no_objects() {
    let mut input = "blah 1.230, 243".as_bytes();
    let out: Vec<Object> = wavefront::read_object_vec(&mut input).unwrap();
    assert_eq!(out.len(), 0);
}

#[allow(non_snake_case)]
#[test]
fn read_in_triangles() {
    let mut input = r##"v -1 1 0
v -1 0 0
v 1 0 0
v 1 1 0
f 1 2 3
f 1 3 4
"##.as_bytes();
    let out: Vec<Object> = wavefront::read_object_vec(&mut input).unwrap();
    assert_eq!(out, vec![
        triangle(point(-1.0, 1.0, 0.0), point(-1.0, 0.0, 0.0), point(1.0, 0.0, 0.0)),
        triangle(point(-1.0, 1.0, 0.0), point(1.0, 0.0, 0.0), point(1.0, 1.0, 0.0)),
    ]);
}

#[allow(non_snake_case)]
#[test]
fn bad_instruction_if_vertex_is_not_triple() {
    let mut input1 = "v -1".as_bytes();
    let mut input2 = "v -1 1".as_bytes();
    assert_eq!(wavefront::read_object_vec(&mut input1), Err(ParseError::BadInstruction));
    assert_eq!(wavefront::read_object_vec(&mut input2), Err(ParseError::BadInstruction));
}

#[allow(non_snake_case)]
#[test]
fn zero_index_for_face_is_illegal() {
    let mut input = r##"v -1 1 0
v -1 0 0
v 1 0 0
v 1 1 0
f 0 2 3
"##.as_bytes();
    assert_eq!(wavefront::read_object_vec(&mut input), Err(ParseError::BadInstruction));
}

#[allow(non_snake_case)]
#[test]
fn face_must_have_three_vertices() {
    let mut input1 = r##"v -1 1 0
v -1 0 0
v 1 0 0
v 1 1 0
f 1 2
"##.as_bytes();
    let mut input2 = r##"v -1 1 0
v -1 0 0
v 1 0 0
v 1 1 0
f 1
"##.as_bytes();
    assert_eq!(wavefront::read_object_vec(&mut input1), Err(ParseError::BadInstruction));
    assert_eq!(wavefront::read_object_vec(&mut input2), Err(ParseError::BadInstruction));
}

#[allow(non_snake_case)]
#[test]
fn fan_triangulation() {
    let mut input = r##"v -1 1 0
v -1 0 0
v 1 0 0
v 1 1 0
v 0 2 0
f 1 2 3 4 5
"##.as_bytes();

    let out: Vec<Object> = wavefront::read_object_vec(&mut input).unwrap();
    assert_eq!(out, vec![
        triangle(point(-1.0, 1.0, 0.0), point(-1.0, 0.0, 0.0), point(1.0, 0.0, 0.0)),
        triangle(point(-1.0, 1.0, 0.0), point(1.0, 0.0, 0.0), point(1.0, 1.0, 0.0)),
        triangle(point(-1.0, 1.0, 0.0), point(1.0, 1.0, 0.0), point(0.0, 2.0, 0.0)),
    ]);
}

#[allow(non_snake_case)]
#[test]
fn extract_groups() {
    let mut input = r##"v -1 1 0
v -1 0 0
v 1 0 0
v 1 1 0

g FirstGroup
f 1 2 3

g SecondGroup
f 1 3 4
"##.as_bytes();

    let out: Vec<Object> = wavefront::read_object_vec(&mut input).unwrap();

    assert_eq!(out, vec![
        group(vec![
            triangle(point(-1.0, 1.0, 0.0), point(-1.0, 0.0, 0.0), point(1.0, 0.0, 0.0)),
        ]),
        group(vec![
            triangle(point(-1.0, 1.0, 0.0), point(1.0, 0.0, 0.0), point(1.0, 1.0, 0.0)),
        ])
    ]);
}

#[allow(non_snake_case)]
#[test]
fn vertex_normals___translate_to_smooth_triangles() {
    let mut input = r##"v 0 1 0
v -1 0 0
v 1 0 0

vn -1 0 0
vn 1 0 0
vn 0 1 0

f 1//3 2//1 3//2
f 1/0/3 2/102/1 3/14/2
"##.as_bytes();
    let out: Vec<Object> = wavefront::read_object_vec(&mut input).unwrap();

    assert_eq!(out, vec![
        smooth_triangle(point(0.0, 1.0, 0.0),
                        point(-1.0, 0.0, 0.0),
                        point(1.0, 0.0, 0.0),
                        vector(0.0, 1.0, 0.0),
                        vector(-1.0, 0.0, 0.0),
                        vector(1.0, 0.0, 0.0)),
        smooth_triangle(point(0.0, 1.0, 0.0),
                        point(-1.0, 0.0, 0.0),
                        point(1.0, 0.0, 0.0),
                        vector(0.0, 1.0, 0.0),
                        vector(-1.0, 0.0, 0.0),
                        vector(1.0, 0.0, 0.0))
    ]);

}


#[allow(non_snake_case)]
#[test]
fn when_faces_have_two_slashes___error_if_normal_index_not_given() {
    let mut input = r##"v 0 1 0
v -1 0 0
v 1 0 0

vn -1 0 0
vn 1 0 0
vn 0 1 0

f 1// 2//1 3//2
"##.as_bytes();
    assert_eq!(wavefront::read_object_vec(&mut input).is_err(), true);
}


#[allow(non_snake_case)]
#[test]
fn when_faces_only_has_one_slash___error() {
    let mut input = r##"v 0 1 0
v -1 0 0
v 1 0 0

vn -1 0 0
vn 1 0 0
vn 0 1 0

f 1/ 2//1 3//2
"##.as_bytes();
    assert_eq!(wavefront::read_object_vec(&mut input).is_err(), true);
}

#[allow(non_snake_case)]
#[test]
fn when_face_specifies_one_vertex_normal___all_must_specify_normals() {
    let mut input = r##"v 0 1 0
v -1 0 0
v 1 0 0

vn -1 0 0
vn 1 0 0
vn 0 1 0

f 1//3 2 3
"##.as_bytes();
    assert_eq!(wavefront::read_object_vec(&mut input).is_err(), true);
}
