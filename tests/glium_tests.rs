#[macro_use]
extern crate glium;

mod test_helpers;
mod shader_helpers;

#[cfg(test)]
pub mod glium_test {
    use std::time::{Duration, SystemTime};

    use glium::{glutin, Surface};

    use straal::*;

    use crate::shader_helpers::*;
    use crate::test_helpers::*;

    #[test]
    fn ogl_triangle() {
        let mut events_loop = glutin::EventsLoop::new();
        let window = glutin::WindowBuilder::new();
        let context = glutin::ContextBuilder::new().with_depth_buffer(24);
        let display = glium::Display::new(window, context, &events_loop).unwrap();

        let draw_parameters = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            ..Default::default()
        };

        #[derive(Copy, Clone)]
        struct Vertex {
            position: Vec2n,
            color: Vec3n,
        }

        implement_vertex!(Vertex, position,color);

        let vertex1 = Vertex { position: Vec2::new(-0.5, -0.5), color: Vec3::new(1.0, 0.0, 0.0) };
        let vertex2 = Vertex { position: Vec2::new(0.0, 0.5), color: Vec3::new(0.0, 1.0, 0.0) };
        let vertex3 = Vertex { position: Vec2::new(0.5, -0.5), color: Vec3::new(0.0, 0.0, 1.0) };
        let shape = vec![vertex1, vertex2, vertex3];

        let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

        let program = Shader::load_glium_shader(&display, Shader::COLORED2D).unwrap();

        let mut model_matrix = Mat4n::identity();

