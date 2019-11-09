use bucktracer::*;

#[test]
fn point_light_has_intensity_and_position() {
    let white = colour(1.0, 1.0, 1.0);
    let p = point(0.0, 0.0, 0.0);
    let l = point_light(p, white);

    assert_eq!(p, l.position());
    assert_eq!(white, l.intensity());
}

#[test]
fn default_material() {
    let m = Material::default();
    let solid_white = Pattern::solid(RGB::white());
    assert_eq!(m.pattern(), solid_white);
    assert_eq!(m.ambient(), 0.1);
    assert_eq!(m.diffuse(), 0.9);
    assert_eq!(m.specular(), 0.9);
    assert_eq!(m.shininess(), 200.0);
}

#[test]
fn materials_not_equal() {
    let df = Material::default();
    let by_pattern =
        *(Material::default().set_pattern(Pattern::stripes(colour(1.0, 0.0, 0.0), RGB::white())));
    let by_ambient = *(Material::default().set_ambient(0.5));
    let by_diffuse = *(Material::default().set_diffuse(0.6));
    let by_specular = *(Material::default().set_specular(0.4));
    let by_shininess = *(Material::default().set_shininess(0.1));

    assert_ne!(df, by_pattern);
    assert_ne!(df, by_ambient);
    assert_ne!(df, by_diffuse);
    assert_ne!(df, by_specular);
    assert_ne!(df, by_shininess);
}

#[test]
fn default_material_on_sphere() {
    let s = unit_sphere();
    assert_eq!(s.material(), Material::default());
}

#[test]
fn assign_sphere_a_material() {
    let mut m = Material::default();
    let mut s = unit_sphere();

    assert_eq!(s.material(), Material::default());

    let p = Pattern::solid(colour(0.4, 0.6, 0.5));
    m.set_pattern(p)
        .set_ambient(1.0)
        .set_diffuse(1.0)
        .set_specular(1.0)
        .set_shininess(100.0);
    s.set_material(m);

    assert_eq!(s.material().pattern(), p);
    assert_eq!(s.material().ambient(), 1.0);
    assert_eq!(s.material().diffuse(), 1.0);
    assert_eq!(s.material().specular(), 1.0);
    assert_eq!(s.material().shininess(), 100.0);

    assert_ne!(s.material(), Material::default());
    assert_eq!(s.material(), m);
}

#[test]
fn lighting_with_eye_between_light_and_the_surface() {
    let s = unit_sphere();
    let pos = point(0.0, 0.0, 0.0);
    let eyev = vector(0.0, 0.0, -1.0);
    let normalv = vector(0.0, 0.0, -1.0);
    let light = point_light(point(0.0, 0.0, -10.0), RGB::white());

    let result = lighting(&light, pos, normalv, &s, eyev, 1.0);
    assert_eq!(result, colour(1.9, 1.9, 1.9));
}

#[test]
fn lighting_with_the_surface_in_shadow() {
    let s = unit_sphere();
    let pos = point(0.0, 0.0, 0.0);
    let eyev = vector(0.0, 0.0, -1.0);
    let normalv = vector(0.0, 0.0, -1.0);
    let light = point_light(point(0.0, 0.0, -10.0), RGB::white());

    let result = lighting(&light, pos, normalv, &s, eyev, 0.0);
    assert_eq!(result, colour(0.1, 0.1, 0.1));
}

#[test]
fn lighting_with_eye_between_light_and_the_surface_at_45_angle() {
    let rt2by2 = f64::sqrt(2.0) / 2.0;
    let s = unit_sphere();
    let pos = point(0.0, 0.0, 0.0);
    let eyev = vector(0.0, rt2by2, -rt2by2);
    let normalv = vector(0.0, 0.0, -1.0);
    let light = point_light(point(0.0, 0.0, -10.0), RGB::white());

    let result = lighting(&light, pos, normalv, &s, eyev, 1.0);
    assert_eq!(result, colour(1.0, 1.0, 1.0));
}

#[test]
fn lighting_with_eye_opposite_surface_light_offset_45_angle() {
    let s = unit_sphere();
    let pos = point(0.0, 0.0, 0.0);
    let eyev = vector(0.0, 0.0, -1.0);
    let normalv = vector(0.0, 0.0, -1.0);
    let light = point_light(point(0.0, 10.0, -10.0), RGB::white());

    let result = lighting(&light, pos, normalv, &s, eyev, 1.0);
    assert_eq!(result, colour(0.7364, 0.7364, 0.7364));
}

#[test]
fn lighting_with_eye_in_path_of_reflection_vector() {
    let rt2by2 = f64::sqrt(2.0) / 2.0;
    let s = unit_sphere();
    let pos = point(0.0, 0.0, 0.0);
    let eyev = vector(0.0, -rt2by2, -rt2by2);
    let normalv = vector(0.0, 0.0, -1.0);
    let light = point_light(point(0.0, 10.0, -10.0), RGB::white());

    let result = lighting(&light, pos, normalv, &s, eyev, 1.0);
    assert_eq!(result, colour(1.6364, 1.6364, 1.6364));
}

#[test]
fn lighting_with_light_behind_the_surface() {
    let s = unit_sphere();
    let pos = point(0.0, 0.0, 0.0);
    let eyev = vector(0.0, 0.0, -1.0);
    let normalv = vector(0.0, 0.0, -1.0);
    let light = point_light(point(0.0, 0.0, 10.0), RGB::white());

    let result = lighting(&light, pos, normalv, &s, eyev, 1.0);
    assert_eq!(result, colour(0.1, 0.1, 0.1));
}

#[test]
fn default_material_is_not_reflective() {
    let m = Material::default();
    assert_eq!(m.reflective(), 0.0);
}


fn run_scenario(objs: Vec<Object>, expected_factor: f64) {
    let light = point_light(point(0.0, 5.0, 0.0), RGB::white());
    let w = World::with(vec![light], objs);
    let p = point(0.0, -5.0, 0.0);

    assert_eq!(true, almost_eq(w.light_factor(p, &light), expected_factor));
}

#[allow(non_snake_case)]
#[test]
fn when_object_between_light_and_point_is_opaque_object_has_ambient_colour() {
    let opaque = plane();
    run_scenario(vec![opaque], 0.0);
}


#[allow(non_snake_case)]
#[test]
fn when_object_between_light_and_point_is_transparent_object_has_some_colour() {
    let mut glass = plane();
    glass.set_material(*Material::default().set_transparency(0.8));
    run_scenario(vec![glass.clone()], 0.8);
    run_scenario(vec![glass.clone(), glass.clone()], 0.64);
}
