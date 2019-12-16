use crate::wavefront::ParseError::BadInstruction;
use crate::*;
use std::f64;
use std::io;
use std::io::{BufRead, BufReader};
use std::vec::Vec;
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    Io,
    BadInstruction,
}

impl From<&ParseError> for ParseError {
    fn from(x: &ParseError) -> Self {
        (*x).clone()
    }
}

struct ParseState {
    pub vertices : Vec<Tuple4>,
    pub group_name: String,
    pub groups : BTreeMap<String, Vec<Object>>,
}

impl ParseState {
    pub fn new() -> Self {
        let mut groups : BTreeMap<String, Vec<Object>> = BTreeMap::new();
        groups.insert("".to_string(), vec![]);
        ParseState { vertices: vec![], group_name: "".to_string(), groups }
    }

    pub fn to_vec(&self) -> Vec<Object> {
        let mut output: Vec<Object> = vec![];
        for (name, members) in self.groups.iter() {
            if name == "" {
                output.extend_from_slice(members);
            } else {
                output.push(group(members.clone()));
            }
        }
        output
    }
}

pub fn parse(input: &mut dyn io::Read) -> Result<Vec<Object>, ParseError> {
    let mut bufread = BufReader::new(input);
    let mut state = ParseState::new();

    let mut lines = bufread.lines();
    while let Some(l) = lines.next() {
        let line = l.or_else(|e| Err(ParseError::Io))?;
        handle_line(line, &mut state)?;
    }

    Ok(state.to_vec())
}

fn handle_line(line: String, state: &mut ParseState) -> Result<(), ParseError>  {
    if line.starts_with("v ") {
        read_point(&line[2..], &mut state.vertices)
    } else if line.starts_with("f ") {
        read_facet(&line[2..], state)
    } else if line.starts_with("g ") {
        handle_group(&line[2..], state)
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

fn read_facet(args: &str, state: &mut ParseState) -> Result<(), ParseError> {
    fn gt_zero(i: usize) -> Result<usize, ParseError> {
        if i > 0 {
            Ok(i - 1)
        } else {
            Err(ParseError::BadInstruction)
        }
    }

    let indices : Vec<Result<usize, ParseError>> = args
        .split_whitespace()
        .map(|x| x.parse::<usize>().or_else(|e| Err(BadInstruction)))
        .collect();

    if indices.len() < 3 { return Err(BadInstruction); }

    let points = &state.vertices;
    let triangles = state.groups.get_mut(&state.group_name).unwrap();

    let first = indices[0].as_ref()?;
    for i in 1..=(indices.len() - 2) {
        let x1 = first;
        let x2 = indices[i].as_ref()?;
        let x3 = indices[i+1].as_ref()?;
        triangles.push(triangle(points[gt_zero(*x1)?], points[gt_zero(*x2)?], points[gt_zero(*x3)?]));
    }
    Ok(())
}

fn handle_group(newgroup: &str, state: &mut ParseState) -> Result<(), ParseError> {
    state.groups.insert(newgroup.to_string(), vec![]);
    state.group_name = newgroup.to_string();
    Ok(())
}
