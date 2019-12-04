use bucktracer::*;
use bucktracer::math::*;
use bucktracer::wavefront;


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
