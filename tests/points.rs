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
    assert_eq!(point(1.0,  1.0, 6.0), 
               point(3.0, -2.0, 5.0) + vector(-2.0, 3.0, 1.0));
}

#[test]
fn test_subtraction() {
    assert_eq!(vector(-2.0, -4.0, -6.0),
               point(3.0, 2.0, 1.0) - point(5.0, 6.0, 7.0));
    assert_eq!(point(-2.0, -4.0, -6.0),
               point(3.0, 2.0, 1.0) - vector(5.0, 6.0, 7.0));
}

#[test]
fn negation() {
    assert_eq!( - vector( 1.0, -2.0,  3.0),
                  vector(-1.0,  2.0, -3.0));
    assert_eq!( vector(-1.0, 2.0, -3.0),
                vector( 0.0, 0.0,  0.0) - vector(1.0, -2.0, 3.0));
}

#[test]
fn scaling() {
    assert_eq!(vector(3.5, -7.0, 10.5), vector(1.0, -2.0, 3.0).scale(3.5));
    assert_eq!(vector(0.5, -1.0, 1.5), vector(1.0, -2.0, 3.0).scale(0.5));
}

#[test]
fn magnitude() {
    assert_eq!(0.0, vector(0.0, 0.0, 0.0).magnitude());
    assert_eq!(1.0, vector(1.0, 0.0, 0.0).magnitude());
    assert_eq!(1.0, vector(0.0, 1.0, 0.0).magnitude());
    assert_eq!(1.0, vector(0.0, 0.0, 1.0).magnitude());

    assert_eq!(f64::sqrt(14.0), vector(1.0, 2.0, 3.0).magnitude());
    assert_eq!(f64::sqrt(14.0), vector(-1.0, -2.0, -3.0).magnitude());
}

#[test]
fn normalize() {
    assert_eq!(vector(1.0, 0.0, 0.0), vector(4.0, 0.0, 0.0).normalize());
    assert_eq!(vector(0.0, 1.0, 0.0), vector(0.0, 2.0, 0.0).normalize());
    assert_eq!(vector(0.0, 0.0, 1.0), vector(0.0, 0.0, 5.0).normalize());

    assert_eq!(vector(-1.0, 0.0, 0.0), vector(-4.0, 0.0, 0.0).normalize());
    assert_eq!(vector(0.0, -1.0, 0.0), vector(0.0, -2.0, 0.0).normalize());
    assert_eq!(vector(0.0, 0.0, -1.0), vector(0.0, 0.0, -5.0).normalize());

    assert_eq!(1.0, vector(4.0, 0.0, 0.0).normalize().magnitude());
    assert_eq!(1.0, vector(0.0, 2.0, 0.0).normalize().magnitude());
    assert_eq!(1.0, vector(0.0, 0.0, 5.0).normalize().magnitude());

    assert_eq!(1.0, vector(-4.0, 0.0, 0.0).normalize().magnitude());
    assert_eq!(1.0, vector(0.0, -2.0, 0.0).normalize().magnitude());
    assert_eq!(1.0, vector(0.0, 0.0, -5.0).normalize().magnitude());
}

#[test]
fn dot_product() {
    let a = vector(1.0, 2.0, 3.0);
    let b = vector(2.0, 3.0, 4.0);
    assert_eq!(20.0, a.dot(b));
}

#[test]
fn cross_product() {
    let a = vector(1.0, 2.0, 3.0);
    let b = vector(2.0, 3.0, 4.0);
    assert_eq!(vector(-1.0, 2.0, -1.0), a.cross(b));
    assert_eq!(vector(1.0, -2.0,  1.0), b.cross(a));
}

#[test]
fn create_a_colour() {
    let c = colour(-0.5, 0.4, 1.7);
    assert_eq!(-0.5, red(c));
    assert_eq!( 0.4, green(c));
    assert_eq!( 1.7, blue(c));
}
