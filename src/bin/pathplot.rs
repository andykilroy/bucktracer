use bucktracer::*;
use bucktracer::ppm;
use std::io::Result;

fn main() -> Result<()> {
    let stone = Projectile {
        position: point(0.0, 1.0, 0.0),
        velocity: vector(1.0, 1.1, 0.0),
    };

    let env = Environment {
        gravity: vector(0.0, -0.1, 0.0),
        wind: vector(0.0, 0.0, 0.0),
    };

    let path = trace_path(&env, &stone);
    let c = plot(canvas(480, 270), 25.0, 14.0625, path);

    let mut stdout = std::io::stdout();

    ppm::encode(&c, &mut stdout)
}

#[derive(Debug, Clone)]
struct Projectile {
    position: Tuple4, // point
    velocity: Tuple4, // vector
}

struct Environment {
    gravity: Tuple4, // vector
    wind: Tuple4,
}

fn trace_path(env: &Environment, start: &Projectile) -> Vec<Projectile> {
    let mut v: Vec<Projectile> = vec![];

    let mut current: Projectile = start.clone();
    while current.position.y() > 0.0 {
        v.push(current.clone());
        current = tick(env, &current);
    }

    v
}

fn tick(env: &Environment, current: &Projectile) -> Projectile {
    let vel = current.velocity + env.gravity + env.wind;
    Projectile {
        position: current.position + vel,
        velocity: vel,
    }
}

fn plot(mut cvs: Canvas, width: f64, height: f64, path: Vec<Projectile>) -> Canvas {
    let red = colour(1.0, 0.0, 0.0);
    for p in path.iter() {
        let xf = p.position.x() / width;
        let yf = p.position.y() / height;
        let xi = asusize(xf * asf64(cvs.width - 1));
        let yi = asusize(yf * asf64(cvs.height - 1));
        let compensatedy = cvs.height - 1 - yi;
        cvs.set_colour_at(xi, compensatedy, red);
    }
    cvs
}

fn asf64(x: usize) -> f64 {
    let s = format!("{}", x);
    s.parse::<f64>().expect("should be parseable to f64")
}

fn asusize(x: f64) -> usize {
    let s = format!("{:.0}", x);
    if x < 0.0 {
        panic!("uh-oh, can't convert negative number to usize");
    }
    usize::from_str_radix(&s, 10).expect("should be parseable to usize")
}
