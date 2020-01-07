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

#[allow(non_snake_case)]
#[test]
fn use_intersection_uv_to_interpolate_the_normal() {
    let tri = smooth_triangle(
        point( 0.0, 1.0, 0.0),
        point(-1.0, 0.0, 0.0),
        point( 1.0, 0.0, 0.0),
        vector( 0.0, 1.0, 0.0),
        vector(-1.0, 0.0, 0.0),
        vector( 1.0, 0.0, 0.0),
    );
    let i = intersection_with_uv(1.0, &tri, 0.45, 0.25);
    let n = i.normal_at(point(0.0, 0.0, 0.0));
    assert_eq!(n, vector(-0.5547, 0.83205, 0.0));
}

#[allow(non_snake_case)]
#[test]
fn bounds_of_smooth_triangle() {
    let n1 = vector( 0.0, 1.0, 0.0);
    let n2 = vector(-1.0, 0.0, 0.0);
    let n3 = vector( 1.0, 0.0, 0.0);
    assert_eq!(smooth_triangle(point(0.0, 0.0, 0.0), point(0.0, 0.0, 0.0), point(0.0, 0.0, 0.0), n1, n2, n3).bounds(),
               Bounds::new(point(0.0, 0.0, 0.0), point(0.0, 0.0, 0.0)));
    assert_eq!(smooth_triangle(point(0.0, 1.0, 0.0), point(-1.0, 0.0, 0.0), point(1.0, 0.0, 0.0), n1, n2, n3).bounds(),
               Bounds::new(point(-1.0, 0.0, 0.0), point(1.0, 1.0, 0.0)));
    assert_eq!(smooth_triangle(point(-12.0, -3.0, 8.0), point(-1.0, -9.0, 13.0), point(1.0, 6.0, 7.0), n1, n2, n3).bounds(),
               Bounds::new(point(-12.0, -9.0, 7.0), point(1.0, 6.0, 13.0)));

}
