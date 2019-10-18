use bucktracer::*;
use exitfailure::ExitFailure;
use serde::Deserialize;
use std::io::stdout;
use std::path::PathBuf;

use structopt::StructOpt;

fn main() -> Result<(), ExitFailure> {
    let args = CommandArgs::from_args();
    let s = std::fs::read_to_string(args.file)?;
    let config: Config = toml::from_str(s.as_str())?;
    let world = config.world();
    let canvas = config.camera().render(&world);
    let mut stdout = stdout();
    encode_ppm(&canvas, &mut stdout)?;
    Ok(())
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Read in a scene", rename_all = "kebab-case")]
struct CommandArgs {
    file: PathBuf,
}

#[derive(Debug, Deserialize)]
struct Config {
    world: WorldConfig,
    camera: CameraConfig,
}

impl Config {
    fn world(self: &Self) -> World {
        World::with(
            self.world.lights.iter().map(to_light).collect(),
            self.world.objects.iter().map(to_object).collect(),
        )
    }

    fn camera(self: &Self) -> Camera {
        let CameraConfig {
            hsize,
            vsize,
            fov_as_degrees,
            from,
            to,
            up,
        } = self.camera;
        let mut camera = Camera::new(hsize, vsize, radians(fov_as_degrees));
        camera.set_view_transform(view_transform(
            point(from.0, from.1, from.2),
            point(to.0, to.1, to.2),
            vector(up.0, up.1, up.2),
        ));
        camera
    }
}

fn to_object(conf: &ObjectConfig) -> Object {
    let mut s = match conf.shape {
        Shape::Sphere => unit_sphere(),
        Shape::Plane => plane(),
        Shape::Cube => cube(),
        Shape::Cylinder (l, u) => cylinder(l, u),
    };

    s.set_object_to_world_spc(transform_matrix(conf.transforms.clone()));
    conf.material.clone().and_then(|mat| {
        s.set_material(mat.as_material());
        Some(mat)
    });

    s
}

fn transform_matrix(v: Vec<Transform>) -> Matrix {
    let acc = identity();
    v.iter().fold(acc, |x, item| item.matrix() * x)
}

fn to_light(conf: &LightSourceConfig) -> RadialLightSource {
    point_light(
        point(conf.position.0, conf.position.1, conf.position.2),
        conf.intensity,
    )
}

fn radians(x: f64) -> f64 {
    x * std::f64::consts::PI / 180.0
}

#[derive(Debug, Clone, Deserialize)]
struct WorldConfig {
    lights: Vec<LightSourceConfig>,
    objects: Vec<ObjectConfig>,
}

#[derive(Debug, Copy, Clone, Deserialize)]
struct CameraConfig {
    hsize: u32,
    vsize: u32,
    fov_as_degrees: f64,
    from: (f64, f64, f64),
    to: (f64, f64, f64),
    up: (f64, f64, f64),
}

#[derive(Debug, Copy, Clone, Deserialize)]
struct LightSourceConfig {
    position: (f64, f64, f64),
    intensity: RGB,
}

#[derive(Debug, Clone, Deserialize)]
struct ObjectConfig {
    transforms: Vec<Transform>,
    material: Option<MaterialConfig>,
    shape: Shape,
}

#[derive(Debug, Copy, Clone, Deserialize)]
enum Transform {
    Identity,
    Translate { x: f64, y: f64, z: f64 },
    Scale { x: f64, y: f64, z: f64 },
    RotateX { angle: f64 },
    RotateY { angle: f64 },
    RotateZ { angle: f64 },
}

impl Transform {
    fn matrix(self: &Self) -> Matrix {
        match *self {
            Transform::Identity => identity(),
            Transform::Translate { x, y, z } => translation(x, y, z),
            Transform::Scale { x, y, z } => scaling(x, y, z),
            Transform::RotateX { angle } => rotation_x(radians(angle)),
            Transform::RotateY { angle } => rotation_y(radians(angle)),
            Transform::RotateZ { angle } => rotation_z(radians(angle)),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
struct MaterialConfig {
    pattern: Option<Pattern>,
    #[serde(default = "default_transform")]
    pattern_to_object_spc: Vec<Transform>,
    colour: Option<RGB>,
    ambient: Option<f64>,
    diffuse: Option<f64>,
    specular: Option<f64>,
    shininess: Option<f64>,
}

fn default_transform() -> Vec<Transform> {
    vec![Transform::Identity]
}

impl MaterialConfig {
    fn as_material(&self) -> Material {
        let mut m = Material::default();
        if self.colour.is_some() {
            m.set_pattern(Pattern::solid(self.colour.unwrap()));
        }
        if self.pattern.is_some() {
            m.set_pattern(self.pattern.unwrap());
        }
        if !self.pattern_to_object_spc.is_empty() {
            m.set_pattern_to_object_spc(transform_matrix(self.pattern_to_object_spc.clone()));
        }
        if self.ambient.is_some() {
            m.set_ambient(self.ambient.unwrap());
        }
        if self.diffuse.is_some() {
            m.set_diffuse(self.diffuse.unwrap());
        }
        if self.specular.is_some() {
            m.set_specular(self.specular.unwrap());
        }
        if self.shininess.is_some() {
            m.set_shininess(self.shininess.unwrap());
        }
        m
    }
}

#[cfg(test)]
mod test {
    use crate::Config;
    use bucktracer::*;
    use std::f64::consts::{FRAC_PI_2, FRAC_PI_3, FRAC_PI_4};

    #[test]
    fn read_of_config() {
        let mat = Material::default()
            .set_pattern(Pattern::solid(colour(1.0, 0.9, 0.9)))
            .set_specular(0.0)
            .clone();

        let mut floor = unit_sphere();
        floor.set_object_to_world_spc(scaling(10.0, 0.01, 10.0));
        floor.set_material(mat);

        let mut left_wall = unit_sphere();
        left_wall.set_object_to_world_spc(
            translation(0.0, 0.0, 5.0)
                * rotation_y(-FRAC_PI_4)
                * rotation_x(FRAC_PI_2)
                * scaling(10.0, 0.01, 10.0),
        );
        left_wall.set_material(mat);

        let mut right_wall = unit_sphere();
        right_wall.set_object_to_world_spc(
            translation(0.0, 0.0, 5.0)
                * rotation_y(FRAC_PI_4)
                * rotation_x(FRAC_PI_2)
                * scaling(10.0, 0.01, 10.0),
        );
        right_wall.set_material(mat);

        let mut middle = unit_sphere();
        middle.set_object_to_world_spc(translation(-0.5, 1.0, 0.5));
        middle.set_material(
            Material::default()
                .set_pattern(Pattern::solid(colour(0.1, 1.0, 0.5)))
                .set_diffuse(0.7)
                .set_specular(0.3)
                .clone(),
        );

        let mut right = unit_sphere();
        right.set_object_to_world_spc(translation(1.5, 0.5, -0.5) * scaling(0.5, 0.5, 0.5));
        right.set_material(
            Material::default()
                .set_pattern(Pattern::solid(colour(0.5, 1.0, 0.1)))
                .set_diffuse(0.7)
                .set_specular(0.3)
                .clone(),
        );

        let mut left = unit_sphere();
        left.set_object_to_world_spc(translation(-1.5, 0.33, -0.75) * scaling(0.33, 0.33, 0.33));
        left.set_material(
            Material::default()
                .set_pattern(Pattern::solid(colour(0.5, 1.0, 0.1)))
                .set_diffuse(0.7)
                .set_specular(0.3)
                .clone(),
        );

        let conf = r##"
[camera]
hsize = 500
vsize = 250
fov_as_degrees = 60
from = [0.0, 1.5, -5.0]
to = [0.0, 1.0, 0.0]
up = [0.0, 1.0, 0.0]

[[world.lights]]
position = [-10.0, 10.0, -10.0]
intensity = [1.0, 1.0, 1.0]

# floor
[[world.objects]]
    shape = "Sphere"
    transforms = [ { Scale = {x = 10.0, y = 0.01, z = 10.0} } ]
    [world.objects.material]
        colour = [1.0, 0.9, 0.9]
        specular = 0.0

# right wall
[[world.objects]]
    shape = "Sphere"
    transforms = [
        { Scale = {x = 10.0, y = 0.01, z = 10.0} },
        { RotateX = {angle = 90.0} },
        { RotateY = {angle = 45.0} },
        { Translate = {x = 0.0, y = 0.0, z = 5.0} }
    ]
    [world.objects.material]
        colour = [1.0, 0.9, 0.9]
        specular = 0.0

# left wall
[[world.objects]]
    shape = "Sphere"
    transforms = [
        { Scale = {x = 10.0, y = 0.01, z = 10.0} },
        { RotateX = {angle = 90.0} },
        { RotateY = {angle = -45.0} },
        { Translate = {x = 0.0, y = 0.0, z = 5.0} }
    ]

    [world.objects.material]
        colour = [1.0, 0.9, 0.9]
        specular = 0.0

# left
[[world.objects]]
    shape = "Sphere"
    transforms = [
        { Scale = { x = 0.33, y = 0.33, z = 0.33} },
        { Translate = { x = -1.5, y = 0.33, z = -0.75} }
    ]
    [world.objects.material]
        colour = [0.5, 1.0, 0.1]
        diffuse = 0.7
        specular = 0.3

# middle
[[world.objects]]
    shape = "Sphere"
    transforms = [
        { Translate = { x = -0.5, y = 1.0, z = 0.5} }
    ]
    [world.objects.material]
        colour = [0.1, 1.0, 0.5]
        diffuse = 0.7
        specular = 0.3

# right
[[world.objects]]
    shape = "Sphere"
    transforms = [
        { Scale = { x = 0.5, y = 0.5, z = 0.5} },
        { Translate = { x = 1.5, y = 0.5, z = -0.5} }
    ]
    [world.objects.material]
        colour = [0.5, 1.0, 0.1]
        diffuse = 0.7
        specular = 0.3

        "##;
        let config: Config = toml::from_str(conf).unwrap();
        let lights = config.world().light_sources();
        assert_eq!(lights.len(), 1);
        assert_eq!(lights[0].position(), point(-10.0, 10.0, -10.0));
        assert_eq!(lights[0].intensity(), colour(1.0, 1.0, 1.0));

        let cam = config.camera();
        assert_eq!(cam.hsize(), 500);
        assert_eq!(cam.vsize(), 250);
        assert_eq!(true, almost_eq(cam.field_of_view(), FRAC_PI_3));
        assert_eq!(
            cam.view_transform(),
            view_transform(
                point(0.0, 1.5, -5.0),
                point(0.0, 1.0, 0.0),
                vector(0.0, 1.0, 0.0)
            )
        );

        assert_eq!(
            config.world().objects(),
            vec![floor, right_wall, left_wall, left, middle, right]
        );
    }

    #[test]
    fn read_gradient_and_striped_object() {
        let mat = Material::default()
            .set_pattern(Pattern::solid(colour(1.0, 0.9, 0.9)))
            .set_specular(0.0)
            .clone();

        let mut floor = unit_sphere();
        floor.set_object_to_world_spc(scaling(10.0, 0.01, 10.0));
        floor.set_material(mat);

        let mut left_wall = unit_sphere();
        left_wall.set_object_to_world_spc(
            translation(0.0, 0.0, 5.0)
                * rotation_y(-FRAC_PI_4)
                * rotation_x(FRAC_PI_2)
                * scaling(10.0, 0.01, 10.0),
        );
        left_wall.set_material(mat);

        let mut right_wall = unit_sphere();
        right_wall.set_object_to_world_spc(
            translation(0.0, 0.0, 5.0)
                * rotation_y(FRAC_PI_4)
                * rotation_x(FRAC_PI_2)
                * scaling(10.0, 0.01, 10.0),
        );
        right_wall.set_material(mat);

        let mut middle = unit_sphere();
        middle.set_object_to_world_spc(translation(-0.5, 1.0, 0.5));
        middle.set_material(
            Material::default()
                .set_pattern(Pattern::stripes(colour(0.1, 1.0, 0.5), RGB::white()))
                .set_diffuse(0.7)
                .set_pattern_to_object_spc(rotation_z(FRAC_PI_3) * translation(0.5, 0.0, 0.0))
                .set_specular(0.3)
                .clone(),
        );

        let mut right = unit_sphere();
        right.set_object_to_world_spc(translation(1.5, 0.5, -0.5) * scaling(0.5, 0.5, 0.5));
        right.set_material(
            Material::default()
                .set_pattern(Pattern::solid(colour(0.5, 1.0, 0.1)))
                .set_diffuse(0.7)
                .set_specular(0.3)
                .clone(),
        );

        let mut left = unit_sphere();
        left.set_object_to_world_spc(translation(-1.5, 0.33, -0.75) * scaling(0.33, 0.33, 0.33));
        left.set_material(
            Material::default()
                .set_pattern(Pattern::gradient(
                    colour(1.0, 0.0, 0.0),
                    colour(1.0, 1.0, 0.0),
                ))
                .set_pattern_to_object_spc(
                    rotation_z(FRAC_PI_3) * scaling(2.0, 2.0, 2.0) * translation(-0.5, 0.0, 0.0),
                )
                .set_diffuse(0.7)
                .set_specular(0.3)
                .clone(),
        );

        let conf = r##"
[camera]
hsize = 500
vsize = 250
fov_as_degrees = 60
from = [0.0, 1.5, -5.0]
to = [0.0, 1.0, 0.0]
up = [0.0, 1.0, 0.0]

[[world.lights]]
position = [-10.0, 10.0, -10.0]
intensity = [1.0, 1.0, 1.0]

# floor
[[world.objects]]
    shape = "Sphere"
    transforms = [ { Scale = {x = 10.0, y = 0.01, z = 10.0} } ]
    [world.objects.material]
        colour = [1.0, 0.9, 0.9]
        specular = 0.0

# right wall
[[world.objects]]
    shape = "Sphere"
    transforms = [
        { Scale = {x = 10.0, y = 0.01, z = 10.0} },
        { RotateX = {angle = 90.0} },
        { RotateY = {angle = 45.0} },
        { Translate = {x = 0.0, y = 0.0, z = 5.0} }
    ]
    [world.objects.material]
        colour = [1.0, 0.9, 0.9]
        specular = 0.0

# left wall
[[world.objects]]
    shape = "Sphere"
    transforms = [
        { Scale = {x = 10.0, y = 0.01, z = 10.0} },
        { RotateX = {angle = 90.0} },
        { RotateY = {angle = -45.0} },
        { Translate = {x = 0.0, y = 0.0, z = 5.0} }
    ]

    [world.objects.material]
        colour = [1.0, 0.9, 0.9]
        specular = 0.0

# left
[[world.objects]]
    shape = "Sphere"
    transforms = [
        { Scale = { x = 0.33, y = 0.33, z = 0.33} },
        { Translate = { x = -1.5, y = 0.33, z = -0.75} }
    ]
    [world.objects.material]
        pattern = { Gradient = { from = [1.0, 0.0, 0.0], to = [1.0, 1.0, 0.0] } }
        pattern_to_object_spc = [
            { Translate = [-0.5, 0.0, 0.0] },
            { Scale = {x = 2.0, y = 2.0, z = 2.0} },
            { RotateZ = {angle = 60.0}}
        ]
        diffuse = 0.7
        specular = 0.3

# middle
[[world.objects]]
    shape = "Sphere"
    transforms = [
        { Translate = { x = -0.5, y = 1.0, z = 0.5} }
    ]
    [world.objects.material]
        pattern = { Stripes = { a = [0.1, 1.0, 0.5], b = [1.0, 1.0, 1.0] } }
        pattern_to_object_spc = [
            { Translate = [0.5, 0.0, 0.0] },
            { RotateZ = {angle = 60.0}}
        ]
        diffuse = 0.7
        specular = 0.3

# right
[[world.objects]]
    shape = "Sphere"
    transforms = [
        { Scale = { x = 0.5, y = 0.5, z = 0.5} },
        { Translate = { x = 1.5, y = 0.5, z = -0.5} }
    ]
    [world.objects.material]
        colour = [0.5, 1.0, 0.1]
        diffuse = 0.7
        specular = 0.3

        "##;
        let config: Config = toml::from_str(conf).unwrap();
        let lights = config.world().light_sources();
        assert_eq!(lights.len(), 1);
        assert_eq!(lights[0].position(), point(-10.0, 10.0, -10.0));
        assert_eq!(lights[0].intensity(), colour(1.0, 1.0, 1.0));

        let cam = config.camera();
        assert_eq!(cam.hsize(), 500);
        assert_eq!(cam.vsize(), 250);
        assert_eq!(true, almost_eq(cam.field_of_view(), FRAC_PI_3));
        assert_eq!(
            cam.view_transform(),
            view_transform(
                point(0.0, 1.5, -5.0),
                point(0.0, 1.0, 0.0),
                vector(0.0, 1.0, 0.0)
            )
        );

        assert_eq!(
            config.world().objects(),
            vec![floor, right_wall, left_wall, left, middle, right]
        );
    }

    fn almost_eq(x1: f64, x2: f64) -> bool {
        f64::abs(x1 - x2) < 1e-5
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_converting_3tuple_to_RGB() {
        let c: RGB = RGB::from((0.5, 0.2, 0.7));
        assert_eq!(c, colour(0.5, 0.2, 0.7));
    }
}
