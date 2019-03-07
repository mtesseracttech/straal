extern crate rand;

use std::time::{Duration, SystemTime};

use rand::Rng;

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

pub fn get_perspective_matrix(target_dims: &Vec2) -> Mat4 {
    let aspect_ratio = target_dims.y as f32 / target_dims.x as f32;
    let fov = std::f32::consts::PI / 3.0;
    let z_far = 1024.0;
    let z_near = 0.1;
    let f = 1.0 / (fov / 2.0).tan();

    Mat4::new(f * aspect_ratio, 0.0, 0.0, 0.0,
              0.0, f, 0.0, 0.0,
              0.0, 0.0, (z_far + z_near) / (z_far - z_near), -(2.0 * z_far * z_near) / (z_far - z_near),
              0.0, 0.0, 1.0, 0.0)
}


pub fn get_view_matrix(pos: &Vec3, dir: &Vec3, up: &Vec3) -> Mat4 {
    let fwd = dir.normalized();
    let rht = Vec3::cross(up, &fwd).normalized();
    let up = Vec3::cross(&fwd, &rht);
    let pos = Vec3::new(-Vec3::dot(pos, &rht), -Vec3::dot(pos, &up), -Vec3::dot(pos, &fwd));

    Mat4::new_from_vec4s(Vec4::from((rht, pos.x)),
                         Vec4::from((up, pos.y)),
                         Vec4::from((fwd, pos.z)),
                         Vec4::new(0.0, 0.0, 0.0, 1.0))
}

pub fn get_model_matrix(pos: &Vec3) -> Mat4 {
    Mat4::new(1.0, 0.0, 0.0, pos.x,
              0.0, 1.0, 0.0, pos.y,
              0.0, 0.0, 1.0, pos.z,
              0.0, 0.0, 0.0, 1.0)
}

pub fn get_time(timer: &SystemTime) -> f32 {
    match timer.elapsed() {
        Ok(elapsed) => ((elapsed.as_secs() * 1_000_000_000 + elapsed.subsec_nanos() as u64) as f64 / 1_000_000_000.0) as f32,
        Err(e) => {
            println!("Error: {:?}", e);
            0.0
        }
    }
}