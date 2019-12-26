use std::io;
use crate::Canvas;
use png;

/// Encode the canvas pixel information to the writer, according to the
/// png format.
pub fn encode(c: &Canvas, w: &mut dyn io::Write) -> io::Result<()> {
    let data = to_rgb_array(c);

    let mut encoder = png::Encoder::new(w, c.width as u32, c.height as u32);
    encoder.set_color(png::ColorType::RGB);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header()?;
    writer.write_image_data(&data)?;
    Ok(())
}

fn to_rgb_array(canv: &Canvas) -> Vec<u8> {
    let mut vec: Vec<u8> = Vec::with_capacity(canv.pixels.len() * 3);
    for rgb in canv.pixels.iter() {
        vec.push(to_u8(rgb.red() * 256.0));
        vec.push(to_u8(rgb.green() * 256.0));
        vec.push(to_u8(rgb.blue() * 256.0));
    }
    vec
}

fn clamp(f: f64) -> f64 {
    if f <= 0.0 { return 0.0 }
    else if f >= 256.0 { return 256.0; }
    else { f }
}

fn to_u8(orig: f64) -> u8 {
    let clamped = clamp(orig);
    if clamped >= 256.0 {return 255;}
    if clamped <= 0.0 {return 0;}

    let f = clamped.trunc().to_bits();
    if f == 0 {return 0u8};

    // TODO is this all platform specific?
    let sgn = 0x8000_0000_0000_0000u64 & f;
    let bias: i64 = 1023;
    let biased_exp = (0x7ff0_0000_0000_0000u64 & f) >> 52;
    let exp: i64 = (biased_exp as i64) - bias;
    let mantissa = 0x000f_ffff_ffff_ffffu64 & f;
    let m = (mantissa >> 45) as u8;
    assert!(sgn == 0);

    let mut x: u8 = 1;
    if exp > 0 && exp < 8 {
        x = (x << exp) | (m >> (7 - exp));
    }
    x
}



#[cfg(test)]
mod pngtest {
    use super::*;
    use crate::*;

    #[allow(non_snake_case)]
    #[test]
    fn empty_canvas_to_empty_rgb_array() {
        let canvs = canvas(0, 0);
        let arr = to_rgb_array(&canvs);
        assert_eq!(arr.len(), 0);
    }

    #[allow(non_snake_case)]
    #[test]
    fn nonempty_canvas_to_nonempty_rgb_array() {
        let mut canvs = canvas(2, 3);
        canvs.set_colour_at(0, 0, colour(1.0, 0.0, 0.0));
        canvs.set_colour_at(1, 0, colour(0.0, 1.0, 0.0));
        canvs.set_colour_at(0, 1, colour(0.0, 0.0, 1.0));
        canvs.set_colour_at(1, 1, colour(0.0, 0.0, 0.5));
        canvs.set_colour_at(0, 2, colour(0.0, 0.5, 0.0));
        canvs.set_colour_at(1, 2, colour(0.5, 0.0, 0.0));
        let arr = to_rgb_array(&canvs);
        assert_eq!(arr, [
            255, 0, 0,
            0, 255, 0,
            0, 0, 255,
            0, 0, 128,
            0, 128, 0,
            128, 0, 0,
        ])
    }
}

#[allow(non_snake_case)]
#[test]
fn conversion_to_u8() {
    assert_eq!(to_u8(0.0), 0u8);
    assert_eq!(to_u8(0.98), 0u8);
    assert_eq!(to_u8(1.0), 1u8);
    assert_eq!(to_u8(1.98), 1u8);
    assert_eq!(to_u8(2.0), 2u8);

    assert_eq!(to_u8(8.0), 8u8);
    assert_eq!(to_u8(12.0), 12u8);
    assert_eq!(to_u8(64.0), 64u8);
    assert_eq!(to_u8(128.0), 128u8);
    assert_eq!(to_u8(254.0), 254u8);
    assert_eq!(to_u8(255.0), 255u8);
    assert_eq!(to_u8(256.0), 255u8);
    assert_eq!(to_u8(257.0), 255u8);

    assert_eq!(to_u8(-1.0), 0u8);
    assert_eq!(to_u8(-2.0), 0u8);
}
