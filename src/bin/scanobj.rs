use std::io;
use std::fs::File;
use exitfailure::ExitFailure;

use bucktracer::wavefront;

use structopt::StructOpt;
use bucktracer::wavefront::ParseError;

#[derive(Debug, StructOpt)]
#[structopt(about = "A tool to output statistics about a wavefront obj file", rename_all = "kebab-case")]
struct CmdOptions {
    objfile: String,
}

#[derive(Debug)]
struct Counter {
    min_bound: (f64, f64, f64),
    max_bound: (f64, f64, f64),
    vertices: usize,
}

impl Counter {
    fn new() -> Counter {
        let max = std::f64::MAX;
        let min = std::f64::MIN;
        Counter { min_bound: (max, max, max), max_bound: (min, min, min), vertices: 0 }
    }
}

impl wavefront::ParseHandler for Counter {
    fn handle_vertex(&mut self, x: f64, y: f64, z: f64) -> Result<(), ParseError> {
        fn max(x: f64, y: f64) -> f64 { if y > x {y} else {x} }
        fn min(x: f64, y: f64) -> f64 { if x < y {x} else {y} }
        self.max_bound = (
            max(x, self.max_bound.0),
            max(y, self.max_bound.1),
            max(z, self.max_bound.2),
        );
        self.min_bound = (
            min(x, self.min_bound.0),
            min(y, self.min_bound.1),
            min(z, self.min_bound.2),
        );
        self.vertices += 1;
        Ok(())
    }

    fn handle_triangle(&mut self, i1: usize, i2: usize, i3: usize) -> Result<(), ParseError> {
        Ok(())
    }

    fn declare_group(&mut self, name: &str) -> Result<(), ParseError> {
        Ok(())
    }
}

fn main() -> Result<(), ExitFailure> {
    let args: CmdOptions = CmdOptions::from_args();
    let mut f = File::open(args.objfile)?;
    let mut c = Counter::new();
    wavefront::parse(&mut c,&mut f)?;
    let stdin = std::io::stdin();
    println!("min_bound ({:.6} {:.6} {:.6})", c.min_bound.0, c.min_bound.1, c.min_bound.2);
    println!("max_bound ({:.6} {:.6} {:.6})", c.max_bound.0, c.max_bound.1, c.max_bound.2);
    println!("vertices  {}", c.vertices);
    Ok(())
}
