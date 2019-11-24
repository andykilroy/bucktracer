//use bucktracer::*;
use bucktracer::math::*;

const P_INF: f64 = std::f64::INFINITY;
const N_INF: f64 = std::f64::NEG_INFINITY;

#[test]
fn simple_point_creation() {
    assert_eq!(true, is_point(point(4.3, 4.2, 3.1)));
    assert_eq!(false, is_point(vector(4.3, 4.2, 3.1)));

    assert_eq!(false, is_vector(point(4.3, 4.2, 3.1)));
    assert_eq!(true, is_vector(vector(4.3, 4.2, 3.1)));
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
    assert_eq!(
        point(1.0, 1.0, 6.0),
        point(3.0, -2.0, 5.0) + vector(-2.0, 3.0, 1.0)
    );
}

#[test]
fn test_subtraction() {
    assert_eq!(
        vector(-2.0, -4.0, -6.0),
        point(3.0, 2.0, 1.0) - point(5.0, 6.0, 7.0)
    );
    assert_eq!(
        point(-2.0, -4.0, -6.0),
        point(3.0, 2.0, 1.0) - vector(5.0, 6.0, 7.0)
    );
}

#[test]
fn negation() {
    assert_eq!(-vector(1.0, -2.0, 3.0), vector(-1.0, 2.0, -3.0));
    assert_eq!(
        vector(-1.0, 2.0, -3.0),
        vector(0.0, 0.0, 0.0) - vector(1.0, -2.0, 3.0)
    );
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
    assert_eq!(vector(1.0, -2.0, 1.0), b.cross(a));
}

#[allow(non_snake_case)]
#[test]
fn minimum_zeroes_less_than_infinity() {
    is_commutative(Tuple4::min, tuple(P_INF, P_INF, P_INF, P_INF), tuple(0.0, 0.0, 0.0, 0.0), tuple(0.0, 0.0, 0.0, 0.0));
}
#[allow(non_snake_case)]
#[test]
fn minimum_neg_infinity_less_than_zeroes() {
    is_commutative(Tuple4::min, tuple(0.0, 0.0, 0.0, 0.0), tuple(N_INF, N_INF, N_INF, N_INF), tuple(N_INF, N_INF, N_INF, N_INF));
}

#[allow(non_snake_case)]
#[test]
fn minimum_mixed_values() {
    let a = tuple(8.0, 7.0, 6.0, 5.0);
    is_commutative(Tuple4::min, a, tuple(9.0, 9.0, 9.0, 9.0), tuple(8.0, 7.0, 6.0, 5.0));
    is_commutative(Tuple4::min, a, tuple(7.5, 7.5, 7.5, 7.5), tuple(7.5, 7.0, 6.0, 5.0));
    is_commutative(Tuple4::min, a, tuple(6.5, 6.5, 6.5, 6.5), tuple(6.5, 6.5, 6.0, 5.0));
    is_commutative(Tuple4::min, a, tuple(5.5, 5.5, 5.5, 5.5), tuple(5.5, 5.5, 5.5, 5.0));
    is_commutative(Tuple4::min, a, tuple(4.5, 4.5, 4.5, 4.5), tuple(4.5, 4.5, 4.5, 4.5));

    let b = tuple(1.0, 2.0, 3.0, 4.0);
    is_commutative(Tuple4::min, b, tuple(5.0, 5.0, 5.0, 5.0), tuple(1.0, 2.0, 3.0, 4.0));
    is_commutative(Tuple4::min, b, tuple(3.5, 3.5, 3.5, 3.5), tuple(1.0, 2.0, 3.0, 3.5));
    is_commutative(Tuple4::min, b, tuple(2.5, 2.5, 2.5, 2.5), tuple(1.0, 2.0, 2.5, 2.5));
    is_commutative(Tuple4::min, b, tuple(1.5, 1.5, 1.5, 1.5), tuple(1.0, 1.5, 1.5, 1.5));
    is_commutative(Tuple4::min, b, tuple(0.5, 0.5, 0.5, 0.5), tuple(0.5, 0.5, 0.5, 0.5));
}


#[allow(non_snake_case)]
#[test]
fn maximum_zeroes_more_than_neg_infinity() {
    is_commutative(Tuple4::max, tuple(N_INF, N_INF, N_INF, N_INF), tuple(0.0, 0.0, 0.0, 0.0), tuple(0.0, 0.0, 0.0, 0.0));
}
#[allow(non_snake_case)]
#[test]
fn maximum_pos_infinity_more_than_zeroes() {
    is_commutative(Tuple4::max, tuple(0.0, 0.0, 0.0, 0.0), tuple(P_INF, P_INF, P_INF, P_INF), tuple(P_INF, P_INF, P_INF, P_INF));
}

#[allow(non_snake_case)]
#[test]
fn maximum_mixed_values() {
    let a = tuple(8.0, 7.0, 6.0, 5.0);
    is_commutative(Tuple4::max, a, tuple(9.0, 9.0, 9.0, 9.0), tuple(9.0, 9.0, 9.0, 9.0));
    is_commutative(Tuple4::max, a, tuple(7.5, 7.5, 7.5, 7.5), tuple(8.0, 7.5, 7.5, 7.5));
    is_commutative(Tuple4::max, a, tuple(6.5, 6.5, 6.5, 6.5), tuple(8.0, 7.0, 6.5, 6.5));
    is_commutative(Tuple4::max, a, tuple(5.5, 5.5, 5.5, 5.5), tuple(8.0, 7.0, 6.0, 5.5));
    is_commutative(Tuple4::max, a, tuple(4.5, 4.5, 4.5, 4.5), tuple(8.0, 7.0, 6.0, 5.0));

    let b = tuple(1.0, 2.0, 3.0, 4.0);
    is_commutative(Tuple4::max, b, tuple(5.0, 5.0, 5.0, 5.0), tuple(5.0, 5.0, 5.0, 5.0));
    is_commutative(Tuple4::max, b, tuple(3.5, 3.5, 3.5, 3.5), tuple(3.5, 3.5, 3.5, 4.0));
    is_commutative(Tuple4::max, b, tuple(2.5, 2.5, 2.5, 2.5), tuple(2.5, 2.5, 3.0, 4.0));
    is_commutative(Tuple4::max, b, tuple(1.5, 1.5, 1.5, 1.5), tuple(1.5, 2.0, 3.0, 4.0));
    is_commutative(Tuple4::max, b, tuple(0.5, 0.5, 0.5, 0.5), tuple(1.0, 2.0, 3.0, 4.0));
}

fn is_commutative(f: fn(Tuple4, Tuple4) -> Tuple4, a: Tuple4, b: Tuple4, expected: Tuple4) {
    let first = f(a, b);
    let second = f(b, a);
    exact_eq(first, second);
    exact_eq(expected, first);
    exact_eq(expected, second);
}

fn exact_eq(l: Tuple4, r: Tuple4) {
    assert_eq!(l.x(), r.x());
    assert_eq!(l.y(), r.y());
    assert_eq!(l.z(), r.z());
    assert_eq!(l.w(), r.w());
}
