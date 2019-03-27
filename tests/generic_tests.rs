mod test_helpers;

#[cfg(test)]
pub mod generic_tests {
    use straal::*;

    use crate::test_helpers::*;

    #[test]
    fn vec2_gen() {
        let v1 = Vec2n::all(2);
        let v2 = Vec2h::all(2);
        let v3 = v1 * Vec2n::new(v2.x, -0.2);
        println!("{}", v3);
    }

    #[test]
    fn vec3_gen() {
        let v1 = Vec3n::all(2);
        let v2 = Vec3h::all(2);
        let v3 = v1 * Vec3n::new(v2.x, -0.2, 47.2);
        println!("{}", v3);
    }

    #[test]
    fn vec4_gen() {
        let v1 = Vec4n::all(2);
        let v2 = Vec4h::all(2);
        let v3 = v1 * Vec4n::new(v2.x, -0.2, 47.2, -652.0);
        println!("{}", v3);
    }


    #[test]
    fn ivec_gen_tests() {
        let v1 = IVec2n::all(3.0);
        let v2 = IVec2n::all(2.0);
        let v3 = v1 / v2;
        println!("{:?}", v3);
    }
}