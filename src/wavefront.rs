
use crate::*;
use std::io;
use std::io::{BufReader, BufRead};
use std::vec::Vec;
use std::f64;
use crate::wavefront::ParseError::BadInstruction;

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

fn triple(s: &str) -> Result<(&str, &str, &str), ParseError> {
    let mut splitted = s.split_whitespace();
    if let Some(s1) = splitted.next() {
        if let Some(s2) = splitted.next() {
            if let Some(s3) = splitted.next() {
                return Ok((s1, s2, s3));
            }
        }
    }
    Err(ParseError::BadInstruction)
}

fn read_point(triplet: &str, points: &mut Vec<Tuple4>) -> Result<(), ParseError> {
    let (s1, s2, s3) = triple(triplet)?;
    let x1 = s1.parse::<f64>().or_else(|e| Err(BadInstruction))?;
    let x2 = s2.parse::<f64>().or_else(|e| Err(BadInstruction))?;
    let x3 = s3.parse::<f64>().or_else(|e| Err(BadInstruction))?;
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
