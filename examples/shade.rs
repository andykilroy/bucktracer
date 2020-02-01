use bucktracer::*;
use bucktracer::math::*;
use bucktracer::ppm;
use std::f64::consts::FRAC_PI_2;
use std::io::{stdout, Result};

fn main() -> Result<()> {
    let mut materl = Material::default();
    materl.set_pattern(Pattern::solid(colour(1.0, 0.2, 1.0)));

    let light = point_light(point(0.0, 60.0, -20.0), RGB::white());
    let mut s = unit_sphere();
    s.set_material(materl);
    s.set_object_to_world_spc(translation(30.0, 30.0, 20.0) * scaling(7.5, 7.5, 7.5));
    let world = World::with(vec![light], vec![s]);

    let mut cam = Camera::new(300, 300, FRAC_PI_2);
    cam.orient(
        point(30.0, 30.0, 0.0),
        point(30.0, 30.0, 20.0),
        vector(0.0, 1.0, 0.0),
    );
    let canv = cam.render(&world);

    let mut stdout = stdout();
    ppm::encode(&canv, &mut stdout)
}
