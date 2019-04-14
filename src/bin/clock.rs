use bucktracer::*;
use std::io::Result;
use std::f64::consts::*;

fn main() -> Result<()> {
    let zero = point(0.0, 1.0, 0.0);
    let mut hours: Vec<Tuple4> = vec![];
    let mut stdout = std::io::stdout();

    for i in 0..12 {
        let ith_point = identity()
            .rotate_z(-(i as f64) * (PI / 6.0))
            .translate(2.0, 2.0, 0.0)
            .mult(zero);
        hours.push(ith_point);
    }

    //for p in hours.iter() {
     //   println!("{:?}", p);
    //}
    let c = plot(canvas(200, 200), 4.0, 4.0, hours);

    encode_ppm(&c, &mut stdout)
}

fn plot(mut cvs: Canvas, width: f64, height: f64, path: Vec<Tuple4>) -> Canvas {
    let red = colour(1.0, 0.0, 0.0);
    for p in path.iter() {
        let xf = p.x() / width;
        let yf = p.y() / height;
        let xi = asusize(xf * asf64(cvs.width - 1));
        let yi = asusize(yf * asf64(cvs.height - 1));
        let compensatedy = cvs.height - 1 - yi;
        cvs.set_colour_at(xi, compensatedy, red);
    }
    cvs
}

fn asf64(x: usize) -> f64 {
    let s = format!("{}", x);
    let v = s.parse::<f64>().expect("should be parseable to f64");
    v
}

fn asusize(x: f64) -> usize {
    let s = format!("{:.0}", x);
    if x < 0.0 {panic!("uh-oh, can't convert negative number to usize");}
    let v = usize::from_str_radix(&s, 10).expect("should be parseable to usize");
    v
}
