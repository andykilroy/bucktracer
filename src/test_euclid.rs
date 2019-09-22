use crate::*;

#[test]
fn positive_numerator_positive_denominator() {
    assert_eq!(rem_euclid(0.0, 2.0), 0.0);
    assert_eq!(rem_euclid(0.5, 2.0), 0.5);
    assert_eq!(rem_euclid(1.0, 2.0), 1.0);
    assert_eq!(rem_euclid(1.5, 2.0), 1.5);
    assert_eq!(rem_euclid(2.0, 2.0), 0.0);
    assert_eq!(rem_euclid(2.5, 2.0), 0.5);
    assert_eq!(rem_euclid(3.0, 2.0), 1.0);
}

#[test]
fn negative_numerator_positive_denominator() {
    assert_eq!(rem_euclid(0.0, 2.0), 0.0);
    assert_eq!(rem_euclid(-0.5, 2.0), 1.5);
    assert_eq!(rem_euclid(-1.0, 2.0), 1.0);
    assert_eq!(rem_euclid(-1.5, 2.0), 0.5);
    assert_eq!(rem_euclid(-2.0, 2.0), 0.0);
    assert_eq!(rem_euclid(-2.5, 2.0), 1.5);
    assert_eq!(rem_euclid(-3.0, 2.0), 1.0);
}
