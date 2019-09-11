use bucktracer::*;

fn white() -> RGB {
    RGB::white()
}
fn black() -> RGB {
    RGB::black()
}
fn stripe_pattern(c1: RGB, c2: RGB) -> Pattern {
    Pattern::stripes(c1, c2)
}
fn stripe_at(ptrn: Pattern, pos: Tuple4) -> RGB {
    ptrn.colour_at(pos)
}

#[test]
fn a_solid_pattern_is_the_same_everywhere() {
    let c = colour(0.8, 0.5, 0.9);
    let p = Pattern::solid(c);
    assert_eq!(p.colour_at(point(0.0,-2.0, 0.0)), c);
    assert_eq!(p.colour_at(point(0.0,-1.0, 0.0)), c);
    assert_eq!(p.colour_at(point(0.0, 0.0, 0.0)), c);
    assert_eq!(p.colour_at(point(0.0, 1.0, 0.0)), c);
    assert_eq!(p.colour_at(point(0.0, 2.0, 0.0)), c);

    assert_eq!(p.colour_at(point(0.0, 0.0,-2.0)), c);
    assert_eq!(p.colour_at(point(0.0, 0.0,-1.0)), c);
    assert_eq!(p.colour_at(point(0.0, 0.0, 0.0)), c);
    assert_eq!(p.colour_at(point(0.0, 0.0, 1.0)), c);
    assert_eq!(p.colour_at(point(0.0, 0.0, 2.0)), c);

    assert_eq!(p.colour_at(point(-2.0, 0.0, 0.0)), c);
    assert_eq!(p.colour_at(point(-1.0, 0.0, 0.0)), c);
    assert_eq!(p.colour_at(point( 0.0, 0.0, 0.0)), c);
    assert_eq!(p.colour_at(point( 1.0, 0.0, 0.0)), c);
    assert_eq!(p.colour_at(point( 2.0, 0.0, 0.0)), c);
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

#[test]
fn lighting_with_a_pattern_applied() {
    let mut m = Material::default();
    m.set_pattern(stripe_pattern(RGB::white(), RGB::black()));
    m.set_ambient(1.0);
    m.set_diffuse(0.0);
    m.set_specular(0.0);
    let eyev = vector(0.0, 0.0, -1.0);
    let normalv = vector(0.0, 0.0, -1.0);
    let light = point_light(point(0.0, 0.0, -10.0), RGB::white());

    let c1 = lighting(&light, point(0.9, 0.0, 0.0), normalv, &m, eyev, false);
    let c2 = lighting(&light, point(1.1, 0.0, 0.0), normalv, &m, eyev, false);

    assert_eq!(c1, RGB::white());
    assert_eq!(c2, RGB::black());
}
