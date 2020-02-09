use crate::*;
use std::f64;
use std::io;
use std::io::{BufRead, BufReader};
use std::vec::Vec;
use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;
use std::str::FromStr;

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

impl Error for ParseError {

}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::Io => write!(f, "Io"),
            ParseError::BadInstruction => write!(f, "BadInstruction"),
        }
    }
}

struct ParseState {
    pub vertices : Vec<Tuple4>,
    pub normals : Vec<Tuple4>,
    pub group_name: String,
    pub groups : BTreeMap<String, Vec<Object>>,
}

impl ParseState {
    pub fn new() -> Self {
        let mut groups : BTreeMap<String, Vec<Object>> = BTreeMap::new();
        groups.insert("".to_string(), vec![]);
        ParseState { vertices: vec![], normals: vec![], group_name: "".to_string(), groups }
    }

    fn handle_triangle(&mut self, i1: &PolygonPoint, i2: &PolygonPoint, i3: &PolygonPoint) -> Result<(), ParseError> {
        let points = &self.vertices;
        let group = self.groups.get_mut(&self.group_name).unwrap();
        let v1: Tuple4 = points[gt_zero(i1.vertex_index)?];
        let v2: Tuple4 = points[gt_zero(i2.vertex_index)?];
        let v3: Tuple4 = points[gt_zero(i3.vertex_index)?];

        if no_normals(i1, i2, i3) {
            group.push(triangle(v1, v2, v3));
        } else if all_have_normals (i1, i2, i3) {
            let normals = &self.normals;

            let n1: Tuple4 = normals[gt_zero(i1.normal_index.unwrap())?];
            let n2: Tuple4 = normals[gt_zero(i2.normal_index.unwrap())?];
            let n3: Tuple4 = normals[gt_zero(i3.normal_index.unwrap())?];
            group.push(smooth_triangle(v1, v2, v3, n1, n2, n3));
        } else {
            return Err(ParseError::BadInstruction);
        }

        Ok(())
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

#[derive(Debug, Clone)]
pub struct PolygonPoint {
    pub vertex_index: usize,
    pub normal_index: Option<usize>
}

impl FromStr for PolygonPoint {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splitted: Vec<&str> = s.split("/").collect();
        if splitted.len() == 3 {
            let vindex: usize = splitted[0].parse::<usize>().or_else(|_| Err(ParseError::BadInstruction))?;
            let nindex: usize = splitted[2].parse::<usize>().or_else(|_| Err(ParseError::BadInstruction))?;
            Ok(PolygonPoint {
                vertex_index: vindex,
                normal_index: Some(nindex)
            })
        } else if splitted.len() == 1 {
            let vindex: usize = splitted[0].parse::<usize>().or_else(|_| Err(ParseError::BadInstruction))?;
            Ok(PolygonPoint {
                vertex_index: vindex,
                normal_index: None
            })
        } else {
            Err(ParseError::BadInstruction)
        }
    }
}


fn no_normals(p1: &PolygonPoint, p2: &PolygonPoint, p3: &PolygonPoint) -> bool {
    match (p1.normal_index, p2.normal_index, p3.normal_index) {
        (None, None, None) => true,
        _ => false,
    }
}

fn all_have_normals(p1: &PolygonPoint, p2: &PolygonPoint, p3: &PolygonPoint) -> bool {
    match (p1.normal_index, p2.normal_index, p3.normal_index) {
        (Some(_), Some(_), Some(_)) => true,
        _ => false,
    }
}

pub trait ParseHandler {
    fn handle_vertex(&mut self, x: f64, y: f64, z: f64) -> Result<(), ParseError>;

    fn handle_normal(&mut self, x: f64, y: f64, z: f64) -> Result<(), ParseError>;

    fn handle_polygon(&mut self, points: &[PolygonPoint]) -> Result<(), ParseError>;

    fn declare_group(&mut self, name: &str) -> Result<(), ParseError> ;
}

impl ParseHandler for ParseState {
    fn handle_vertex(&mut self, x: f64, y: f64, z: f64) -> Result<(), ParseError> {
        self.vertices.push(point(x, y, z));
        Ok(())
    }

    fn handle_normal(&mut self, x: f64, y: f64, z: f64) -> Result<(), ParseError> {
        self.normals.push(vector(x, y, z));
        Ok(())
    }

    fn handle_polygon(&mut self, points: &[PolygonPoint]) -> Result<(), ParseError>{
        if points.len() < 3 { return Err(ParseError::BadInstruction); }

        let first = &points[0];
        for i in 1..=(points.len() - 2) {
            let x1 = first;
            let x2 = &points[i];
            let x3 = &points[i+1];
            self.handle_triangle(x1, x2, x3)?;
        }
        Ok(())
    }

