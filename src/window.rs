use glium::glutin::dpi::{LogicalSize, PhysicalSize};
use glium::glutin::event::{Event, KeyboardInput, WindowEvent};
use glium::glutin::platform::windows::WindowBuilderExtWindows;
use glium::Surface;
use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::thread;

const SCALE: u32 = 4;
const WINDOW_WIDTH: u32 = 160; // 160 * SCALE;
const WINDOW_HEIGHT: u32 = 144; // 144 * SCALE;

fn create_window_builder() -> glium::glutin::window::WindowBuilder {
    return glium::glutin::window::WindowBuilder::new()
        .with_drag_and_drop(false)
        .with_inner_size(glium::glutin::dpi::LogicalSize::<u32>::from((
            WINDOW_WIDTH as u32,
            WINDOW_HEIGHT as u32,
        )))
        .with_title("AGER");
}

fn set_window_size(window: &glium::glutin::window::Window) {
    let dpi = window.scale_factor();
    let physical_size = PhysicalSize::<u32>::from((WINDOW_WIDTH * SCALE, WINDOW_HEIGHT * SCALE));
    let logical_size = LogicalSize::<u32>::from_physical(physical_size, dpi);

    window.set_inner_size(logical_size);
}

fn draw_buffer(
    display: &glium::Display,
    buffer: &Vec<u32>,
    texture: &mut glium::texture::texture2d::Texture2d,
) {
    let interpolation_type = glium::uniforms::MagnifySamplerFilter::Nearest;
    let rawimage2d = glium::texture::RawImage2d {
        data: std::borrow::Cow::Borrowed(buffer),
        width: WINDOW_WIDTH as u32,
        height: WINDOW_HEIGHT as u32,
        format: glium::texture::ClientFormat::U8U8U8U8,
    };

    texture.write(
        glium::Rect {
            left: 0,
            bottom: 0,
            width: WINDOW_WIDTH as u32,
            height: WINDOW_HEIGHT as u32,
        },
        rawimage2d,
    );

    // We use a custom BlitTarget to transform OpenGL coordinates to row-column coordinates
    let target = display.draw();
    let (target_w, target_h) = target.get_dimensions();
    texture.as_surface().blit_whole_color_to(
        &target,
        &glium::BlitTarget {
            left: 0,
            bottom: target_h,
            width: target_w as i32,
            height: -(target_h as i32),
        },
        interpolation_type,
    );
    target.finish().unwrap();
}

pub fn init_window(rx: Receiver<Vec<u32>>) {
    let window_builder = create_window_builder();
    let context_builder = glium::glutin::ContextBuilder::new();
    let mut event_loop = glium::glutin::event_loop::EventLoop::new();
    let display =
        glium::backend::glutin::Display::new(window_builder, context_builder, &event_loop).unwrap();
    let mut texture = glium::texture::texture2d::Texture2d::empty_with_format(
        &display,
        glium::texture::UncompressedFloatFormat::U8U8U8U8,
        glium::texture::MipmapsOption::NoMipmap,
        WINDOW_WIDTH as u32,
        WINDOW_HEIGHT as u32,
    )
    .unwrap();

    set_window_size(display.gl_window().window());
    event_loop.run(move |ev, i, cflow| {
        let mut stop_win = false;

        match ev {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => stop_win = true,
                _ => (),
            },
            Event::MainEventsCleared => match rx.recv() {
                Ok(buffer) => draw_buffer(&display, &buffer, &mut texture),
                Err(_) => stop_win = true,
            },
            _ => (),
        }
        if stop_win {
            *cflow = glium::glutin::event_loop::ControlFlow::Exit;
        }
    });
}
