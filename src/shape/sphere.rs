use crate::*;

/// Creates a sphere of radius 1 centred at the origin.
pub fn unit_sphere() -> Object {
    Object {
        world_to_object_spc: identity(),
        material: Material::default(),
        shape: Shape::Sphere,
    }
}

/// Creates a transparent sphere of radius 1 centred at the origin.
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


pub fn intersect_sphere(r: &Ray, sphere: &Object) -> Option<(Intersection, Intersection)> {
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

