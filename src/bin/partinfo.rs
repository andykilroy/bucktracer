use std::fs::File;
use std::io::Write;

use exitfailure::ExitFailure;

use bucktracer::wavefront;
use bucktracer::*;
use bucktracer::math::*;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
name = "partinfo",
about = "Shows bounding box information for a model after it has passed through a binary partitioning operation",
rename_all = "kebab-case",
)]
struct CmdOptions {
    #[structopt(short="b")]
    before_partitioning: bool,

    /// The input obj file
    #[structopt(parse(from_os_str))]
    objfile: std::ffi::OsString,

}

fn main() -> Result<(), ExitFailure> {
    let args: CmdOptions = CmdOptions::from_args();
    let mut f = File::open(&args.objfile)?;
    let objects = wavefront::read_object_vec(&mut f)?;
    if args.before_partitioning {
        for obj in objects.iter() {
            println!("{}", obj.bounds());
        }
    } else {
        let root = binary_partition(2, objects);
        print_bounding_box_info(0, 3, &root)?;
    }
    Ok(())
}

fn print_bounding_box_info(lvl: usize, upto: usize, node: &Object) -> Result<(), ExitFailure> {
    if lvl < upto {
        println!("{}\t{}", lvl, node.bounds());
        let children = node.children();
        let nextlvl = lvl + 1;
        for child in children.iter() {
            print_bounding_box_info(nextlvl, upto, child)?;
        }
    }
    // TODO print the number of children contained by this bounding box
    Ok(())
}
