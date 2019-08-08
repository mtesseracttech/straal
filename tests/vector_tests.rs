mod test_helpers;

#[cfg(test)]
pub mod vector_test {
    use straal::*;

    use crate::test_helpers::*;

    #[test]
    fn vector_instantiation_and_display_testing() {
        let v0 = Vec2n::new(-2.0, 232.2);
        assert_eq!("(-2.000 232.200)", v0.to_string());

        let v0 = Vec3n::new(987.123, -53.09, 1232.232);
        assert_eq!("(987.123 -53.090 1232.232)", v0.to_string());

        let v0 = Vec4n::new(9345.23463, -95723.091232, -231242.232, 79854.3983);
        assert_eq!("(9345.234 -95723.094 -231242.234 79854.398)", v0.to_string());


        let vec_vec = vec![Vec3n::new(1, 2, 3), Vec3n::new(4, 5, 6), Vec3n::new(7, 8, 9)];
        let total: Vec3n = vec_vec.iter().sum();
        assert_eq!(Vec3n::new(12, 15, 18), total);

        let i = Vec3n::new(1, -1, 0).normalized();
        let n = Vec3n::new(0, 1, 0);
        let r = Vec3n::refract(i, n, 0.66666);
        println!("{:?}", r);
    }
}
