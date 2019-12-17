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

#[allow(non_snake_case)]
#[test]
fn normal_for_triangle___points_out_from_anticlockwise_face() {
    let t = triangle(point(0.0, 1.0, 0.0), point(1.0, 0.0, 0.0), point(0.0, -1.0, 0.0));
    let n1 = t.normal_at(point( 0.0,  0.5, 0.0));
    let n2 = t.normal_at(point( 0.5,  0.75, 0.0));
    let n3 = t.normal_at(point( 0.5, -0.25, 0.0));
    assert_eq!(n1, vector(0.0, 0.0, 1.0));
    assert_eq!(n2, vector(0.0, 0.0, 1.0));
    assert_eq!(n3, vector(0.0, 0.0, 1.0));
}

#[allow(non_snake_case)]
#[test]
fn parallel_ray___does_not_intersect_triangle() {
    let mut vec = vec![];
    let t = triangle(point(0.0, 1.0, 0.0), point(-1.0, 0.0, 0.0), point(1.0, 0.0, 0.0));
    let r = ray(point(0.0, -1.0, -2.0), vector(0.0, 1.0, 0.0));
    append_intersects(&r, &t, &mut vec);
    assert_eq!(vec.len(), 0);
}

#[allow(non_snake_case)]
#[test]
fn ray_misses_the_p3_p1_edge() {
    let mut vec = vec![];
    let t = triangle(point(0.0, 1.0, 0.0), point(-1.0, 0.0, 0.0), point(1.0, 0.0, 0.0));
    let r = ray(point(1.0, 1.0, -2.0), vector(0.0, 0.0, 1.0));
    append_intersects(&r, &t, &mut vec);
    assert_eq!(vec.len(), 0);
}

#[allow(non_snake_case)]
#[test]
fn ray_misses_the_p2_p1_edge() {
    let mut vec = vec![];
    let t = triangle(point(0.0, 1.0, 0.0), point(-1.0, 0.0, 0.0), point(1.0, 0.0, 0.0));
    let r = ray(point(-1.0, 1.0, -2.0), vector(0.0, 0.0, 1.0));
    append_intersects(&r, &t, &mut vec);
    assert_eq!(vec.len(), 0);
}


#[allow(non_snake_case)]
#[test]
fn ray_misses_the_p2_p3_edge() {
    let mut vec = vec![];
    let t = triangle(point(0.0, 1.0, 0.0), point(-1.0, 0.0, 0.0), point(1.0, 0.0, 0.0));
    let r = ray(point(0.0, -1.0, -2.0), vector(0.0, 0.0, 1.0));
    append_intersects(&r, &t, &mut vec);
    assert_eq!(vec.len(), 0);
}

#[allow(non_snake_case)]
#[test]
fn ray_strikes_the_triangle() {
    let mut vec = vec![];
    let t = triangle(point(0.0, 1.0, 0.0), point(-1.0, 0.0, 0.0), point(1.0, 0.0, 0.0));
    let r = ray(point(0.0, 0.5, -2.0), vector(0.0, 0.0, 1.0));
    append_intersects(&r, &t, &mut vec);
    assert_eq!(vec.len(), 1);
    assert_eq!(vec[0].t_value(), 2.0);
    assert_eq!(vec[0].intersected(), t);
}

