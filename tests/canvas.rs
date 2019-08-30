use bucktracer::*;


#[test]
fn create_a_colour() {
    let c = colour(-0.5, 0.4, 1.7);
    assert_eq!(-0.5, c.red());
    assert_eq!( 0.4, c.green());
    assert_eq!( 1.7, c.blue());
}

#[test]
fn multiply_colours() {
    let a: Tuple4 = colour(1.0, 0.2, 0.4).into();
    let b: Tuple4 = colour(0.9, 1.0, 0.1).into();
    assert_eq!(colour(0.9, 0.2, 0.04),
               RGB::from(a.mult_pairwise(b)));
}

#[test]
fn create_a_canvas() {
    let c = canvas(10, 20);
    assert_eq!(c.width, 10);
    assert_eq!(c.height, 20);

    let black = colour(0.0, 0.0, 0.0);
    for i in 0..10 {
        for j in 0..20 {
            assert_eq!(c.colour_at(i, j), black);
        }
    }
}

#[test]
fn colour_the_canvas() {
    let mut c = canvas(10, 20);
    let red = colour(1.0, 0.0, 0.0);
    let green = colour(0.0, 1.0, 0.0);
    let blue = colour(0.0, 0.0, 1.0);
    let white = colour(1.0, 1.0, 1.0);

    c.set_colour_at(0, 0, red);
    c.set_colour_at(9, 0, green);
    c.set_colour_at(0, 19, blue);
    c.set_colour_at(9, 19, white);

    assert_eq!(c.colour_at(0, 0), red);
    assert_eq!(c.colour_at(9, 0), green);
    assert_eq!(c.colour_at(0, 19), blue);
    assert_eq!(c.colour_at(9, 19), white);
}
