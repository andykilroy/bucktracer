use std::fs::File;
use exitfailure::ExitFailure;

use bucktracer::wavefront;
use bucktracer::*;
use bucktracer::math::*;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "Shows the model represented by a wavefront obj file", rename_all = "kebab-case")]
struct CmdOptions {
    // TODO from: (f64, f64, f64)
    // TODO to: (f64, f64, f64)
    // TODO up: (f64, f64, f64)

    /// The field of view of the camera, stated in degrees.
    #[structopt(long="fov", default_value="90.0")]
    fov_degrees: f64,

    /// The horizontal number of pixels in the output image.
    #[structopt(short="h", long="hsize", default_value="800")]
    hsize: u32,

    /// The vertical number of pixels in the output image.
    #[structopt(short="v", long="vsize", default_value="600")]
    vsize: u32,

    /// The input obj file
    #[structopt(parse(from_os_str))]
    objfile: std::ffi::OsString,
}

fn main() -> Result<(), ExitFailure> {
    let args: CmdOptions = CmdOptions::from_args();

    let mut f = File::open(&args.objfile)?;
    let objects = wavefront::read_object_vec(&mut f)?;

    let light = point_light(point(10.0, 10.0, -10.0), RGB::white());
    let world = World::with(vec![light], vec![group(objects)]);
    let mut cam = Camera::new(args.hsize, args.vsize, args.fov_degrees);
    cam.set_view_transform(view_transform(point(0.0, 0.0, -5.0), point(0.0, 0.0, 0.0), vector(0.0, 1.0, 0.0)));
    let canvas = cam.render(&world);
    let mut stdout = std::io::stdout();

    ppm::encode(&canvas, &mut stdout)?;
    Ok(())
}
