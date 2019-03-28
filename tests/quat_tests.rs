mod test_helpers;

#[cfg(test)]
pub mod quat_test {
    use rand::Rng;
    use straal::*;

    use crate::test_helpers::*;

    #[test]
    fn quaternion_inverse() {
        let q0 = Quat::new(20.0, 232.0, -858.0, 6429.0);
        let q1 = q0.inverse();
        //Asserting q0 == q1 is not reliable (floating point error problems)
        assert_eq!(1.0, Quat::dot(&Quat::identity(), &(q0 * q1)));
    }

    #[test]
    fn quat_to_mat() {
        let m0 = Mat3::from(Quat::new(0.0, 1.0, 0.0, 0.0)); //rotation around x
        let m1 = Mat3::new_from_vec3s(Vec3::right(), Vec3::up(), Vec3::forward());
        assert_eq!(Mat3::new_from_vec3s(Vec3::right(), -Vec3::up(), -Vec3::forward()), m0 * m1)
    }

    #[test]
    fn quat_rotation_order_verification() {
        let q0 = angles_to_quat_zxy_unoptimized(Vec3::new(45.0, 45.0, 45.0));
        let q1 = Quat::from_euler_deg_zxy(Vec3::new(45.0, 45.0, 45.0));
        assert_eq!(q0, q1);
    }


    #[test]
    fn quat_euler() {
        let input = Vec3::new(75.2, -123.0, 62.0);
        let output = Quat::from_euler_deg_zxy(input).to_euler_deg_zxy();
        let delta = input - output;
        assert!(delta.length().abs() < 0.0001); //tested this way, because floating point error is hell

        //Should give the singularity situation
        let input = Vec3::new(90.0, 0.0, -60.0);
        let output = Quat::from_euler_deg_zxy(input).to_euler_deg_zxy();
        let delta = input - output;
        assert!(delta.length().abs() < 0.0001); //tested this way, because floating point error is hell
    }
}