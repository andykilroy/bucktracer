use std::vec::*;
use bucktracer::*;

#[test]
fn simple_point() {
    assert_eq!(true,  is_point((4.3, 4.2, 3.1, 1.0)));
    assert_eq!(false, is_point((4.3, 4.2, 3.1, 0.0)));

    assert_eq!(false, is_vector((4.3, 4.2, 3.1, 1.0)));
    assert_eq!(true,  is_vector((4.3, 4.2, 3.1, 0.0)));
}

#[test]
fn simple_point_creation() {
    assert_eq!(true,  is_point(point(4.3, 4.2, 3.1)));
    assert_eq!(false, is_point(vector(4.3, 4.2, 3.1)));

    assert_eq!(false, is_vector(point(4.3, 4.2, 3.1)));
    assert_eq!(true,  is_vector(vector(4.3, 4.2, 3.1)));
}
