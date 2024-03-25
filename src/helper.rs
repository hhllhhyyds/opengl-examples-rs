use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

use glium::glutin::surface::WindowSurface;
use glium::{Display, Program, ProgramCreationError};
use winit::event::WindowEvent;
use winit::event_loop::EventLoop;
use winit::window::Window;

pub fn load_shader<P>(path: P) -> io::Result<String>
where
    P: AsRef<Path>,
{
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

pub fn event_loop_run<T, F>(
    event_loop: EventLoop<T>,
    display: &Display<WindowSurface>,
    window: &Window,
    mut on_redraw: F,
) where
    F: FnMut() -> (),
{
    event_loop
        .run(move |ev, window_target| {
            match ev {
                winit::event::Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => {
                        window_target.exit();
                    }
                    WindowEvent::RedrawRequested => on_redraw(),
                    WindowEvent::Resized(window_size) => {
                        display.resize(window_size.into());
                    }
                    _ => (),
                },
                // By requesting a redraw in response to a AboutToWait event we get continuous rendering.
                // For applications that only change due to user input we could remove this handler.
                winit::event::Event::AboutToWait => {
                    window.request_redraw();
                }
                _ => (),
            }
        })
        .expect("Eventloop crash");
}

pub fn gl_draw_program<P: AsRef<Path> + std::fmt::Display>(
    display: &Display<WindowSurface>,
    vertex_shader_path: P,
    fragment_shader_path: P,
) -> Result<Program, ProgramCreationError> {
    let vertex_shader_src = load_shader(vertex_shader_path.as_ref())
        .expect(format!("Fail to load {}", vertex_shader_path).as_str());

    let fragment_shader_src = load_shader(fragment_shader_path.as_ref())
        .expect(format!("Fail to load {}", fragment_shader_path).as_str());

    glium::Program::from_source(
        display,
        vertex_shader_src.as_str(),
        fragment_shader_src.as_str(),
        None,
    )
}
