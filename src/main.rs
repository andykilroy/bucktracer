use bucktracer::*;

fn main() {
    let stone = Projectile {
        position: point(0.0, 1.0, 0.0),
        velocity: vector(1.0, 1.0, 0.0)
    };

    let env = Environment {
        gravity: vector(0.0, -0.1, 0.0),
        wind: vector(0.0, 0.0, 0.0)
    };
    
    let path = trace_path(&env, &stone);

    for p in path.iter() {
        println!("{:?}", p);
    }
}

#[derive(Debug, Clone)]
struct Projectile {
    position: Tuple4, // point
    velocity: Tuple4  //vector
}

struct Environment {
    gravity: Tuple4,  //vector
    wind: Tuple4
}

fn trace_path(env: &Environment, start: &Projectile) -> Vec<Projectile> {
    let mut v: Vec<Projectile> = vec![];
    
    v.push(start.clone());
    let mut current: Projectile = start.clone();
    while current.position.y() > 0.0 {
        current = tick(env, &current);
        v.push(current.clone());
    }

    v
}

fn tick(env: &Environment, current: &Projectile) -> Projectile {
    let vel = current.velocity + env.gravity + env.wind;
    Projectile {
        position: current.position + vel,
        velocity: vel
    }
}


