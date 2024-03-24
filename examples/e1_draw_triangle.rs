use glium::{implement_vertex, Surface};

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}
implement_vertex!(Vertex, position);

fn main() {
    let event_loop = winit::event_loop::EventLoopBuilder::new()
        .build()
        .expect("event loop building");
    let (_window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .with_title("Draw Triangle")
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

    let vertex_shader_src_0 = r#"
        #version 140

        in vec2 position;

        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
        }
    "#;

    let vertex_shader_src_1 = r#"
        #version 140

        in vec2 position;

        void main() {
            gl_Position = vec4(position + 0.3, 0.0, 1.0);
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

    let mut frame = display.draw();
    frame.clear_color(0.0, 0.0, 0.0, 1.0);
    frame
        .draw(
            &v_buf,
            i_buf,
            &program_0,
            &glium::uniforms::EmptyUniforms,
            &Default::default(),
        )
        .unwrap();
    frame
        .draw(
            &v_buf,
            i_buf,
            &program_1,
            &glium::uniforms::EmptyUniforms,
            &Default::default(),
        )
        .unwrap();
    frame.finish().unwrap();

    let _ = event_loop.run(move |event, window_target| {
        if let winit::event::Event::WindowEvent { event, .. } = event {
            if event == winit::event::WindowEvent::CloseRequested {
                window_target.exit()
            }
        };
    });
}
