use bucktracer::*;
use bucktracer::math::*;
use bucktracer::ppm;
use std::f64::consts::FRAC_PI_2;
use std::io::{stdout, Result};

fn main() -> Result<()> {
    let mut cam = Camera::new(300, 300, FRAC_PI_2);
    cam.orient(
        point(30.0, 30.0, 0.0),
        point(30.0, 30.0, 20.0),
        vector(0.0, 1.0, 0.0),
    );
    let light = point_light(point(30.0, 30.0, 40.0), RGB::white());
    let mut s = unit_sphere();
    s.set_object_to_world_spc(translation(30.0, 30.0, 20.0) * scaling(7.5, 7.5, 7.5));
    let world = World::with(vec![light], vec![s]);
    let canvas = raytrace(&cam, &world);
    let mut stdout = stdout();
    ppm::encode(&canvas, &mut stdout)
}

fn raytrace(cam: &Camera, world: &World) -> Canvas {
    let mut canv = canvas(cam.hsize() as usize, cam.vsize() as usize);
    let red = colour(1.0, 0.0, 0.0);
    let black = colour(0.0, 0.0, 0.0);

    for y in 0..cam.vsize() {
        for x in 0..cam.hsize() {
            let r = cam.ray_for_pixel(x, y);
            let intersects = world.intersect(&r);
            let col = match index_of_hit(&intersects) {
                None => red,
                Some(_) => black
            };

            canv.set_colour_at(x as usize, y as usize, col);
        }
    }
    canv
}
