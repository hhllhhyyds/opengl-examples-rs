use std::{cell::RefCell, rc::Rc};

use glam::Vec3;
use glium::{uniform, Surface};
use opengl_examples_rs::{camera::Camera, helper};

mod model;

fn main() {
    let event_loop = winit::event_loop::EventLoopBuilder::new()
        .build()
        .expect("event loop building");
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .with_title("Teapot")
        .with_inner_size(1600, 1200)
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

    let position = 250. * Vec3::Z - 50. * Vec3::Y;
    let camera = Rc::new(RefCell::new(
        Camera::new()
            .with_position(position)
            .with_up_lookat(position.cross(Vec3::X), Vec3::ZERO),
    ));

    let fn_on_redraw = || {
        let mut target = display.draw();

        target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);

        // the direction of the light
        let light = [-1.0, 0.4, 0.9f32];

        camera.borrow_mut().update();
        target
            .draw(
                (&positions, &normals),
                &indices,
                &program,
                &uniform! {
                    view: camera.borrow().view_matrix().to_cols_array_2d(),
                    perspective: camera.borrow().perspective_matrix().to_cols_array_2d(),
                    u_light: light
                },
                &params,
            )
            .unwrap();
        target.finish().unwrap();
    };

    helper::event_loop_run(event_loop, &display, &window, &camera, fn_on_redraw);
}
