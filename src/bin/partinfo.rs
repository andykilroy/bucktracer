use std::fs::File;

use exitfailure::ExitFailure;

use bucktracer::wavefront;
use bucktracer::*;

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

    /// The depth with which to run the binary partitioning algorithm
    #[structopt(long="depth", default_value="0")]
    depth: usize,

    /// The input obj file
    #[structopt(parse(from_os_str))]
    objfile: std::ffi::OsString,

}

fn main() -> Result<(), ExitFailure> {
    let args: CmdOptions = CmdOptions::from_args();
    let mut f = File::open(&args.objfile)?;
    let objects = wavefront::read_object_vec(&mut f)?;
    if args.before_partitioning {
        let root = group(objects);
        print_bounding_box_info(0, &root);
    } else {
        let map = bbox_map(args.depth, objects);
        for (ind, (k, v)) in map.iter().enumerate() {
            println!("{}\t{}\t{}", ind, v.len(), k);
        }
//        let root = map.groups();
//        print_bounding_box_info(0, &root);
    }
    Ok(())
}

fn print_bounding_box_info(lvl: usize, node: &Object) -> usize {
    let children = node.children();
    let mut c = 0;
    if children.len() > 0 {
        let nextlvl = lvl + 1;
        for child in children.iter() {
            c += print_bounding_box_info(nextlvl, child);
        }
        println!("{}\t{}\t{}", lvl, c, node.bounds());
    } else {
        c += 1;
    }
    return c;
}
