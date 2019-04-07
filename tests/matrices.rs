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

#[test]
fn equality() {
    assert_eq!(
        matrix((1.0, 2.0, 3.0, 4.0), 
               (5.0, 6.0, 7.0, 8.0), 
               (9.0, 8.0, 7.0, 6.0), 
               (5.0, 4.0, 3.0, 2.0)),
        matrix((1.0, 2.0, 3.0, 4.0), 
               (5.0, 6.0, 7.0, 8.0), 
               (9.0, 8.0, 7.0, 6.0), 
               (5.0, 4.0, 3.0, 2.0))
        );
}

#[test]
fn not_equal() {
    assert_ne!(
        matrix((1.0, 2.0, 3.0, 4.0), 
               (5.0, 6.0, 7.0, 8.0), 
               (9.0, 8.0, 7.0, 6.0), 
               (5.0, 4.0, 3.0, 2.0)),
        matrix((2.0, 3.0, 4.0, 5.0), 
               (6.0, 7.0, 8.0, 9.0), 
               (8.0, 7.0, 6.0, 5.0), 
               (4.0, 3.0, 2.0, 1.0))
        );
}

#[test]
fn matrix_tuple_multiplication() {
    let a = matrix((1.0, 2.0, 3.0, 4.0),
                   (2.0, 4.0, 4.0, 2.0), 
                   (8.0, 6.0, 4.0, 1.0), 
                   (0.0, 0.0, 0.0, 1.0)); 
    let b = tuple(1.0, 2.0, 3.0, 1.0);

    assert_eq!(a.mult(b), tuple(18.0, 24.0, 33.0, 1.0));
}

#[test]
fn matrix_mult() {
    let a = matrix((1.0, 2.0, 3.0, 4.0),
                   (5.0, 6.0, 7.0, 8.0), 
                   (9.0, 8.0, 7.0, 6.0), 
                   (5.0, 4.0, 3.0, 2.0)); 
    let b = matrix((-2.0, 1.0, 2.0, 3.0),
                   ( 3.0, 2.0, 1.0,-1.0),
                   ( 4.0, 3.0, 6.0, 5.0),
                   ( 1.0, 2.0, 7.0, 8.0));

    assert_eq!(
        matrix((20.0, 22.0,  50.0,  48.0), 
               (44.0, 54.0, 114.0, 108.0), 
               (40.0, 58.0, 110.0, 102.0), 
               (16.0, 26.0,  46.0,  42.0)),
        a * b
        );
}

#[test]
fn mult_by_identity() {
    let a = matrix((0.0, 1.0, 2.0, 4.0),
                   (1.0, 2.0, 4.0, 8.0),
                   (2.0, 4.0, 8.0, 16.0),
                   (4.0, 8.0, 16.0, 32.0));
    let i = identity();

    assert_eq!(a.clone(), a * i);
}

#[test]
fn mult_tuple_by_identity() {
    let a = tuple(1.0, 2.0, 3.0, 4.0);
    let i = identity();

    assert_eq!(a.clone(), i.mult(a));
}

#[test]
fn transpose() {
    let a = matrix((0.0, 9.0, 3.0, 0.0),
                   (9.0, 8.0, 0.0, 8.0),
                   (1.0, 8.0, 5.0, 3.0),
                   (0.0, 0.0, 5.0, 8.0));
    assert_eq!(
        matrix((0.0, 9.0, 1.0, 0.0),
               (9.0, 8.0, 8.0, 0.0),
               (3.0, 0.0, 5.0, 5.0),
               (0.0, 8.0, 3.0, 8.0)),

        a.transpose())   
}

#[test]
fn determinant() {

    let a = 
        matrix2(( 1.0, 5.0),
                 (-3.0, 2.0));
    assert_eq!(17.0, a.det());

}

#[test]
fn submatrix_of_3x3() {
    let a = matrix3((1.0, 5.0, 0.0), 
                    (-3.0, 2.0, 7.0), 
                    (0.0, 6.0, -3.0));

    assert_eq!(a.submatrix(0, 2),
               matrix2((-3.0, 2.0), 
                       (0.0, 6.0)));
}

