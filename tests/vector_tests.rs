mod test_helpers;

#[cfg(test)]
pub mod vector_test {
    use straal::*;

    use crate::test_helpers::*;

    #[test]
    fn vector_instantiation_and_display_testing() {
        let v0 = Vec2::new(-2.0, 232.2);
        assert_eq!("(-2.00 232.20)", v0.to_string());

        let v0 = Vec3::new(987.123, -53.09, 1232.232);
        assert_eq!("(987.12 -53.09 1232.23)", v0.to_string());

        let v0 = Vec4::new(9345.23463, -95723.091232, -231242.232, 79854.3983);
        assert_eq!("(9345.23 -95723.09 -231242.23 79854.40)", v0.to_string());
    }
}
