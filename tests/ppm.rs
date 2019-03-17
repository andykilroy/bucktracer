use bucktracer::*;
use std::string::*;

#[test]
fn print_ppm_header_for_empty_canvas() {
    let cvs = canvas(0, 0);
    let mut bytes: Vec<u8> = vec![];
    encode_ppm(&cvs, &mut bytes);

    let mut s = String::from_utf8_lossy(&bytes);
    assert_eq!(s,
r##"P3
0 0
255
"##
        );
}

#[test]
fn print_ppm_output_without_clamping() {
    let mut cvs = canvas(5, 3);
    cvs.set_colour_at(0, 0, colour(1.0, 0.0, 0.0));
    cvs.set_colour_at(2, 1, colour(0.0, 0.5, 0.0));
    cvs.set_colour_at(4, 2, colour(0.0, 0.0, 1.0));
    let mut bytes: Vec<u8> = vec![];
    encode_ppm(&cvs, &mut bytes);

    let mut s = String::from_utf8_lossy(&bytes);
    assert_eq!(s,
r##"P3
5 3
255
255 0 0 0 0 0 0 0 0 0 0 0 0 0 0 
0 0 0 0 0 0 0 128 0 0 0 0 0 0 0 
0 0 0 0 0 0 0 0 0 0 0 0 0 0 255 
"##
        );
}

#[test]
fn print_ppm_output_with_clamping() {
    let mut cvs = canvas(5, 3);
    cvs.set_colour_at(0, 0, colour(1.5, 0.0, 0.0));
    cvs.set_colour_at(2, 1, colour(0.0, 0.5, 0.0));
    cvs.set_colour_at(4, 2, colour(-0.5, 0.0, 1.0));
    let mut bytes: Vec<u8> = vec![];
    encode_ppm(&cvs, &mut bytes);

    let mut s = String::from_utf8_lossy(&bytes);
    assert_eq!(s,
r##"P3
5 3
255
255 0 0 0 0 0 0 0 0 0 0 0 0 0 0 
0 0 0 0 0 0 0 128 0 0 0 0 0 0 0 
0 0 0 0 0 0 0 0 0 0 0 0 0 0 255 
"##
        );
}

#[test]
fn print_ppm_output_max_70_chars_per_line() {

    let mut cvs = canvas(10, 2);

    for col in 0..(cvs.width) {
        for row in 0..(cvs.height) {
            cvs.set_colour_at(col, row, colour(1.0, 0.8, 0.6));
        }
    }
    let mut bytes: Vec<u8> = vec![];
    encode_ppm(&cvs, &mut bytes);

    let mut s = String::from_utf8_lossy(&bytes);
    assert_eq!(s,
r##"P3
10 2
255
255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 
255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 
255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 
255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 
"##
        );
}