#[test]
fn submatrix_of_4x4() {
    let a = matrix((-6.0, 1.0, 1.0, 6.0), 
                   (-8.0, 5.0, 8.0, 6.0), 
                   (-1.0, 0.0, 8.0, 2.0), 
                   (-7.0, 1.0, -1.0, 1.0));

    assert_eq!(a.submatrix(2, 1),
               matrix3((-6.0, 1.0, 6.0), 
                       (-8.0, 8.0, 6.0),
                       (-7.0, -1.0, 1.0)));
}

#[test]
fn minor_of_3x3() {
    let a = matrix3((3.0, 5.0, 0.0), 
                    (2.0, -1.0, -7.0),
                    (6.0, -1.0, 5.0));
    let b = a.submatrix(1, 0);

    assert_eq!(25.0, b.det());
    assert_eq!(25.0, a.minor(1, 0));
}

#[test]
fn cofactors_of_3x3() {
    let a = matrix3((3.0, 5.0, 0.0), 
                    (2.0, -1.0, -7.0),
                    (6.0, -1.0, 5.0));
    assert_eq!(a.minor(0, 0), -12.0);
    assert_eq!(a.cofactor(0, 0), -12.0);
    assert_eq!(a.minor(1, 0), 25.0);
    assert_eq!(a.cofactor(1, 0), -25.0);
}

#[test]
fn determinant_of_3x3() {
    let a = matrix3((1.0, 2.0, 6.0), 
                    (-5.0, 8.0, -4.0), 
                    (2.0, 6.0, 4.0));
    assert_eq!(a.cofactor(0, 0), 56.0);
    assert_eq!(a.cofactor(0, 1), 12.0);
    assert_eq!(a.cofactor(0, 2), -46.0);
    assert_eq!(a.det(), -196.0);
}

#[test]
fn determinant_of_4x4() {
    let a = matrix((-2.0, -8.0,  3.0, 5.0), 
                   (-3.0,  1.0,  7.0, 3.0), 
                   ( 1.0,  2.0, -9.0, 6.0), 
                   (-6.0,  7.0,  7.0, -9.0)); 
    assert_eq!(a.cofactor(0, 0), 690.0);
    assert_eq!(a.cofactor(0, 1), 447.0);
    assert_eq!(a.cofactor(0, 2), 210.0);
    assert_eq!(a.cofactor(0, 3),  51.0);
    assert_eq!(a.det(), -4071.0);
}

#[test]
fn test_for_invertibility() {
    let a = matrix(( 6.0,  4.0,  4.0,  4.0), 
                   ( 5.0,  5.0,  7.0,  6.0), 
                   ( 4.0, -9.0,  3.0, -7.0), 
                   ( 9.0,  1.0,  7.0, -6.0)); 
    assert_eq!(-2120.0, a.det());

    let b = matrix((-4.0,  2.0, -2.0, -3.0), 
                   ( 9.0,  6.0,  2.0,  6.0), 
                   ( 0.0, -5.0,  1.0, -5.0), 
                   ( 0.0,  0.0,  0.0,  0.0)); 
    assert_eq!( 0.0, b.det());
}

#[test]
fn inverse() {
    let a = matrix((-5.0,  2.0,  6.0, -8.0), 
                   ( 1.0, -5.0,  1.0,  8.0), 
                   ( 7.0,  7.0, -6.0, -7.0), 
                   ( 1.0, -3.0,  7.0,  4.0)); 
    let b = a.inverse();

    assert_eq!(532.0, a.det());

    assert_eq!(
        matrix(( 0.21805,  0.45113,  0.24060, -0.04511), 
               (-0.80827, -1.45677, -0.44361,  0.52068), 
               (-0.07895, -0.22368, -0.05263,  0.19737), 
               (-0.52256, -0.81391, -0.30075,  0.30639)), b);


}
