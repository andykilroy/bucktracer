use bucktracer::*;

fn white() -> RGB {
    RGB::white()
}
fn black() -> RGB {
    RGB::black()
}

#[test]
fn a_stripe_pattern_is_constant_in_y() {
    let p = stripe_pattern(white(), black());
    assert_eq!(stripe_at(p, point(0.0, 0.0, 0.0)), white());
    assert_eq!(stripe_at(p, point(0.0, 1.0, 0.0)), white());
    assert_eq!(stripe_at(p, point(0.0, 2.0, 0.0)), white());
}

#[test]
fn a_stripe_pattern_is_constant_in_z() {
    let p = stripe_pattern(white(), black());
    assert_eq!(stripe_at(p, point(0.0, 0.0, 0.0)), white());
    assert_eq!(stripe_at(p, point(0.0, 0.0, 1.0)), white());
    assert_eq!(stripe_at(p, point(0.0, 0.0, 2.0)), white());
}

#[test]
fn a_stripe_pattern_alternates_in_x() {
    let p = stripe_pattern(white(), black());

    assert_eq!(stripe_at(p, point(-2.1, 0.0, 0.0)), black());
    assert_eq!(stripe_at(p, point(-2.0, 0.0, 0.0)), white());

    assert_eq!(stripe_at(p, point(-1.1, 0.0, 0.0)), white());
    assert_eq!(stripe_at(p, point(-1.0, 0.0, 0.0)), black());

    assert_eq!(stripe_at(p, point(-0.1, 0.0, 0.0)), black());
    assert_eq!(stripe_at(p, point( 0.0, 0.0, 0.0)), white());

    assert_eq!(stripe_at(p, point( 0.9, 0.0, 0.0)), white());
    assert_eq!(stripe_at(p, point( 1.0, 0.0, 0.0)), black());

    assert_eq!(stripe_at(p, point( 1.9, 0.0, 0.0)), black());
    assert_eq!(stripe_at(p, point( 2.0, 0.0, 0.0)), white());
}
