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
    assert_eq!(m.colour(), white());
    assert_eq!(m.ambient(), 0.1);
    assert_eq!(m.diffuse(), 0.9);
    assert_eq!(m.specular(), 0.9);
    assert_eq!(m.shininess(), 200.0);
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

    m.set_colour(colour(0.4, 0.6, 0.5))
        .set_ambient(1.0)
        .set_diffuse(1.0)
        .set_specular(1.0)
        .set_shininess(100.0);
    s.set_material(&m);

    assert_eq!(s.material().colour(), colour(0.4, 0.6, 0.5));
    assert_eq!(s.material().ambient(), 1.0);
    assert_eq!(s.material().diffuse(), 1.0);
    assert_eq!(s.material().specular(), 1.0);
    assert_eq!(s.material().shininess(), 100.0);

    assert_ne!(s.material(), Material::default());
    assert_eq!(s.material(), m);
}
