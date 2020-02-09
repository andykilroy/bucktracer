use crate::EPSILON;
use crate::math::*;
use crate::Ray;
use crate::Object;

use std::f64::INFINITY;

/// Describes an axis aligned bounding box
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Bounds {
    min: Tuple4,
    max: Tuple4,
}

impl Bounds {
    pub fn unit() -> Bounds {
        Bounds { min: point(-1.0, -1.0, -1.0), max: point(1.0, 1.0, 1.0) }
    }

    pub fn new(min: Tuple4, max: Tuple4) -> Bounds {
        Bounds { min, max }
    }

    pub fn min(&self) -> Tuple4 {
        self.min
    }
    pub fn max(&self) -> Tuple4 {
        self.max
    }
    pub fn all_corners(&self) -> [Tuple4 ; 8] {
        [
            point(self.min.x(), self.min.y(), self.min.z()),
            point(self.min.x(), self.min.y(), self.max.z()),
            point(self.min.x(), self.max.y(), self.min.z()),
            point(self.min.x(), self.max.y(), self.max.z()),
            point(self.max.x(), self.min.y(), self.min.z()),
            point(self.max.x(), self.min.y(), self.max.z()),
            point(self.max.x(), self.max.y(), self.min.z()),
            point(self.max.x(), self.max.y(), self.max.z()),
        ]
    }

    /// Finds the smallest bounding box that contains all given objects
    pub fn enclose(objects: &[Object]) -> Bounds {
        use std::f64::MIN;
        use std::f64::MAX;
        let max: Tuple4 = objects.iter().map(|o| o.bounds().max()).fold(
            tuple(MIN, MIN, MIN, MIN),
            |acc, next| Tuple4::max(acc, next)
        );
        let min: Tuple4 = objects.iter().map(|o| o.bounds().min()).fold(
            tuple(MAX, MAX, MAX, MAX),
            |acc, next| Tuple4::min(acc, next)
        );
        Bounds::new(min, max)
    }

    pub fn contains(&self, b: &Bounds) -> bool {
        (self.min.x() <= b.min.x()) && (b.min.x() <= b.max.x()) && (b.max.x() <= self.max.x())
            && (self.min.y() <= b.min.y()) && (b.min.y() <= b.max.y()) && (b.max.y() <= self.max.y())
            && (self.min.z() <= b.min.z()) && (b.min.z() <= b.max.z()) && (b.max.z() <= self.max.z())
            && (self.min.w() <= b.min.w()) && (b.min.w() <= b.max.w()) && (b.max.w() <= self.max.w())
    }

}

impl std::fmt::Display for Bounds {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Bounds {{ min: {}, max: {} }}", self.min, self.max)
    }
}

pub fn intersect_bounding_box(r: &Ray, bbox: Bounds) -> Option<(f64, f64)> {
    let (x_tmin, x_tmax) = check_axis(r.origin.x(), r.direction.x(), bbox.min().x(), bbox.max().x());
    let (y_tmin, y_tmax) = check_axis(r.origin.y(), r.direction.y(), bbox.min().y(), bbox.max().y());
    let (z_tmin, z_tmax) = check_axis(r.origin.z(), r.direction.z(), bbox.min().z(), bbox.max().z());

    let tmin = [x_tmin, y_tmin, z_tmin].iter().cloned().fold(std::f64::MIN, f64::max);
    let tmax = [x_tmax, y_tmax, z_tmax].iter().cloned().fold(std::f64::MAX, f64::min);

    if tmin > tmax {
        None
    } else {
        Some((tmin, tmax))
    }
}

fn check_axis(origin: f64, direction: f64, bbox_min: f64, bbox_max: f64) -> (f64, f64) {
    let tmin_numerator = bbox_min - origin;
    let tmax_numerator = bbox_max - origin;

    let (tmin, tmax) = if direction.abs() >= EPSILON {
        (tmin_numerator / direction,
         tmax_numerator / direction)
    } else {
        (tmin_numerator * INFINITY,
         tmax_numerator * INFINITY)
    };

    if tmin > tmax { (tmax, tmin) }
    else {(tmin, tmax)}
}
