mod test_helpers;

#[cfg(test)]
pub mod matrix_test {
    use straal::*;

    use crate::test_helpers::*;

    #[test]
    fn mat3_determinant() {
        let m0 = Mat2::from([
            [10.0, 4.0],
            [8.0, 2.0],
        ]);
        let det = m0.determinant();
        assert_eq!(-12.0, det);

        let m0 = Mat3::from([
            [2.0, -3.0, 1.0],
            [2.0, 0.0, -1.0],
            [1.0, 4.0, 5.0]
        ]);
        let det = m0.determinant();
        assert_eq!(49.0, det);

        let m0 = Mat4::from([
            [10.0, 4.0, 6.0, 9.0],
            [8.0, 2.0, 4.0, 7.0],
            [11.0, 5.0, 7.0, 10.0],
            [3.0, 9.0, 11.0, 2.0],
        ]);
        let det = m0.determinant();
        assert_eq!(0.0, det);
    }

    #[test]
    fn matrix_products_and_identities() {
        let m0 = Mat2::from([
            [2.0, 4.0],
            [5.0, 7.0]
        ]);
        let m1 = Mat2::identity();
        let m2 = m0 * m1;
        assert_eq!(m2, m0);

        let m0 = Mat3::from([
            [10.0, 4.0, 6.0],
            [8.0, 2.0, 4.0],
            [11.0, 5.0, 7.0],
        ]);
        let m1 = Mat3::identity();
        let m2 = m0 * m1;
        assert_eq!(m0, m2);


        let m0 = Mat4::from([
            [10.0, 4.0, 6.0, 9.0],
            [8.0, 2.0, 4.0, 7.0],
            [11.0, 5.0, 7.0, 10.0],
            [3.0, 9.0, 11.0, 2.0],
        ]);
        let m1 = Mat4::identity();
        let m2 = m0 * m1;
        assert_eq!(m2, m0);
    }

    #[test]
    fn matrix_inverses() {
        let base = Mat2n::new(4.0, 7.0,
                              2.0, 6.0);
        let inv = base.inverse();
        let prod = base * inv;
        let det = base.determinant();
        assert_eq!(Mat2::identity(), prod);

        let base = Mat3n::new(2.0, -3.0, 1.0,
                              2.0, 0.0, -1.0,
                              1.0, 4.0, 5.0);
        let inv = base.inverse();
        let prod = base * inv;
        let det = base.determinant();

        let base = Mat4n::new(2.0, -3.0, 1.0, -3.45,
                              2.0, 0.0, -1.0, 232.34267,
                              1.0, 4.0, 5.0, 7.223,
                              32.0, -0.453, 23.23, 1.0);
        let inv = base.inverse();
        let prod = base * inv;
        let det = base.determinant();
        println!("Base:\n{}\n Determinant: {}\nInverse:\n{} \nProduct: \n{}", base, det, inv, prod);
        assert_eq!(Mat4::identity(), prod);
    }

    #[test]
    fn rotation_matrices() {
        let m0 = Mat3n::get_rotation_mat_euler_obj_upr_deg(89.0, 89.0, 89.0);
        let m1 = angles_to_axes_zxy_unoptimized(Vec3n::new(89.0, 89.0, 89.0));
        assert_eq!(m0, m1);
    }
}
