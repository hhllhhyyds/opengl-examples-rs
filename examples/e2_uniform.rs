use glium::{implement_vertex, uniform, Surface};

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}
implement_vertex!(Vertex, position);

fn main() {
    let event_loop = winit::event_loop::EventLoopBuilder::new()
        .build()
        .expect("event loop building");
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .with_title("Triangle Movement By Change Uniform")
        .with_inner_size(1200, 800)
        .build(&event_loop);

    let vertex1 = Vertex {
        position: [-0.5, -0.5],
    };
    let vertex2 = Vertex {
        position: [0.0, 0.5],
    };
    let vertex3 = Vertex {
        position: [0.5, -0.25],
    };
    let triangle_vertices = vec![vertex1, vertex2, vertex3];

    let v_buf = glium::VertexBuffer::new(&display, &triangle_vertices).unwrap();
    let i_buf = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader_src_1 = r#"
        #version 140

        in vec2 position;

        uniform mat4 matrix;

        void main() {
            gl_Position = matrix * vec4(position, 0.0, 1.0);
        }
    "#;

    let vertex_shader_src_0 = r#"
        #version 140

        in vec2 position;

        uniform float x;
    
        void main() {
            vec2 pos = position;
            pos.x += x;
            gl_Position = vec4(pos, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140

        out vec4 color;

        void main() {
            color = vec4(1.0, 0.0, 0.0, 1.0);
        }
    "#;

    let program_0 =
        glium::Program::from_source(&display, vertex_shader_src_0, fragment_shader_src, None)
            .unwrap();

    let program_1 =
        glium::Program::from_source(&display, vertex_shader_src_1, fragment_shader_src, None)
            .unwrap();

    let mut t = 0_f32;

    let _ = event_loop.run(move |event, window_target| {
        match event {
            winit::event::Event::WindowEvent { event, .. } => match event {
                winit::event::WindowEvent::CloseRequested => window_target.exit(),
                winit::event::WindowEvent::Resized(window_size) => {
                    display.resize(window_size.into());
                }
                winit::event::WindowEvent::RedrawRequested => {
                    // We update `t`
                    t += 0.02;
                    // We use the sine of t as an offset, this way we get a nice smooth animation
                    let x = t.sin() * 0.5;

                    let mut frame = display.draw();
                    frame.clear_color(0.0, 0.0, 0.0, 1.0);
                    frame
                        .draw(
                            &v_buf,
                            i_buf,
                            &program_0,
                            &uniform! { x: x },
                            &Default::default(),
                        )
                        .unwrap();
                    frame
                        .draw(
                            &v_buf,
                            i_buf,
                            &program_1,
                            &uniform! {
                                matrix: [
                                    [1.0, 0.0, 0.0, 0.0],
                                    [x, 1.0, 0.0, 0.0],
                                    [0.0, 0.0, 1.0, 0.0],
                                    [0.0, 0.0, 0.0, 1.0f32],
                                ]
                            },
                            &Default::default(),
                        )
                        .unwrap();
                    frame.finish().unwrap();
                }
                _ => (),
            },
            winit::event::Event::AboutToWait => {
                window.request_redraw();
            }
            _ => (),
        };
    });
}
