use std::{cell::RefCell, rc::Rc};

use glam::Vec3;
use glium::{uniform, Surface};
use opengl_examples_rs::{camera::Camera, helper, obj_loader};

fn main() {
    let event_loop = winit::event_loop::EventLoopBuilder::new()
        .build()
        .expect("event loop building");
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .with_title("Basic shading")
        .with_inner_size(1600, 1200)
        .build(&event_loop);

    let (vertices, indices) = obj_loader::load_obj("examples/model/bugatti.obj");

    let vb = glium::VertexBuffer::new(&display, &vertices).unwrap();
    let ib = glium::IndexBuffer::new(
        &display,
        glium::index::PrimitiveType::TrianglesList,
        &indices,
    )
    .unwrap();

    let program = helper::gl_draw_program(
        &display,
        "examples/e5_basic_shading/shaders/vert.glsl",
        "examples/e5_basic_shading/shaders/frag.glsl",
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

    let position = 25. * Vec3::Z - 5. * Vec3::Y;
    let camera = Rc::new(RefCell::new(
        Camera::new()
            .with_position(position)
            .with_up_lookat(position.cross(Vec3::X), Vec3::ZERO)
            .with_move_sensitivity(2.)
            .with_rotate_sensitivity(0.05),
    ));

    let fn_on_redraw = || {
        let mut target = display.draw();

        target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);

        // the direction of the light
        let light = [-1.0, 0.4, 0.9f32];

        camera.borrow_mut().update();
        target
            .draw(
                &vb,
                &ib,
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
