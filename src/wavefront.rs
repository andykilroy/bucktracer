
use crate::*;
use std::io;
use std::io::{BufReader, BufRead};
use std::vec::Vec;
use std::f64;

pub fn parse(input: &mut dyn io::Read) -> io::Result<Vec<Object>> {
    let mut bufread = BufReader::new(input);
    let mut vertices: Vec<Tuple4> = vec![];
    let mut tris: Vec<Object> = vec![];

    let mut lines = bufread.lines();
    while let Some(line) = lines.next() {
        handle_line(line?, &mut vertices, &mut tris);
    }

    Ok(tris)
}

fn handle_line(line: String, points: &mut Vec<Tuple4>, triangles: &mut Vec<Object>) {
    if line.starts_with("v ") {
        read_point(&line[2..], points)
    }
    else if line.starts_with("f "){
        read_facet(&line[2..], points, triangles)
    }
}

fn read_point(triplet: &str, points: &mut Vec<Tuple4>) {
    let splitted: Vec<&str> = triplet.split(" ").collect();
    let x1 = splitted[0].parse::<f64>().unwrap();
    let x2 = splitted[1].parse::<f64>().unwrap();
    let x3 = splitted[2].parse::<f64>().unwrap();
    points.push(point(x1, x2, x3));
}

fn read_facet(triplet: &str, points: &[Tuple4], triangles: &mut Vec<Object>) {
    let splitted: Vec<&str> = triplet.split(" ").collect();
    let x1 = splitted[0].parse::<usize>().unwrap();
    let x2 = splitted[1].parse::<usize>().unwrap();
    let x3 = splitted[2].parse::<usize>().unwrap();
    triangles.push(triangle(points[x1 - 1], points[x2 - 1], points[x3 - 1]));
}
