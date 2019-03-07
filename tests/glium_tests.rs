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
            position: Vec2,
            color: Vec3,
        }

        implement_vertex!(Vertex, position,color);

        let vertex1 = Vertex { position: Vec2::new(-0.5, -0.5), color: Vec3::new(1.0, 0.0, 0.0) };
        let vertex2 = Vertex { position: Vec2::new(0.0, 0.5), color: Vec3::new(0.0, 1.0, 0.0) };
        let vertex3 = Vertex { position: Vec2::new(0.5, -0.25), color: Vec3::new(0.0, 0.0, 1.0) };
        let shape = vec![vertex1, vertex2, vertex3];

        let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);


        let timer = SystemTime::now();
        let mut t_c = 0.0;
        let mut t_p = 0.0;
        let mut dt = 0.0;

        let program = Shader::load(&display, Shader::COLORED2D).unwrap();

        let mut closed = false;
        while !closed {
            t_p = t_c;
            t_c = get_time(&timer);
            dt = t_c - t_p;

            println!("{}", dt);


            let mut target = display.draw();
            target.clear_color(0.0, 0.0, 1.0, 1.0);
            target.draw(&vertex_buffer, &indices, &program, &glium::uniforms::EmptyUniforms,
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