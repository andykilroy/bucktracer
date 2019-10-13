use crate::*;

pub fn unit_sphere() -> Object {
    Object {
        world_to_object_spc: identity(),
        material: Material::default(),
        shape: Shape::Sphere,
    }
}

pub fn glass_sphere() -> Object {
    let mut glass = Material::default();
    glass.set_transparency(1.0);
    glass.set_refractive_index(1.5);
    Object {
        world_to_object_spc: identity(),
        material: glass,
        shape: Shape::Sphere,
    }
}

pub fn plane() -> Object {
    Object {
        world_to_object_spc: identity(),
        material: Material::default(),
        shape: Shape::Plane,
    }
}

/// Determines what shape an object has.
///
/// Influences the calculation of surface normals and intersections.
#[derive(Debug, Copy, Clone, PartialEq, Deserialize)]
pub enum Shape {
    Sphere,
    Plane,
}

impl Shape {
    /// Calculates a normal appropriate for the object at the
    /// specified position.  The position is always in object
    /// co-ordinates.  The returned normal vector is also in
    /// object space.
    fn local_normal_at(self, position: Tuple4) -> Tuple4 {
        match self {
            Shape::Sphere => {
                // presume the sphere is centred at (0, 0, 0)
                position - point(0.0, 0.0, 0.0)
            }
            Shape::Plane => vector(0.0, 1.0, 0.0),
        }
    }
}

/// An object to be placed in the world.
///
/// The object has a transform that dictates where it is placed in
/// the world, and also whether it is scaled or rotated in any way.
/// It also is associated with material dictating its surface
/// properties.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Object {
    world_to_object_spc: Matrix,
    material: Material,
    shape: Shape,
}

impl Object {
    /// The transformation matrix to convert co-ordinates from
    /// object space to world space.
    pub fn object_to_world_spc(&self) -> Matrix {
        self.world_to_object_spc.inverse()
    }

    pub fn set_object_to_world_spc(self: &mut Self, m: Matrix) -> &mut Self {
        // The given matrix describes the object -> world coordinate transform.
        // The transformation from world -> object is performed more frequently.
        // In the interests of performance, store the world -> object transform,
        // so the inverse does not have to be computed all the time.
        self.world_to_object_spc = m.inverse();
        self
    }

    /// The transformation matrix to convert co-ordinates from
    /// world space to object space.
    pub fn world_to_object_spc(&self) -> Matrix {
        self.world_to_object_spc
    }

    pub fn material(&self) -> Material {
        self.material
    }

    pub fn set_material(self: &mut Self, m: Material) -> &mut Self {
        self.material = m;
        self
    }

    pub fn mut_material(&mut self) -> &mut Material {
        &mut self.material
    }

    pub fn normal_at(self: &Self, world_point: Tuple4) -> Tuple4 {
        let inversion_mat = self.world_to_object_spc();
        let object_point = inversion_mat.mult(world_point);
        let object_normal = self.shape.local_normal_at(object_point);
        let tmp = inversion_mat.transpose().mult(object_normal);

        tuple(tmp.x(), tmp.y(), tmp.z(), 0.0).normalize()
    }

    pub fn material_colour_at(self: &Self, world_point: Tuple4) -> RGB {
        let to_pattern_space = self.material().object_to_pattern_spc() * self.world_to_object_spc();
        let p = to_pattern_space.mult(world_point);
        self.material().pattern().colour_at(p)
    }
}


fn intersect_sphere(r: &Ray, sphere: &Object) -> Option<(Intersection, Intersection)> {
    // presume the sphere is centred at (0,0,0)
    let s_to_ray = r.origin - point(0.0, 0.0, 0.0);
    let a = r.direction.dot(r.direction);
    let b = 2.0 * r.direction.dot(s_to_ray);
    let c = s_to_ray.dot(s_to_ray) - 1.0;
    let discriminant = b.powf(2.0) - (4.0 * a * c);

    if discriminant < 0.0 {
        None
    } else {
        let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b + discriminant.sqrt()) / (2.0 * a);
        Some((intersection(t1, sphere), intersection(t2, sphere)))
    }
}

fn intersect_plane(r: &Ray, s: &Object) -> Option<Intersection> {
    if r.direction.y().abs() < EPSILON {
        None
    } else {
        Some(intersection(-r.origin.y() / r.direction.y(), s))
    }
}

pub fn append_intersects(orig: &Ray, s: &Object, vec: &mut Vec<Intersection>) {
    let to_object_space = s.world_to_object_spc();
    let r = orig.transform(&to_object_space);
    let shape = s.shape;
    match shape {
        Shape::Sphere => {
            if let Some((a, b)) = intersect_sphere(&r, s) {
                vec.push(a);
                vec.push(b);
            }
        }
        Shape::Plane => {
            if let Some(a) = intersect_plane(&r, s) {
                vec.push(a);
            }
        }
    }
}

#[cfg(test)]
mod test_shapes;
