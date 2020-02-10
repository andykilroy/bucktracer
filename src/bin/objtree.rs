use std::fs::File;
use std::io::Write;

use exitfailure::ExitFailure;

use bucktracer::wavefront;
use bucktracer::*;
use bucktracer::math::*;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
name = "objtree",
about = "Shows the number of objects at nodes in an object tree",
rename_all = "kebab-case",
)]
struct CmdOptions {
    /// The input obj file
    #[structopt(parse(from_os_str))]
    objfile: std::ffi::OsString,

}

fn main() -> Result<(), ExitFailure> {
    let args: CmdOptions = CmdOptions::from_args();
    let mut f = File::open(&args.objfile)?;
    let objects = wavefront::read_object_vec(&mut f)?;
    let lvl = 0;
    let g = group(objects);
    count_all(lvl, &g);
    Ok(())
}

fn count_all(lvl: usize, node: &Object) -> usize {
    let mut c = 0;
    if node.children().len() > 0 {
        let next = lvl + 1;
        for ch in node.children().iter() {
            c += count_all(next, ch);
        }
        println!("{}\t{}", lvl, c);
    } else {
        c += 1;
    }
    c
}
