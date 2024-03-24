use glium::{uniform, Surface};

mod model;

fn main() {
    let event_loop = winit::event_loop::EventLoopBuilder::new()
        .build()
        .expect("event loop building");
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .with_title("Teapot")
        .build(&event_loop);

    let positions = glium::VertexBuffer::new(&display, &model::VERTICES).unwrap();
    let normals = glium::VertexBuffer::new(&display, &model::NORMALS).unwrap();
    let indices = glium::IndexBuffer::new(
        &display,
        glium::index::PrimitiveType::TrianglesList,
        &model::INDICES,
    )
    .unwrap();

    let vertex_shader_src = r#"
        #version 150      

        in vec3 position;
        in vec3 normal;
        
        out vec3 v_normal;      
        
        uniform mat4 matrix;
        uniform mat4 perspective; 
        
        void main() {
            v_normal = transpose(inverse(mat3(matrix))) * normal;      
            gl_Position = perspective * matrix * vec4(position, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140

        in vec3 v_normal;
        out vec4 color;
        uniform vec3 u_light;
        
        void main() {
            float brightness = dot(normalize(v_normal), normalize(u_light));
            vec3 dark_color = vec3(0.6, 0.0, 0.0);
            vec3 regular_color = vec3(1.0, 0.0, 0.0);
            color = vec4(mix(dark_color, regular_color, brightness), 1.0);
        }
    "#;

    let program =
        glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
            .unwrap();
    let params = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::draw_parameters::DepthTest::IfLess,
            write: true,
            ..Default::default()
        },
        ..Default::default()
    };

    let perspective = {
        let (width, height) = (1.0, 0.6);
        let aspect_ratio = height as f32 / width as f32;

        let fov: f32 = 3.141592 / 3.0;
        let zfar = 1024.0;
        let znear = 0.1;

        let f = 1.0 / (fov / 2.0).tan();

        [
            [f * aspect_ratio, 0.0, 0.0, 0.0],
            [0.0, f, 0.0, 0.0],
            [0.0, 0.0, (zfar + znear) / (zfar - znear), 1.0],
            [0.0, 0.0, -(2.0 * zfar * znear) / (zfar - znear), 0.0],
        ]
    };

    event_loop
        .run(move |ev, window_target| {
            match ev {
                winit::event::Event::WindowEvent { event, .. } => match event {
                    winit::event::WindowEvent::CloseRequested => {
                        window_target.exit();
                    }
                    // We now need to render everyting in response to a RedrawRequested event due to the animation
                    winit::event::WindowEvent::RedrawRequested => {
                        let mut target = display.draw();

                        target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);   
                        let matrix = [
                            [0.01, 0.0, 0.0, 0.0],
                            [0.0, 0.01, 0.0, 0.0],
                            [0.0, 0.0, 0.01, 0.0],
                            [0.0, 0.0, 1.5, 1.0f32]
                        ];
                        // the direction of the light
                        let light = [-1.0, 0.4, 0.9f32];

                        target
                            .draw(
                                (&positions, &normals),
                                &indices,
                                &program,
                                &uniform! { matrix: matrix, perspective: perspective, u_light: light },
                                &params,
                            )
                            .unwrap();
                        target.finish().unwrap();
                    }
                    // Because glium doesn't know about windows we need to resize the display
                    // when the window's size has changed.
                    winit::event::WindowEvent::Resized(window_size) => {
                        display.resize(window_size.into());
                    }
                    _ => (),
                },
                // By requesting a redraw in response to a AboutToWait event we get continuous rendering.
                // For applications that only change due to user input you could remove this handler.
                winit::event::Event::AboutToWait => {
                    window.request_redraw();
                }
                _ => (),
            }
        })
        .unwrap();
}
