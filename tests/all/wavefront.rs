use bucktracer::*;
use bucktracer::math::*;
use bucktracer::wavefront;
use bucktracer::wavefront::ParseError;


#[allow(non_snake_case)]
#[test]
fn read_empty_file___produces_no_objects() {
    let mut input = "".as_bytes();
    let out: Vec<Object> = wavefront::parse(&mut input).unwrap();
    assert_eq!(out.len(), 0);
}

#[allow(non_snake_case)]
#[test]
fn unknown_instructions___produces_no_objects() {
    let mut input = "blah 1.230, 243".as_bytes();
    let out: Vec<Object> = wavefront::parse(&mut input).unwrap();
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
    let out: Vec<Object> = wavefront::parse(&mut input).unwrap();
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
    assert_eq!(wavefront::parse(&mut input1), Err(ParseError::BadInstruction));
    assert_eq!(wavefront::parse(&mut input2), Err(ParseError::BadInstruction));
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
    assert_eq!(wavefront::parse(&mut input), Err(ParseError::BadInstruction));
}