        let mut closed = false;
        while !closed {
            let mut target = display.draw();
            target.clear_color(0.0, 0.0, 0.0, 0.0);
            target.draw(&vertex_buffer, &indices, &program, &uniform! {model : model_matrix},
                        &Default::default()).unwrap();
            target.finish().unwrap();

            //Processing the glutin events
            events_loop.poll_events(|ev| {
                match ev {
                    glutin::Event::WindowEvent { event, .. } => match event {
                        glutin::WindowEvent::CloseRequested => closed = true,
                        _ => (), //Don't do anything for other window events
                    }
                    _ => (), //Don't do anything for other events
                }
            });
        }
    }

    #[test]
    fn rotate_triangle() {
        let mut events_loop = glutin::EventsLoop::new();
        let window = glutin::WindowBuilder::new();
        let context = glutin::ContextBuilder::new().with_depth_buffer(24);
        let display = glium::Display::new(window, context, &events_loop).unwrap();

        let draw_parameters = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            ..Default::default()
        };

        #[derive(Copy, Clone)]
        struct Vertex {
            position: Vec2n,
            color: Vec3n,
        }

        implement_vertex!(Vertex, position,color);

        let vertex1 = Vertex { position: Vec2::new(-0.5, -0.5), color: Vec3::new(1.0, 0.0, 0.0) };
        let vertex2 = Vertex { position: Vec2::new(0.0, 0.5), color: Vec3::new(0.0, 1.0, 0.0) };
        let vertex3 = Vertex { position: Vec2::new(0.5, -0.5), color: Vec3::new(0.0, 0.0, 1.0) };
        let shape = vec![vertex1, vertex2, vertex3];

        let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);


        let timer = SystemTime::now();
        let mut time_current = 0.0;
        let mut time_previous = 0.0;
        let mut delta_time = 0.0;

        let program = Shader::load_glium_shader(&display, Shader::COLORED2D).unwrap();

        let mut model_matrix = get_model_matrix(&Vec3::new(0.0, 0.0, 0.0));

        let mut frames = 0;

        let mut closed = false;
        while !closed {
            frames += 1;
            time_previous = time_current;
            time_current = get_time(&timer);
            delta_time = time_current - time_previous;

            if frames == 100 {
                frames = 0;
                let fps = 1.0 / delta_time;
                println!("fps: {}", fps);
            }

            let mut target = display.draw();

            model_matrix.rotate_around_axis_rad(Vec3::up(), delta_time * 2.3);
            model_matrix.rotate_around_axis_rad(Vec3::from(model_matrix.r0).normalized(), delta_time * 2.9);

            //model_matrix.rotate_around(Vec3::right(), delta_time * 2.3);
            //model_matrix.rotate_around(Vec3::up(), delta_time * 2.9);
            //model_matrix.rotate_around(-Vec3::forward(), delta_time * 3.1);

            target.clear_color(0.0, 0.0, 0.0, 0.0);
            target.draw(&vertex_buffer, &indices, &program, &uniform! {model : model_matrix},
                        &Default::default()).unwrap();
            target.finish().unwrap();

            //Processing the glutin events
            events_loop.poll_events(|ev| {
                match ev {
                    glutin::Event::WindowEvent { event, .. } => match event {
                        glutin::WindowEvent::CloseRequested => closed = true,
                        _ => (), //Don't do anything for other window events
                    }
                    _ => (), //Don't do anything for other events
                }
            });
        }
    }

    #[test]
    fn rotate_triangle_quat() {
        let mut events_loop = glutin::EventsLoop::new();
        let window = glutin::WindowBuilder::new();
        let context = glutin::ContextBuilder::new().with_depth_buffer(24);
        let display = glium::Display::new(window, context, &events_loop).unwrap();

        let vertex_buffer = glium::VertexBuffer::new(&display, &get_triangle()).unwrap();
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

        let timer = SystemTime::now();
        let mut time_current = 0.0;
        let mut time_previous = 0.0;
        let mut delta_time = 0.0;

        let program = Shader::load_glium_shader(&display, Shader::COLORED2D).unwrap();

        let mut quat = Quatn::identity();

        //quat.rotate_around(Vec3::up(), 0.2 * 2.3);
        //let x_axis = Mat3::from(quat).r0.normalized();
        //let x_axis = (quat * Vec3::right()).normalized();
        //quat.rotate_around(x_axis, 0.2 * 2.9);
        //let x_axis2 = Mat3::from(quat).r0.normalized();
        //let x_axis2 = (quat * Vec3::right()).normalized();
        //println!("x axis 1: {:?}, x axis 2: {:?}", x_axis, x_axis2);

        let mut closed = false;
        while !closed {
            time_previous = time_current;
            time_current = get_time(&timer);
            delta_time = time_current - time_previous;

            let mut target = display.draw();

            //quat.rotate_around(Vec3::up(), delta_time * 2.3);
            //let x_axis = (quat * Vec3::right()).normalized();
            //quat.rotate_around(Vec3::right(), delta_time * 2.9);

            let model_matrix = get_model_matrix(&Vec3::new(0.0, 0.0, 0.0)) * Mat4::from(quat);

            target.clear_color(0.0, 0.0, 0.0, 0.0);
            target.draw(&vertex_buffer, &indices, &program, &uniform! {model : model_matrix},
                        &Default::default()).unwrap();
            target.finish().unwrap();

            //Processing the glutin events
            events_loop.poll_events(|ev| {
                match ev {
                    glutin::Event::WindowEvent { event, .. } => match event {
                        glutin::WindowEvent::CloseRequested => closed = true,
                        _ => (), //Don't do anything for other window events
                    }
                    _ => (), //Don't do anything for other events
                }
            });
        }
    }

    #[test]
    fn rotate_triangle_rot_mat() {
        let mut events_loop = glutin::EventsLoop::new();
        let window = glutin::WindowBuilder::new();
        let context = glutin::ContextBuilder::new().with_depth_buffer(24);
        let display = glium::Display::new(window, context, &events_loop).unwrap();

        let vertex_buffer = glium::VertexBuffer::new(&display, &get_triangle()).unwrap();
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

        let timer = SystemTime::now();
        let mut time_current = 0.0;
        let mut time_previous = 0.0;
        let mut delta_time = 0.0;

        let program = Shader::load_glium_shader(&display, Shader::COLORED2D).unwrap();

        let mut rotation_matrix = Mat3::identity();

        rotation_matrix.rotate_around_axis_rad(Vec3::up(), 0.2 * 2.3);
        let x_axis = rotation_matrix.r0.normalized();
        rotation_matrix.rotate_around_axis_rad(x_axis, 0.2 * 2.9);
        let x_axis2 = rotation_matrix.r0.normalized();
        println!("x axis 1: {:?}, x axis 2: {:?}", x_axis, x_axis2);

        let mut closed = false;
        while !closed {
            time_previous = time_current;
            time_current = get_time(&timer);
            delta_time = time_current - time_previous;

            let mut target = display.draw();

            rotation_matrix.rotate_around_axis_rad(Vec3::up(), delta_time * 2.3);
            rotation_matrix.rotate_around_axis_rad(Vec3::right(), delta_time * 2.9);

            let mut model_matrix = get_model_matrix(&Vec3::new(0.0, 0.0, 0.0)) * Mat4::from(rotation_matrix);

            target.clear_color(0.0, 0.0, 0.0, 0.0);
            target.draw(&vertex_buffer, &indices, &program, &uniform! {model : model_matrix},
                        &Default::default()).unwrap();
            target.finish().unwrap();

            //Processing the glutin events
            events_loop.poll_events(|ev| {
                match ev {
                    glutin::Event::WindowEvent { event, .. } => match event {
                        glutin::WindowEvent::CloseRequested => closed = true,
                        _ => (), //Don't do anything for other window events
                    }
                    _ => (), //Don't do anything for other events
                }
            });
        }
    }
}