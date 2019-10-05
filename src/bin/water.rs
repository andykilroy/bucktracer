use bucktracer::*;
use exitfailure::ExitFailure;
use std::f64::consts::{FRAC_PI_2, FRAC_PI_3, FRAC_PI_4};
use std::io::stdout;

fn main() -> Result<(), ExitFailure> {
    let water = *Material::default()
        .set_pattern(Pattern::solid(colour(0.90, 0.95, 1.00)))
        .set_transparency(0.8)
        .set_refractive_index(1.2);
    let red_matrl = *Material::default()
        .set_pattern(Pattern::solid(colour(1.0, 0.0, 0.0)));
    let green_matrl = *Material::default()
        .set_pattern(Pattern::solid(colour(0.0, 1.0, 0.0)));

    let mut floor = plane();
    floor.set_material(*Material::default()
        .set_pattern(Pattern::checkers(RGB::white(), RGB::black()))
    );
    floor.set_object_to_world_spc(translation(0.0, -10.0, 0.0));

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
        translation(1.0, -1.0, 0.0)
    );

    let light = point_light(point(-10.0, 10.0, -10.0), RGB::white());
    let world = World::with(
        vec![light],
        vec![floor, above, below, water_surface],
    );
    let mut cam = Camera::new(2560, 1600, FRAC_PI_2);
    cam.set_view_transform(view_transform(
        point(0.0, 1.5, -5.0),
        point(0.0, 0.0, 0.0),
        vector(0.0, 1.0, 0.0),
    ));
    let canvas = cam.render(&world);
    let mut stdout = stdout();
    encode_ppm(&canvas, &mut stdout)?;
    Ok(())
}
