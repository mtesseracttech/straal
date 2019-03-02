use straal::*;

//Test function to verify the correctness of the rolled out implementations of matrix rotations
pub fn angles_to_axes_zxy_unoptimized(angles: Vec3) -> Mat3 {
    const DEG_TO_RAD: f32 = std::f32::consts::PI / 180.0;
    let angles = angles * DEG_TO_RAD;
    let sx = angles.x.sin();
    let cx = angles.x.cos();
    let sy = angles.y.sin();
    let cy = angles.y.cos();
    let sz = angles.z.sin();
    let cz = angles.z.cos();

    let mx = Mat3::new(1.0, 0.0, 0.0,
                       0.0, cx, sx,
                       0.0, -sx, cx);

    let my = Mat3::new(cy, 0.0, -sy,
                       0.0, 1.0, 0.0,
                       sy, 0.0, cy);

    let mz = Mat3::new(cz, sz, 0.0,
                       -sz, cz, 0.0,
                       0.0, 0.0, 1.0);

    mz * mx * my
}

pub fn angles_to_quat_zxy_unoptimized(angles: Vec3) -> Quat {
    const DEG_TO_RAD: f32 = std::f32::consts::PI / 180.0;
    let angles = angles * DEG_TO_RAD / 2.0;
    let sx = angles.x.sin();
    let cx = angles.x.cos();
    let sy = angles.y.sin();
    let cy = angles.y.cos();
    let sz = angles.z.sin();
    let cz = angles.z.cos();

    let qx = Quat::new(cx, sx, 0.0, 0.0);
    let qy = Quat::new(cy, 0.0, sy, 0.0);
    let qz = Quat::new(cz, 0.0, 0.0, sz);

    qz * qx * qy
}

//Helper function to easily get quaternion products with variables (yes, it's as stupid as it looks)
pub fn var_quat_multiplication(lhs: [&str; 4], rhs: [&str; 4]) -> [String; 4] {
    [
        format!("{}*{}-{}*{}-{}*{}-{}*{}", lhs[0], rhs[0], lhs[1], rhs[1], lhs[2], rhs[2], lhs[3], rhs[3]),
        format!("{}*{}+{}*{}+{}*{}-{}*{}", lhs[0], rhs[1], lhs[1], rhs[0], lhs[2], rhs[3], lhs[3], rhs[2]),
        format!("{}*{}-{}*{}+{}*{}+{}*{}", lhs[0], rhs[2], lhs[1], rhs[3], lhs[2], rhs[0], lhs[3], rhs[1]),
        format!("{}*{}+{}*{}-{}*{}+{}*{}", lhs[0], rhs[3], lhs[1], rhs[2], lhs[2], rhs[1], lhs[3], rhs[0]),
    ]
}