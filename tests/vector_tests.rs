mod test_helpers;

#[cfg(test)]
pub mod vector_test {
    use straal::*;

    use crate::test_helpers::*;

    #[test]
    fn vec2_instantiation() {
        let v0 = Vec2::new(-2.0, 232.2);
        assert_eq!("(-2.00 232.20)", v0.to_string())
    }
}
