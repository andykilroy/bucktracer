
use crate::*;
use std::io;
use std::io::{BufReader, BufRead};
use std::vec::Vec;
use std::f64;

#[derive(Debug, PartialEq)]
pub enum ParseError {
    Io,
    BadInstruction,
}

pub fn parse(input: &mut dyn io::Read) -> Result<Vec<Object>, ParseError> {
    let mut bufread = BufReader::new(input);
    let mut vertices: Vec<Tuple4> = vec![];
    let mut tris: Vec<Object> = vec![];

    let mut lines = bufread.lines();
    while let Some(l) = lines.next() {
        let line = l.or_else(|e| Err(ParseError::Io))?;
        handle_line(line, &mut vertices, &mut tris)?;
    }

    Ok(tris)
}

fn handle_line(line: String, points: &mut Vec<Tuple4>, triangles: &mut Vec<Object>) -> Result<(), ParseError>  {
    if line.starts_with("v ") {
        read_point(&line[2..], points)
    } else if line.starts_with("f "){
        read_facet(&line[2..], points, triangles)
    } else {
        Ok(())
    }
}

fn read_point(triplet: &str, points: &mut Vec<Tuple4>) -> Result<(), ParseError> {
    let splitted: Vec<&str> = triplet.split(" ").collect();
    let x1 = splitted[0].parse::<f64>().unwrap();
    let x2 = splitted[1].parse::<f64>().unwrap();
    let x3 = splitted[2].parse::<f64>().unwrap();
    points.push(point(x1, x2, x3));
    Ok(())
}

fn read_facet(triplet: &str, points: &[Tuple4], triangles: &mut Vec<Object>) -> Result<(), ParseError> {
    fn gt_zero(i: usize) -> Result<usize, ParseError> {
        if i > 0 { Ok(i - 1) }
        else { Err(ParseError::BadInstruction) }
    }

    let splitted: Vec<&str> = triplet.split(" ").collect();
    let x1 = splitted[0].parse::<usize>().unwrap();
    let x2 = splitted[1].parse::<usize>().unwrap();
    let x3 = splitted[2].parse::<usize>().unwrap();
    triangles.push(triangle(points[gt_zero(x1)?], points[gt_zero(x2)?], points[gt_zero(x3)?]));
    Ok(())
}
