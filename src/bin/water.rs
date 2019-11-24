use bucktracer::*;
use bucktracer::math::*;
use bucktracer::ppm;
use exitfailure::ExitFailure;
use std::f64::consts::FRAC_PI_2;
use std::io::stdout;

fn main() -> Result<(), ExitFailure> {
    let water = *Material::default()
        .set_pattern(Pattern::solid(colour(0.1, 0.1, 0.1)))
        .set_reflective(0.55)
        .set_transparency(0.4)
        .set_refractive_index(1.33);
    let red_matrl = *Material::default()
        .set_pattern(Pattern::solid(colour(1.0, 0.0, 0.0)));
    let green_matrl = *Material::default()
        .set_pattern(Pattern::solid(colour(0.0, 1.0, 0.0)));
    let orange_matrl = *Material::default()
        .set_pattern(Pattern::solid(colour(1.0, 0.5, 0.0)))
        .set_ambient(1.0)
        .set_specular(0.0);
    let silver = *Material::default()
        .set_pattern(Pattern::solid(colour(0.8, 0.8, 0.8)))
        .set_reflective(0.5);

    let mut floor = plane();
    floor.set_material(*Material::default()
        .set_pattern(Pattern::checkers(RGB::white(), RGB::black()))
        .set_pattern_to_object_spc(scaling(2.0, 2.0, 2.0))
    );
    floor.set_object_to_world_spc(translation(0.0, -10.0, 0.0));

    let mut sky = plane();
    sky.set_material(*Material::default()
        .set_pattern(Pattern::solid(colour(0.4726, 0.8281, 1.0)))
    );
    sky.set_object_to_world_spc(translation(0.0, 0.0, 5000.0) * rotation_x(FRAC_PI_2));

    let mut water_surface = plane();
    water_surface.set_material(water);

    let mut above = unit_sphere();
    above.set_material(red_matrl);
    above.set_object_to_world_spc(
        translation(-1.0, 1.0, 0.0)
    );

    let mut below = above.clone();
    below.set_material(green_matrl);
    below.set_object_to_world_spc(
        translation(1.0, -0.75, 0.0)
    );

    let mut sun = unit_sphere();
    sun.set_material(orange_matrl);
    sun.set_object_to_world_spc(
        translation(0.0, 300.0, 4000.0) * scaling(400.0, 400.0, 400.0)
    );

    let mut cyl = cylinder(CylKind::Closed, -1.0, 1.0);
    cyl.set_material(silver);
    cyl.set_object_to_world_spc(
        translation(3.0, 0.0, 3.0)
    );

    let light = point_light(point(-10.0, 10.0, -10.0), RGB::white());
    let world = World::with(
        vec![light],
        vec![floor, above, below, cyl, water_surface, sun, sky],
    );
    let mut cam = Camera::new(1280, 800, FRAC_PI_2);
    cam.set_view_transform(view_transform(
        point(0.0, 1.5, -5.0),
        point(0.0, 0.0, 0.0),
        vector(0.0, 1.0, 0.0),
    ));
    let canvas = cam.render(&world);
    let mut stdout = stdout();
    ppm::encode(&canvas, &mut stdout)?;
    Ok(())
}
