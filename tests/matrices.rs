use bucktracer::*;

#[test]
fn create_4x4_matrix() {

    let m = matrix(
        (1.0, 2.0, 3.0, 4.0),
        (5.5, 6.5, 7.5, 8.5),
        (9.0, 10.0, 11.0, 12.0),
        (13.5, 14.5, 15.5, 16.5)
        );


    assert_eq!(m[0][0], 1.0);
    assert_eq!(m[0][3], 4.0);
    assert_eq!(m[1][0], 5.5);
    assert_eq!(m[1][2], 7.5);
    assert_eq!(m[2][2], 11.0);
    assert_eq!(m[3][0], 13.5);
    assert_eq!(m[3][2], 15.5);
}
