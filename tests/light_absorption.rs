use bucktracer::*;
use std::f64::consts::FRAC_PI_2;


fn run_scenario(objs: Vec<Object>, expected_factor: f64) {
    let light = point_light(point(0.0, 5.0, 0.0), RGB::white());
    let w = World::with(vec![light], objs);
    let p = point(0.0, -5.0, 0.0);

    assert_eq!(true, almost_eq(w.light_factor(p, &light), expected_factor));
}

#[allow(non_snake_case)]
#[test]
fn when_object_between_light_and_point_is_opaque_object_has_ambient_colour() {
    let mut opaque = plane();
    run_scenario(vec![opaque], 0.0);
}


#[allow(non_snake_case)]
#[test]
fn when_object_between_light_and_point_is_transparent_object_has_some_colour() {
    let mut glass = plane();
    glass.set_material(*Material::default().set_transparency(0.8));
    run_scenario(vec![glass], 0.8);
    run_scenario(vec![glass, glass.clone()], 0.64);
}

fn almost_eq(x1: f64, x2: f64) -> bool {
    f64::abs(x1 - x2) < 0.000001
}
