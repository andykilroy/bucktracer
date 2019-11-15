use bucktracer::*;

#[allow(non_snake_case)]
#[test]
fn sphere_bounds() {
    let s = unit_sphere();
    assert_eq!(point(-1.0, -1.0, -1.0), s.bounds().min());
    assert_eq!(point( 1.0,  1.0,  1.0), s.bounds().max());
}
