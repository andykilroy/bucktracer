use bucktracer::*;

#[test]
fn simple_point_creation() {
    assert_eq!(true,  is_point(point(4.3, 4.2, 3.1)));
    assert_eq!(false, is_point(vector(4.3, 4.2, 3.1)));

    assert_eq!(false, is_vector(point(4.3, 4.2, 3.1)));
    assert_eq!(true,  is_vector(vector(4.3, 4.2, 3.1)));
}

#[test]
fn almost_equal() {
    assert_eq!(point(4.3, -4.2, 3.1), point(4.3, -4.2, 3.1));
    assert_eq!(point(4.300001, -4.2, 3.1), point(4.3, -4.2, 3.1));
    assert_eq!(point(4.3, -4.200001, 3.1), point(4.3, -4.2, 3.1));
    assert_eq!(point(4.3, -4.2, 3.100001), point(4.3, -4.2, 3.1));

    assert_eq!(vector(4.3, -4.2, 3.1), vector(4.3, -4.2, 3.1));
    assert_eq!(vector(4.300001, -4.2, 3.1), vector(4.3, -4.2, 3.1));
    assert_eq!(vector(4.3, -4.200001, 3.1), vector(4.3, -4.2, 3.1));
    assert_eq!(vector(4.3, -4.2, 3.100001), vector(4.3, -4.2, 3.1));
}

#[test]
fn definitely_not_equal() {
    assert_ne!(point(4.35, -4.2, 3.1), point(4.3, -4.2, 3.1));
    assert_ne!(point(4.3, -4.25, 3.1), point(4.3, -4.2, 3.1));
    assert_ne!(point(4.3, -4.2, 3.15), point(4.3, -4.2, 3.1));

    assert_ne!(vector(4.35, -4.2, 3.1), vector(4.3, -4.2, 3.1));
    assert_ne!(vector(4.3, -4.25, 3.1), vector(4.3, -4.2, 3.1));
    assert_ne!(vector(4.3, -4.2, 3.15), vector(4.3, -4.2, 3.1));

    assert_ne!(point(4.3, -4.2, 3.1), vector(4.3, -4.2, 3.1));
    assert_ne!(vector(4.3, -4.2, 3.1), point(4.3, -4.2, 3.1));
}

#[test]
fn test_addition() {
    
}
