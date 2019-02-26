mod test_helpers;

#[cfg(test)]
pub mod matrix_tests {
    use straal::*;

    #[test]
    fn quaternion_inverse() {
        let q0 = Quat::new(20.0, 232.1, -858.34, 6429.0);
        let q1 = q0.inverse();
        assert_eq!(Quat::identity(), q0 * q1);
    }
}