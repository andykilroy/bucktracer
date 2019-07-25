use bucktracer::*;
use std::io::{Result, stdout};
use std::f64::consts::*;

fn main() -> Result<()> {

    let mut cam = camera(canvas(300, 300),
                     (0.0, 0.0, -10.0), (300.0, 300.0, -10.0),
                     (0.0, 0.0, 1.0));
    let light = point_light_source(point(150.0, 150.0, 300.0));
    let mut s = unit_sphere();
    s.set_transform(&(scaling(10.0, 10.0, 10.0) * translation(150.0, 150.0, 50.0)));

    raytrace(&mut cam, &light, &s);
    let mut stdout = stdout();
    encode_ppm(&cam.canvas, &mut stdout)
}

fn raytrace(cam: &mut Camera, light: &PointLightSource, spher: &Sphere) {
    // 1. Calculate the set of rays that point from each pixel to the
    // light source
    // 2. Calculate which rays intersection with the sphere
    // 3. For each ray decide whether to paint the pixel
}

type Triple = (f64, f64, f64);

fn camera(c: Canvas, l_left: Triple, u_right: Triple, normal: Triple) -> Camera {
    Camera::new(c, BoundedPlane {
        lower_left: point(l_left.0, l_left.1, l_left.2),
        upper_right: point(u_right.0, u_right.1, u_right.2),
        surface_normal: vector(normal.0, normal.1, normal.2),
    })
}

struct Camera {
    canvas: Canvas,
    plane: BoundedPlane
}

impl Camera {
    fn new(canv: Canvas, pln: BoundedPlane) -> Camera {
        Camera {
            canvas : canv,
            plane: pln
        }
    }
}

struct BoundedPlane {
    lower_left: Tuple4,
    upper_right: Tuple4,
    surface_normal: Tuple4
}

struct PointLightSource {
    position: Tuple4,
}

fn point_light_source(p: Tuple4) -> PointLightSource {
    PointLightSource {
        position: p,
    }
}

