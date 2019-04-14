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
            .mult(zero);
        hours.push(ith_point);
    }

    //for p in hours.iter() {
     //   println!("{:?}", p);
    //}
    let c = plot(canvas(200, 200), (-1.5, -1.5), (1.5, 1.5), hours);

    encode_ppm(&c, &mut stdout)
}

fn plot(mut cvs: Canvas, 
        bottom_left: (f64, f64), 
        top_right: (f64, f64), 
        points: Vec<Tuple4>) -> Canvas {

    let width = top_right.0 - bottom_left.0;
    let height = top_right.1 - bottom_left.1;

    let world_centroid = centroid(bottom_left, top_right);
    let transform = identity()
        .translate(-world_centroid.x(), 
                   -world_centroid.y(), 
                   -world_centroid.z())  // take world coords, and centre them around (0,0)
        .scale(1.0 / width, 1.0 / height, 1.0)  // squash the coordinates so they fit in 
                                                // bounds of box (-1/2, -1/2), (1/2, 1/2)
        .translate(0.5, 0.5, 0.0);       // move so coords are within box (0,0), (1,1)

    

    let red = colour(1.0, 0.0, 0.0);
    for p in points.iter() {
        let q = transform.mult(*p);
        let xi = asusize(q.x() * asf64(cvs.width - 1));
        let yi = asusize(q.y() * asf64(cvs.height - 1));
        let compensatedy = cvs.height - 1 - yi;
        cvs.set_colour_at(xi, compensatedy, red);
    }
    cvs
}

fn centroid((bx, by): (f64, f64), (tx, ty): (f64, f64)) -> Tuple4 {
    tuple((bx + tx) / 2.0, (by + ty) / 2.0, 0.0, 0.0)
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
