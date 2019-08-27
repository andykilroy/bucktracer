use bucktracer::*;
use std::io::{Result, stdout};
use std::f64::consts::FRAC_PI_2;

fn main() -> Result<()> {

    let mut materl = Material::default();
    materl.set_colour(colour(1.0, 0.2, 1.0));
    let light = point_light(point(0.0, 60.0, -20.0), white());
    let mut s = unit_sphere();
    s.set_material(&materl);
    s.set_transform(&(translation(30.0, 30.0, 20.0) * scaling(7.5, 7.5, 7.5)));
    let world = World::with(vec![s], vec![light]);

    let mut cam = Camera::new(300, 300, FRAC_PI_2);
    cam.set_transform(view_transform(
        point(30.0, 30.0, 0.0),
        point(30.0, 30.0, 20.0),
        vector(0.0, 1.0, 0.0)));
    let canv = cam.render(&world);

    let mut stdout = stdout();
    encode_ppm(&canv, &mut stdout)
}

fn raytrace(
    cam: &mut OldCamera,
    light: &RadialLightSource,
    spher: &Sphere,
    rays_target: Tuple4
) {
    let canvas_rays = rays_to_point(cam, rays_target);

    for (p, r) in canvas_rays.iter() {
        let intersects = intersect(r, spher);
        hit(intersects).and_then( |h| {
            let point_on_surface = position(r.clone(), h.t_value);
            let c = lighting(light,
                             point_on_surface,
                             normal_at(spher, point_on_surface),
                             &spher.material(),
                             -r.direction);
            cam.paint_colour_at(p.0, p.1, c);
            Some(h)
        });
    }
}

fn rays_to_point(cam: &mut OldCamera, target: Tuple4) -> Vec<(Coord, Ray)> {
    let mut v : Vec<(Coord, Ray)> = vec![];

    for col in 0..cam.canvas().width {
        for row in 0..cam.canvas().height {
            let point_of_canvas = cam.pixel_to_point(col, row);
            let r = ray_to_point(point_of_canvas, target);
            let normed = ray(r.origin, r.direction.normalize());
            v.push(((col, row), normed))
        }
    }

    return v;
}



