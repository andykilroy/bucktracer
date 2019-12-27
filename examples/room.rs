use bucktracer::*;
use bucktracer::math::*;
use bucktracer::ppm;
use exitfailure::ExitFailure;
use std::f64::consts::{FRAC_PI_2, FRAC_PI_3, FRAC_PI_4};
use std::io::stdout;

fn main() -> Result<(), ExitFailure> {
    let mat = *Material::default()
        .set_pattern(Pattern::solid(colour(1.0, 0.9, 0.9)))
        .set_specular(0.0);

    let mut floor = unit_sphere();
    floor.set_object_to_world_spc(scaling(10.0, 0.01, 10.0));
    floor.set_material(mat);

    let mut left_wall = unit_sphere();
    left_wall.set_object_to_world_spc(
        translation(0.0, 0.0, 5.0)
            * rotation_y(-FRAC_PI_4)
            * rotation_x(FRAC_PI_2)
            * scaling(10.0, 0.01, 10.0),
    );
    left_wall.set_material(mat);

    let mut right_wall = unit_sphere();
    right_wall.set_object_to_world_spc(
        translation(0.0, 0.0, 5.0)
            * rotation_y(FRAC_PI_4)
            * rotation_x(FRAC_PI_2)
            * scaling(10.0, 0.01, 10.0),
    );
    right_wall.set_material(mat);

    let mut middle = unit_sphere();
    middle.set_object_to_world_spc(translation(-0.5, 1.0, 0.5));
    middle.set_material(
        Material::default()
            .set_pattern(Pattern::stripes(colour(0.1, 1.0, 0.5), RGB::white()))
            .set_diffuse(0.7)
            .set_pattern_to_object_spc(rotation_z(FRAC_PI_3) * translation(0.5, 0.0, 0.0))
            .set_specular(0.3)
            .clone(),
    );

    let mut right = unit_sphere();
    right.set_object_to_world_spc(translation(1.5, 1.0, -0.5));
    right.set_material(
        Material::default()
            .set_pattern(Pattern::ring(colour(0.0, 0.75, 1.0), colour(0.0, 0.0, 1.0)))
            .set_pattern_to_object_spc(
                rotation_y(-FRAC_PI_3) * rotation_z(FRAC_PI_3) * scaling(0.1, 0.1, 0.1),
            )
            .set_diffuse(0.7)
            .set_specular(0.3)
            .clone(),
    );

    let mut left = unit_sphere();
    left.set_object_to_world_spc(translation(-1.5, 0.33, -0.75) * scaling(0.33, 0.33, 0.33));
    left.set_material(
        Material::default()
            .set_pattern(Pattern::gradient(
                colour(1.0, 0.0, 0.0),
                colour(1.0, 1.0, 0.0),
            ))
            .set_pattern_to_object_spc(
                rotation_z(FRAC_PI_3) * scaling(2.0, 2.0, 2.0) * translation(-0.5, 0.0, 0.0),
            )
            .set_diffuse(0.7)
            .set_specular(0.3)
            .clone(),
    );

    let light = point_light(point(-10.0, 10.0, -10.0), RGB::white());
    let world = World::with(
        vec![light],
        vec![floor, right_wall, left_wall, left, middle, right],
    );
    let mut cam = Camera::new(2560, 1600, FRAC_PI_3);
    cam.set_view_transform(view_transform(
        point(0.0, 1.5, -5.0),
        point(0.0, 1.0, 0.0),
        vector(0.0, 1.0, 0.0),
    ));
    let canvas = cam.render(&world);
    let mut stdout = stdout();
    ppm::encode(&canvas, &mut stdout)?;
    Ok(())
}
