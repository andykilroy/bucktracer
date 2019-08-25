use bucktracer::*;
use std::io::{Result, stdout};

fn main() -> Result<()> {

    let mut cam = camera(canvas(300, 300),
                     (0.0, 0.0, 0.0), (60.0, 60.0, 0.0),
                     (0.0, 0.0, 1.0));
    let light = point_light(point(30.0, 30.0, 40.0), white());
    let mut s = unit_sphere();
    s.set_transform(&(translation(30.0, 30.0, 20.0) * scaling(7.5, 7.5, 7.5)));

    raytrace(&mut cam, &light, &s);
    let mut stdout = stdout();
    encode_ppm(cam.canvas(), &mut stdout)
}

fn raytrace(cam: &mut OldCamera, light: &RadialLightSource, spher: &Sphere) {
    let origins_rays = rays_between(cam, light);

    for (p, r) in origins_rays.iter() {
        let intersects = intersect(r, spher);
        if intersects.len() > 0 {
            cam.paint_colour_at(p.0, p.1, colour(1.0, 0.0, 0.0));
        }
    }
}

fn rays_between(cam: &mut OldCamera, light: &RadialLightSource) -> Vec<(Coord, Ray)> {
    let mut v : Vec<(Coord, Ray)> = vec![];

    for col in 0..cam.canvas().width {
        for row in 0..cam.canvas().height {
            let point_of_canvas = cam.pixel_to_point(col, row);
            v.push(((col, row), ray_to_point(point_of_canvas, light.position())))
        }
    }

    return v;
}



