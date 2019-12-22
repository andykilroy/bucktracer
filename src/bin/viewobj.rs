use std::fs::File;
use exitfailure::ExitFailure;

use bucktracer::wavefront;
use bucktracer::*;
use bucktracer::math::*;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "viewobj",
    about = "Shows the model represented by a wavefront obj file",
    rename_all = "kebab-case",
)]
struct CmdOptions {

    /// The position of the camera.
    #[structopt(long="from", default_value="(0.0, 0.0, -1.0)", parse(try_from_str))]
    from: Tuple4,

    /// The position the camera points at.
    #[structopt(long="to", default_value="(0.0, 0.0, 0.0)", parse(try_from_str))]
    to: Tuple4,

    /// Which direction is considered 'up'
    #[structopt(long="up", default_value="(0.0, 1.0, 0.0)", parse(try_from_str))]
    up: Tuple4,

    /// The position of the light source
    #[structopt(long="light-pos", default_value="(10.0, 10.0, -10.0)", parse(try_from_str))]
    light_pos: Tuple4,

    /// The position of the light source
    #[structopt(long="light-colour", default_value="(1.0, 1.0, 1.0)", parse(try_from_str))]
    light_colour: Tuple4,

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

    let light = point_light(
        args.light_pos,
        colour(args.light_colour.x(), args.light_colour.y(), args.light_colour.z())
    );
    let world = World::with(vec![light], vec![group(objects)]);
    let mut cam = Camera::new(args.hsize, args.vsize, args.fov_degrees.to_radians());
    cam.set_view_transform(view_transform(args.from, args.to, vector(args.up.x(), args.up.y(), args.up.z())));
    let canvas = cam.render(&world);
    let mut stdout = std::io::stdout();

    ppm::encode(&canvas, &mut stdout)?;
    Ok(())
}
