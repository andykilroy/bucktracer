use bucktracer::*;
use bucktracer::math::*;

#[allow(non_snake_case)]
#[test]
fn normal_for_triangle() {
    let t = triangle(point(0.0, 1.0, 0.0), point(-1.0, 0.0, 0.0), point(1.0, 0.0, 0.0));
    let n1 = t.normal_at(point( 0.0, 0.5, 0.0));
    let n2 = t.normal_at(point(-0.5, 0.75, 0.0));
    let n3 = t.normal_at(point( 0.5, 0.25, 0.0));
    assert_eq!(n1, vector(0.0, 0.0, -1.0));
    assert_eq!(n2, vector(0.0, 0.0, -1.0));
    assert_eq!(n3, vector(0.0, 0.0, -1.0));
}
