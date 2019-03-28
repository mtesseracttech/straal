use std::time::{Duration, SystemTime};

use straal::*;

//Test function to verify the correctness of the rolled out implementations of matrix rotations
pub fn angles_to_axes_zxy_unoptimized<S: FloatType<S>>(angles: Vec3<S>) -> Mat3<S> {
    //const DEG_TO_RAD: f32 = std::f32::consts::PI / 180.0;
    //let deg_to_rad = num::cast(std::f32::consts::PI / 180.0).unwrap();
    let angles: Vec3<S> = angles * S::one().to_radians();//DEG_TO_RAD;// * num::cast(DEG_TO_RAD).unwrap();
    let sx = angles.x.sin();
    let cx = angles.x.cos();
    let sy = angles.y.sin();
    let cy = angles.y.cos();
    let sz = angles.z.sin();
    let cz = angles.z.cos();

    let mx = Mat3::<S>::new(S::one(), S::zero(), S::zero(),
                            S::zero(), cx, sx,
                            S::zero(), -sx, cx);

    let my = Mat3::<S>::new(cy, S::zero(), -sy,
                            S::zero(), S::one(), S::zero(),
                            sy, S::zero(), cy);

    let mz = Mat3::<S>::new(cz, sz, S::zero(),
                            -sz, cz, S::zero(),
                            S::zero(), S::zero(), S::one());

    mz * mx * my
}

pub fn angles_to_quat_zxy_unoptimized<S: FloatType<S>>(angles: Vec3<S>) -> Quat<S> {
    //const DEG_TO_RAD: f32 = std::f32::consts::PI / 180.0;
    let angles: Vec3<S> = angles * S::one().to_radians() / (S::one() + S::one());// * num::cast(DEG_TO_RAD / 2.0).unwrap();
    let sx = angles.x.sin();
    let cx = angles.x.cos();
    let sy = angles.y.sin();
    let cy = angles.y.cos();
    let sz = angles.z.sin();
    let cz = angles.z.cos();

    let qx = Quat::new(cx, sx, S::zero(), S::zero());
    let qy = Quat::new(cy, S::zero(), sy, S::zero());
    let qz = Quat::new(cz, S::zero(), S::zero(), sz);

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

pub fn get_perspective_matrix<S: FloatType<S>>(target_dims: &Vec2<S>) -> Mat4<S> {
    let aspect_ratio = target_dims.y / target_dims.x;
    let fov: S = num::cast(std::f32::consts::PI / 3.0).unwrap();
    let z_far = num::cast(1024.0).unwrap();
    let z_near = num::cast(0.1).unwrap();
    let f: S = S::one() / (fov / (num::cast(2.0).unwrap())).tan();

    Mat4::new(f * aspect_ratio, S::zero(), S::zero(), S::zero(),
              S::zero(), f, S::zero(), S::zero(),
              S::zero(), S::zero(), (z_far + z_near) / (z_far - z_near), -((S::one() + S::one()) * z_far * z_near) / (z_far - z_near),
              S::zero(), S::zero(), S::one(), S::zero())
}


pub fn get_view_matrix<S: FloatType<S>>(pos: &Vec3<S>, dir: &Vec3<S>, up: &Vec3<S>) -> Mat4<S> {
    let fwd = dir.normalized();
    let rht = Vec3::cross(*up, fwd).normalized();
    let up = Vec3::cross(fwd, rht);
    let pos = Vec3::new(-Vec3::dot(*pos, rht), -Vec3::dot(*pos, up), -Vec3::dot(*pos, fwd));

    Mat4::new_from_vec4s(Vec4::from((rht, pos.x)),
                         Vec4::from((up, pos.y)),
                         Vec4::from((fwd, pos.z)),
                         Vec4::new(0.0, 0.0, 0.0, 1.0))
}

pub fn get_model_matrix<S: FloatType<S>>(pos: &Vec3<S>) -> Mat4<S> {
    Mat4::new(S::one(), S::zero(), S::zero(), pos.x,
              S::zero(), S::one(), S::zero(), pos.y,
              S::zero(), S::zero(), S::one(), pos.z,
              S::zero(), S::zero(), S::zero(), S::one())
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

#[derive(Copy, Clone)]
pub struct VertexPosColor {
    position: Vec2n,
    color: Vec3n,
}

implement_vertex!(VertexPosColor, position,color);

pub fn get_triangle() -> Vec<VertexPosColor> {
    let vertex1 = VertexPosColor { position: Vec2::new(-0.5, -0.5), color: Vec3::new(1.0, 0.0, 0.0) };
    let vertex2 = VertexPosColor { position: Vec2::new(0.0, 0.5), color: Vec3::new(0.0, 1.0, 0.0) };
    let vertex3 = VertexPosColor { position: Vec2::new(0.5, -0.5), color: Vec3::new(0.0, 0.0, 1.0) };
    vec![vertex1, vertex2, vertex3]
}

pub fn def_draw_params() -> glium::DrawParameters<'static> {
    glium::DrawParameters {
        depth: glium::Depth {
            test: glium::draw_parameters::DepthTest::IfLess,
            write: true,
            ..Default::default()
        },
        ..Default::default()
    }
}