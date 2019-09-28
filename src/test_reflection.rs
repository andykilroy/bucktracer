use crate::*;
use std::f64::consts::*;

const ROOT2_BY_2: f64 = SQRT_2 / 2.0;

#[test]
fn compute_reflective_vector() {
    let p = plane();
    let r = ray(point(0.0, 1.0, -1.0), vector(0.0, -ROOT2_BY_2, ROOT2_BY_2));
    let inter = intersection(SQRT_2, &p);
    let comps = singleton_hit_data(&r, &inter);

    assert_eq!(comps.reflectv, vector(0.0, ROOT2_BY_2, ROOT2_BY_2));
}

#[test]
fn reflected_colour_for_non_reflective_material_is_black() {
    let mut w = World::default();
    w.objects[1].material.set_reflective(0.0);
    w.objects[1].material.set_ambient(1.0);
    let r = ray(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
    let obj: Object = w.objects[1];
    let i = intersection(1.0, &obj);

    assert_eq!(
        w.reflected_colour(&singleton_hit_data(&r, &i), RECURSION_LIMIT),
        RGB::black()
    );
}

#[test]
fn reflected_colour_for_reflective_material() {
    let mut w = World::default();
    let mut p = plane();
    p.material.reflective = 0.5;
    p.set_object_to_world_spc(translation(0.0, -1.0, 0.0));
    w.objects.push(p);

    let r = ray(point(0.0, 0.0, -3.0), vector(0.0, -ROOT2_BY_2, ROOT2_BY_2));
    let i = intersection(SQRT_2, &p);
    let rgb = w.reflected_colour(&singleton_hit_data(&r, &i), RECURSION_LIMIT);

    assert_eq!(rgb, colour(0.19033, 0.23791, 0.14274));
}

#[test]
fn shade_hit_for_reflective_material() {
    let mut w = World::default();
    let mut p = plane();
    p.material.reflective = 0.5;
    p.set_object_to_world_spc(translation(0.0, -1.0, 0.0));
    w.objects.push(p);

    let r = ray(point(0.0, 0.0, -3.0), vector(0.0, -ROOT2_BY_2, ROOT2_BY_2));
    let i = intersection(SQRT_2, &p);
    let rgb = shade_hit(&w, &singleton_hit_data(&r, &i), RECURSION_LIMIT);

    assert_eq!(rgb, colour(0.87675, 0.92434, 0.82918));
}

#[test]
fn colour_at_max_recursion_depth() {
    let mut w = World::default();
    let mut p = plane();
    p.material.reflective = 0.5;
    p.set_object_to_world_spc(translation(0.0, -1.0, 0.0));
    w.objects.push(p);

    let r = ray(point(0.0, 0.0, -3.0), vector(0.0, -ROOT2_BY_2, ROOT2_BY_2));
    let i = intersection(SQRT_2, &p);
    let rgb = w.reflected_colour(&singleton_hit_data(&r, &i), 0);

    assert_eq!(rgb, RGB::black());
}