    fn declare_group(&mut self, name: &str) -> Result<(), ParseError> {
        self.groups.insert(name.to_string(), vec![]);
        self.group_name = name.to_string();
        Ok(())
    }
}

pub fn read_object_vec(input: &mut dyn io::Read) -> Result<Vec<Object>, ParseError> {
    let mut state = ParseState::new();
    parse(&mut state, input)?;
    Ok(state.to_vec())
}

pub fn parse(handler: &mut dyn ParseHandler, input: &mut dyn io::Read) -> Result<(), ParseError> {
    let bufread = BufReader::new(input);

    let mut lines = bufread.lines();
    while let Some(l) = lines.next() {
        let line = l.or_else(|_| Err(ParseError::Io))?;
        handle_line(line, handler)?;
    }
    Ok(())
}

fn handle_line(line: String, handler: &mut dyn ParseHandler) -> Result<(), ParseError>  {
    if line.starts_with("v ") {
        read_point(&line[2..], handler)
    } else if line.starts_with("vn ") {
        read_normal(&line[3..], handler)
    } else if line.starts_with("f ") {
        read_facet(&line[2..], handler)
    } else if line.starts_with("g ") {
        handler.declare_group(&line[2..])
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

fn read_point(triplet: &str, handler: &mut dyn ParseHandler) -> Result<(), ParseError> {
    let (s1, s2, s3) = triple(triplet)?;
    let x1 = s1.parse::<f64>().or_else(|_| Err(ParseError::BadInstruction))?;
    let x2 = s2.parse::<f64>().or_else(|_| Err(ParseError::BadInstruction))?;
    let x3 = s3.parse::<f64>().or_else(|_| Err(ParseError::BadInstruction))?;
    handler.handle_vertex(x1, x2, x3)?;
    Ok(())
}

fn read_normal(triplet: &str, handler: &mut dyn ParseHandler) -> Result<(), ParseError> {
    let (s1, s2, s3) = triple(triplet)?;
    let x1 = s1.parse::<f64>().or_else(|_| Err(ParseError::BadInstruction))?;
    let x2 = s2.parse::<f64>().or_else(|_| Err(ParseError::BadInstruction))?;
    let x3 = s3.parse::<f64>().or_else(|_| Err(ParseError::BadInstruction))?;
    handler.handle_normal(x1, x2, x3)?;
    Ok(())
}

fn read_facet(args: &str, handler: &mut dyn ParseHandler) -> Result<(), ParseError> {
    let (parsed, errs): (Vec<Result<PolygonPoint, ParseError>>, Vec<Result<PolygonPoint, ParseError>>) = args
        .split_whitespace()
        .map(|x| x.parse::<PolygonPoint>().or_else(|_| Err(ParseError::BadInstruction)))
        .partition(|x| x.is_ok());

    if errs.len() > 0 {
        return Err(ParseError::BadInstruction);
    } else {
        let oks: Vec<PolygonPoint> = parsed.iter().cloned().map(|x| x.unwrap()).collect();
        handler.handle_polygon(&oks)
    }
}

fn gt_zero(i: usize) -> Result<usize, ParseError> {
    if i > 0 {
        Ok(i - 1)
    } else {
        Err(ParseError::BadInstruction)
    }
}

#[cfg(test)]
mod test_string_to_polygon_point_conversion {
    use super::*;

    #[allow(non_snake_case)]
    #[test]
    fn no_slashes_produce_no_normals() {
        let p : PolygonPoint = "8".parse::<PolygonPoint>().unwrap();
        assert_eq!(p.vertex_index, 8);
        assert_eq!(p.normal_index, None);
    }

    #[allow(non_snake_case)]
    #[test]
    fn with_slashes_produces_a_vertex_index_and_normal() {
        let p : PolygonPoint = "8//10".parse::<PolygonPoint>().unwrap();
        assert_eq!(p.vertex_index, 8);
        assert_eq!(p.normal_index, Some(10));
    }

    #[allow(non_snake_case)]
    #[test]
    fn allow_a_texture_index() {
        let p : PolygonPoint = "8/7/10".parse::<PolygonPoint>().unwrap();
        assert_eq!(p.vertex_index, 8);
        assert_eq!(p.normal_index, Some(10));
    }

    #[allow(non_snake_case)]
    #[test]
    fn dont_permit_negative_vertex_indices() {
        assert_eq!("-8//10".parse::<PolygonPoint>().is_err(), true);
    }

    #[allow(non_snake_case)]
    #[test]
    fn dont_permit_negative_normal_indices() {
        assert_eq!("8//-10".parse::<PolygonPoint>().is_err(), true);
    }

    #[allow(non_snake_case)]
    #[test]
    fn dont_permit_rational_numbers_for_vertex_index() {
        assert_eq!("8.34//2".parse::<PolygonPoint>().is_err(), true);
    }

    #[allow(non_snake_case)]
    #[test]
    fn dont_permit_rational_numbers_for_normal_index() {
        assert_eq!("8//2.1".parse::<PolygonPoint>().is_err(), true);
    }
}
