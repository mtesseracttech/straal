#[macro_use]
extern crate glium;

mod test_helpers;
mod shader_helpers;

#[cfg(test)]
pub mod glium_test {
    use glium::{glutin, Surface};

    use straal::*;

    use crate::shader_helpers::*;
    use crate::test_helpers::*;

    #[test]
    fn run_glium() {
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

        let program = Shader::load(&display, Shader::PHONG).unwrap();

        #[derive(Copy, Clone)]
        struct Vertex {
            position: Vec3,
            normal: Vec3,
            tex_coords: Vec2,
        }

        implement_vertex!(Vertex, position, normal, tex_coords);

        let shape = glium::vertex::VertexBuffer::new(&display, &[
            Vertex { position: Vec3::from([-1.0, 1.0, 0.0]), normal: Vec3::from([0.0, 0.0, -1.0]), tex_coords: Vec2::from([0.0, 1.0]) },
            Vertex { position: Vec3::from([1.0, 1.0, 0.0]), normal: Vec3::from([0.0, 0.0, -1.0]), tex_coords: Vec2::from([1.0, 1.0]) },
            Vertex { position: Vec3::from([-1.0, -1.0, 0.0]), normal: Vec3::from([0.0, 0.0, -1.0]), tex_coords: Vec2::from([0.0, 0.0]) },
            Vertex { position: Vec3::from([1.0, -1.0, 0.0]), normal: Vec3::from([0.0, 0.0, -1.0]), tex_coords: Vec2::from([1.0, 0.0]) },
        ]).unwrap();

        let shape = glium::vertex::VertexBuffer::new(&display, &[]);

        let mut model_pos = Vec3::new(0.0, 0.0, 0.0);

        let mut closed = false;
        while !closed {
            //Updating time variable
            t += 0.005;
            if t > 0.5 {
                t = -0.5;
            }

            let light_direction = Vec3::new(1.4, 0.4, -0.7);

            //Starting the drawing pass
            let mut target = display.draw();

            //Creating the matrices
            let model_matrix = get_model_matrix(&model_pos);
            let perspective_matrix = get_perspective_matrix(&Vec2::from(target.get_dimensions()));
            let view_matrix = get_view_matrix(&Vec3::new(0.5, 0.2, -3.0), &Vec3::new(-0.5, -0.2, 3.0), &Vec3::new(0.0, 1.0, 0.0));

            target.clear_color_and_depth((clear_color[0], clear_color[1], clear_color[2], 1.0), 1.0);

            target.draw(&shape,
                        glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip),
                        &program,
                        &uniform! {
                                model: model_matrix,
                                view: view_matrix,
                                perspective: perspective_matrix,
                                light_dir: light_direction,
                                diffuse: &diffuse_texture,
                                normal: &normal_texture,
                                },
                        &draw_parameters).unwrap();

            //Ending the drawing pass
            target.finish().unwrap();


            //mouse_down = false;
            mouse_delta = [0.0, 0.0];

            //Processing the glutin events
            events_loop.poll_events(|ev| {
                match ev {
                    glutin::Event::WindowEvent { event, .. } => match event {
                        glutin::WindowEvent::CloseRequested => closed = true,
                        glutin::WindowEvent::MouseInput { state, button, .. } => {
                            if button == glutin::MouseButton::Left {
                                mouse_down = state == glutin::ElementState::Pressed;
                            }
                        }
                        _ => (), //Don't do anything for other window events
                    },
                    glutin::Event::DeviceEvent { event, .. } => match event {
                        glutin::DeviceEvent::MouseMotion { delta } => {
                            mouse_delta[0] = delta.0 as f32;
                            mouse_delta[1] = delta.1 as f32;
                        }
                        _ => ()
                    }
                    _ => (), //Don't do anything for other events
                }
            });
        }

        fn get_model_matrix(pos: &Vec3) -> Mat4 {
            Mat4::new(1.0, 0.0, 0.0, pos.x,
                      0.0, 1.0, 0.0, pos.y,
                      0.0, 0.0, 1.0, pos.z,
                      0.0, 0.0, 0.0, 1.0)
        }
    }
}