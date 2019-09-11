use bucktracer::*;
use std::f64::consts::{FRAC_PI_4, FRAC_PI_2, FRAC_PI_3};
use std::io::stdout;
use exitfailure::ExitFailure;

fn main() -> Result<(), ExitFailure>  {
    let mat = Material::default()
        .set_pattern(Pattern::solid(colour(1.0, 0.9, 0.9)))
        .set_specular(0.0).clone();

    let mut floor = unit_sphere();
    floor.set_transform(scaling(10.0, 0.01, 10.0));
    floor.set_material(mat);

    let mut left_wall = unit_sphere();
    left_wall.set_transform(
        translation(0.0, 0.0, 5.0) *
        rotation_y(-FRAC_PI_4) * rotation_x(FRAC_PI_2) *
        scaling(10.0, 0.01, 10.0)
    );
    left_wall.set_material(mat);

    let mut right_wall = unit_sphere();
    right_wall.set_transform(
        translation(0.0, 0.0, 5.0) *
        rotation_y(FRAC_PI_4) * rotation_x(FRAC_PI_2) *
        scaling(10.0, 0.01, 10.0)
    );
    right_wall.set_material(mat);

    let mut middle = unit_sphere();
    middle.set_transform(translation(-0.5, 1.0, 0.5));
    middle.set_material(Material::default()
        .set_pattern(Pattern::solid(colour(0.1, 1.0, 0.5)))
        .set_diffuse(0.7)
        .set_specular(0.3).clone());

    let mut right = unit_sphere();
    right.set_transform(translation(1.5, 0.5, -0.5) * scaling(0.5, 0.5, 0.5));
    right.set_material(Material::default()
        .set_pattern(Pattern::solid(colour(0.5, 1.0, 0.1)))
        .set_diffuse(0.7)
        .set_specular(0.3)
        .clone()
    );

    let mut left = unit_sphere();
    left.set_transform(translation(-1.5, 0.33, -0.75) * scaling(0.33, 0.33, 0.33));
    left.set_material(Material::default()
        .set_pattern(Pattern::solid(colour(0.5, 1.0, 0.1)))
        .set_diffuse(0.7)
        .set_specular(0.3)
        .clone()
    );

    let light = point_light(point(-10.0, 10.0, -10.0), RGB::white());
    let world = World::with(vec![light], vec![floor, right_wall, left_wall, left, middle, right]);
    let mut cam = Camera::new(500, 250, FRAC_PI_3);
    cam.set_transform(
        view_transform(
            point(0.0, 1.5, -5.0),
            point(0.0, 1.0, 0.0),
            vector(0.0, 1.0, 0.0)
        ));
    let canvas = cam.render(&world);
    let mut stdout = stdout();
    encode_ppm(&canvas, &mut stdout)?;
    Ok(())
}
