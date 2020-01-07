use std::fs::File;
use exitfailure::ExitFailure;

use bucktracer::wavefront;

use structopt::StructOpt;
use bucktracer::wavefront::ParseError;
use bucktracer::wavefront::PolygonPoint;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "scanobj",
    about = "A tool to output statistics about a wavefront obj file",
    rename_all = "kebab-case",
)]
struct CmdOptions {
    #[structopt(parse(from_os_str))]
    /// List of files to scan
    objfiles: Vec<std::ffi::OsString>,
}

#[derive(Debug)]
struct Counter {
    min_bound: (f64, f64, f64),
    max_bound: (f64, f64, f64),
    vertices: usize,
    normals: usize,
    polygons: usize,
    groups: usize,
}

impl Counter {
    fn new() -> Counter {
        let max = std::f64::MAX;
        let min = std::f64::MIN;
        Counter { 
            min_bound: (max, max, max),
            max_bound: (min, min, min),
            vertices: 0,
            normals: 0,
            polygons: 0,
            groups: 0,
        }
    }

    fn centre (&self) -> (f64, f64, f64) {
        (
            (self.min_bound.0 + self.max_bound.0) / 2.0, 
            (self.min_bound.1 + self.max_bound.1) / 2.0, 
            (self.min_bound.2 + self.max_bound.2) / 2.0
        )
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

    fn handle_normal(&mut self, _x: f64, _y: f64, _z: f64) -> Result<(), ParseError> {
        self.normals += 1;
        Ok(())
    }

    fn handle_polygon(&mut self, _points: &[PolygonPoint]) -> Result<(), ParseError> {
        self.polygons += 1;
        Ok(())
    }

    fn declare_group(&mut self, _name: &str) -> Result<(), ParseError> {
        self.groups += 1;
        Ok(())
    }
}

fn main() -> Result<(), ExitFailure> {
    let args: CmdOptions = CmdOptions::from_args();

    for file in &args.objfiles {
        let mut f = File::open(&file)?;
        let mut c = Counter::new();
        wavefront::parse(&mut c, &mut f)?;
        println!("filepath    {}", file.to_string_lossy());
        println!("min_bound   {:.6} {:.6} {:.6}", c.min_bound.0, c.min_bound.1, c.min_bound.2);
        println!("max_bound   {:.6} {:.6} {:.6}", c.max_bound.0, c.max_bound.1, c.max_bound.2);
        println!("centre      {:.6} {:.6} {:.6}", c.centre().0, c.centre().1, c.centre().2);
        println!("vertices    {}", c.vertices);
        println!("polygons    {}", c.polygons);
        println!("groups      {}", c.groups);
    }
    Ok(())
}


