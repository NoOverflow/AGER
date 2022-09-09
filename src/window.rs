use core::ffi::c_void;
use glium::backend::Context;
use glium::glutin::dpi::{LogicalSize, PhysicalSize};
use glium::glutin::event::{Event, WindowEvent};
use glium::glutin::platform::windows::WindowBuilderExtWindows;
use glium::Surface;
use glium::SwapBuffersError;
use gtk::prelude::*;
use std::sync::mpsc::Receiver;

use gio::prelude::*;
use std::env::args;

const SCALE: u32 = 4;
const WINDOW_WIDTH: u32 = 160; // 160 * SCALE;
const WINDOW_HEIGHT: u32 = 144; // 144 * SCALE;

struct GLAreaBackend {
    glarea: gtk::GLArea,
}

unsafe impl glium::backend::Backend for GLAreaBackend {
    fn swap_buffers(&self) -> Result<(), SwapBuffersError> {
        // GTK swaps the buffers after each "render" signal itself
        Ok(())
    }
    unsafe fn get_proc_address(&self, symbol: &str) -> *const c_void {
        gl_loader::get_proc_address(symbol) as *const _
    }
    fn get_framebuffer_dimensions(&self) -> (u32, u32) {
        // let allocation = self.glarea.get_allocation();
        //(allocation.width as u32, allocation.height as u32)
        (700, 700)
    }
    fn is_current(&self) -> bool {
        // GTK makes it current itself on each "render" signal
        true
    }
    unsafe fn make_current(&self) {
        self.glarea.make_current();
    }
}

impl GLAreaBackend {
    fn new(glarea: gtk::GLArea) -> Self {
        Self { glarea }
    }
}

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
    let application =
        gtk::Application::new(Some("com.github.gtk-rs.examples.basic"), Default::default());

    application.connect_activate(init_window_gtk);
    application.run();
}

fn init_window_gtk(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    window.set_title("AGER");
    window.set_border_width(0);
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(
        WINDOW_WIDTH as i32 * SCALE as i32,
        WINDOW_HEIGHT as i32 * SCALE as i32,
    );

    let glarea = gtk::GLArea::new();

    window.add(&glarea);
    window.show_all();

    gl_loader::init_gl();
    let context = unsafe {
        Context::new(
            GLAreaBackend::new(glarea.clone()),
            true,
            glium::debug::DebugCallbackBehavior::DebugMessageOnError,
        )
        .unwrap()
    };

    let counter = std::rc::Rc::new(std::cell::RefCell::new(0u32));

    glarea.connect_render(move |_glarea, _glcontext| {
        let mut frame = glium::Frame::new(context.clone(), context.get_framebuffer_dimensions());
        // this is where you can do your glium rendering

        let c = *counter.borrow() as f32 / 100.;
        let r = c.sin() / 2. + 0.5;
        let g = (c * 1.25).sin() / 2. + 0.5;
        let b = (c * 1.5).sin() / 2. + 0.5;
        frame.clear_color(r, g, b, 1.0);

        frame.finish().unwrap();
        *counter.borrow_mut() += 1;
        Inhibit(true)
    });

    // This makes the GLArea redraw 60 times per second
    // You can remove this if you want to redraw only when focused/resized
    /*const FPS: u32 = 60;
    glib::source::timeout_add_local(1_000 / FPS, move || {
        glarea.queue_draw();
        glib::source::Continue(true)
    });*/
}

pub fn init_window_old(rx: Receiver<Vec<u32>>) {
    let window_builder = create_window_builder();
    let context_builder = glium::glutin::ContextBuilder::new();
    let event_loop = glium::glutin::event_loop::EventLoop::new();
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
    event_loop.run(move |ev, _, cflow| {
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
