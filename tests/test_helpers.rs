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