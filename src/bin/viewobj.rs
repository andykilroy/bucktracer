use std::fs::File;
use exitfailure::ExitFailure;

use bucktracer::wavefront;
use bucktracer::*;
use bucktracer::math::*;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "Shows the model represented by a wavefront obj file", rename_all = "kebab-case")]
struct CmdOptions {
    #[structopt(parse(from_os_str))]
    /// The input obj file
    objfile: std::ffi::OsString,
}

fn main() -> Result<(), ExitFailure> {
    let args: CmdOptions = CmdOptions::from_args();

    let mut f = File::open(&args.objfile)?;
    let objects = wavefront::read_object_vec(&mut f)?;

    let light = point_light(point(10.0, 10.0, -10.0), RGB::white());
    let world = World::with(vec![light], objects);
    let mut cam = Camera::new(800, 600, 90.0);
    cam.set_view_transform(view_transform(point(0.0, 0.0, -5.0), point(0.0, 0.0, 0.0), vector(0.0, 1.0, 0.0)));
    let canvas = cam.render(&world);
    let mut stdout = std::io::stdout();
    ppm::encode(&canvas, &mut stdout)?;
    Ok(())
}
