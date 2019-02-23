#![allow(dead_code)]

pub use ivec2::*;
pub use ivec3::*;
pub use ivec4::*;
pub use mat2::*;
pub use mat3::*;
pub use mat4::*;
pub use quat::*;
pub use vec2::*;
pub use vec3::*;
pub use vec4::*;

pub mod ivec2;
pub mod ivec3;
pub mod ivec4;
pub mod mat2;
pub mod mat3;
pub mod mat4;
pub mod quat;
pub mod vec2;
pub mod vec3;
pub mod vec4;

pub type Scalar = f32;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec2_instantiation() {
        let v0 = Vec2::new(-2.0, 232.2);
        assert_eq!("(-2.00 232.20)", v0.to_string())
    }

    #[test]
    fn mat2_determinant() {
        let m0 = Mat2::from([
            [10.0, 4.0],
            [8.0, 2.0],
        ]);
        let det = m0.determinant();
        assert_eq!(-12.0, det);
    }

    #[test]
    fn mat3_determinant() {
        let m0 = Mat3::from([
            [10.0, 4.0, 6.0],
            [8.0, 2.0, 4.0],
            [11.0, 5.0, 7.0]
        ]);
        let det = m0.determinant();
        assert_eq!(0.0, det);
    }

    #[test]
    fn mat4_determinant() {
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
        let m0 = Mat2::from([[2.0, 4.0], [5.0, 7.0]]);
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
}