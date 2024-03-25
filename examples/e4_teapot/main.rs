use glium::{uniform, Surface};
use opengl_examples_rs::helper;

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

    let program = helper::gl_draw_program(
        &display,
        "examples/e4_teapot/shaders/vert.glsl",
        "examples/e4_teapot/shaders/frag.glsl",
    )
    .expect("Fail to create program");

    let params = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::draw_parameters::DepthTest::IfLess,
            write: true,
            ..Default::default()
        },
        backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
        ..Default::default()
    };

    let fn_on_redraw = || {
        let mut target = display.draw();

        target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);
        let matrix = [
            [0.01, 0.0, 0.0, 0.0],
            [0.0, 0.01, 0.0, 0.0],
            [0.0, 0.0, 0.01, 0.0],
            [0.0, 0.0, 1.5, 1.0f32],
        ];
        // the direction of the light
        let light = [-1.0, 0.4, 0.9f32];
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
    };

    helper::event_loop_run(event_loop, &display, &window, fn_on_redraw);
}
