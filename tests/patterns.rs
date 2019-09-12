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
fn green() -> RGB {
    colour(0.0, 1.0, 0.0)
}
fn blue() -> RGB {
    colour(0.0, 0.0, 1.0)
}


#[test]
fn patterns_are_equal() {
    let wht = RGB::white();
    let p1 = Pattern::solid(wht);
    let p2 = Pattern::solid(RGB::white());
    let p3 = Pattern::solid(RGB::white());
    let p4 = Pattern::solid(colour(1.0, 0.0, 0.0));
    let p5 = Pattern::stripes(RGB::white(), RGB::black());
    let p6 = Pattern::stripes(RGB::black(), RGB::white());

    // reflexive
    assert_eq!(p1, p1);
    assert_eq!(p2, p2);
    assert_eq!(p3, p3);
    assert_eq!(p4, p4);
    assert_eq!(p5, p5);
    assert_eq!(p6, p6);

    // symmetric
    assert_eq!(p1, p2);
    assert_eq!(p2, p1);
    assert_eq!(p3, p2);
    assert_eq!(p2, p3);

    // transitive
    assert_eq!(p1, p3)
}

#[test]
fn patterns_not_equal() {
    let wht = RGB::white();
    let p1 = Pattern::solid(wht);
    let _p2 = Pattern::solid(RGB::white());
    let _p3 = Pattern::solid(RGB::white());
    let p4 = Pattern::solid(colour(1.0, 0.0, 0.0));
    let p5 = Pattern::stripes(RGB::white(), RGB::black());
    let p6 = Pattern::stripes(RGB::black(), RGB::white());

    assert_ne!(p1, p4);
    assert_ne!(p4, p1);

    assert_ne!(p4, p5);
    assert_ne!(p5, p4);

    assert_ne!(p5, p6);
    assert_ne!(p6, p5);

    assert_ne!(p6, p1);
    assert_ne!(p1, p6);
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
    let mut s = unit_sphere();
    s.set_material(m);

    let eyev = vector(0.0, 0.0, -1.0);
    let normalv = vector(0.0, 0.0, -1.0);
    let light = point_light(point(0.0, 0.0, -10.0), RGB::white());

    let c1 = lighting(&light, point(0.9, 0.0, 0.0), normalv, &s, eyev, false);
    let c2 = lighting(&light, point(1.1, 0.0, 0.0), normalv, &s, eyev, false);

    assert_eq!(c1, RGB::white());
    assert_eq!(c2, RGB::black());
}


fn test_mat(p: Pattern, t: Matrix) -> Material {
    Material::default()
        .set_pattern(p)
        .set_pattern_to_object_spc(t)
        .clone()
}

#[test]
fn stripes_with_an_object_transform() {
    let mut s = unit_sphere();
    s.set_transform_to_world(scaling(2.0, 2.0, 2.0));
    s.set_material(test_mat(
        stripe_pattern(RGB::white(), RGB::black()),
        identity()
    ));

    let c1 = s.material_colour_at(point(1.5, 0.0, 0.0));
    let c2 = s.material_colour_at(point(2.5, 0.0, 0.0));
    assert_eq!(c1, RGB::white());
    assert_eq!(c2, RGB::black());
}

#[test]
fn stripes_with_a_pattern_transform() {
    let mut s = unit_sphere();
    s.set_material(test_mat(
        stripe_pattern(RGB::white(), RGB::black()),
        scaling(2.0, 2.0, 2.0)
    ));

    let c1 = s.material_colour_at(point(1.5, 0.0, 0.0));
    let c2 = s.material_colour_at(point(2.5, 0.0, 0.0));
    assert_eq!(c1, RGB::white());
    assert_eq!(c2, RGB::black());
}

#[test]
fn stripes_with_an_object_and_pattern_transform() {
    let mut s = unit_sphere();
    s.set_transform_to_world(scaling(2.0, 2.0, 2.0));
    s.set_material(test_mat(
        stripe_pattern(RGB::white(), RGB::black()),
        translation(0.5, 0.0, 0.0)
    ));

    let c1 = s.material_colour_at(point(2.5, 0.0, 0.0));
    let c2 = s.material_colour_at(point(3.0, 0.0, 0.0));
    assert_eq!(c1, RGB::white());
    assert_eq!(c2, RGB::black());
}

#[test]
fn gradient_x_within_0_and_1() {
    let mut s = unit_sphere();
    s.set_transform_to_world(identity());
    s.set_material(test_mat(
        Pattern::gradient(green(), blue()),
        identity()
    ));

    assert_eq!(s.material_colour_at(point(0.0, 0.0, 0.0)), colour(0.0, 1.0, 0.0));
    assert_eq!(s.material_colour_at(point(0.5, 0.0, 0.0)), colour(0.0, 0.5, 0.5));
    assert_eq!(s.material_colour_at(point(1.0, 0.0, 0.0)), colour(0.0, 0.0, 1.0));
}

#[test]
fn gradient_x_out_of_bounds() {
    let mut s = unit_sphere();
    s.set_transform_to_world(identity());
    s.set_material(test_mat(
        Pattern::gradient(green(), blue()),
        identity()
    ));

    assert_eq!(s.material_colour_at(point(-1.0, 0.0, 0.0)), colour(0.0, 1.0, 0.0));
    assert_eq!(s.material_colour_at(point(-0.5, 0.0, 0.0)), colour(0.0, 1.0, 0.0));
    assert_eq!(s.material_colour_at(point(1.5, 0.0, 0.0)), colour(0.0, 0.0, 1.0));
    assert_eq!(s.material_colour_at(point(2.0, 0.0, 0.0)), colour(0.0, 0.0, 1.0));
}
