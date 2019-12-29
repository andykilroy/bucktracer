use bucktracer::*;
use bucktracer::math::*;
use crate::almost_eq;

#[allow(non_snake_case)]
#[test]
fn intersection_with_smooth_triangle() {
    let tri = smooth_triangle(
        point( 0.0, 1.0, 0.0),
        point(-1.0, 0.0, 0.0),
        point( 1.0, 0.0, 0.0),
        vector( 0.0, 1.0, 0.0),
        vector(-1.0, 0.0, 0.0),
        vector( 1.0, 0.0, 0.0),
    );

    let i = intersection_with_uv(3.5, &tri, 0.2, 0.4);
    assert_eq!(i.u(), Some(0.2));
    assert_eq!(i.v(), Some(0.4));
    assert_eq!(i.t_value(), 3.5);
    assert_eq!(i.intersected(), tri);
}

#[allow(non_snake_case)]
#[test]
fn an_intersection_with_smooth_triangle_stores_u_v() {
    let tri = smooth_triangle(
        point( 0.0, 1.0, 0.0),
        point(-1.0, 0.0, 0.0),
        point( 1.0, 0.0, 0.0),
        vector( 0.0, 1.0, 0.0),
        vector(-1.0, 0.0, 0.0),
        vector( 1.0, 0.0, 0.0),
    );
    let r = ray(point(-0.2, 0.3, -2.0), vector(0.0, 0.0, 1.0));
    let mut vec = vec![];
    append_intersects(&r, &tri, &mut vec);
    almost_eq(vec[0].u().unwrap(), 0.45);
    almost_eq(vec[0].v().unwrap(), 0.25);
}
