use gio::prelude::*;
use glium::backend::{Backend, Context, Facade};
use glium::{Frame, Surface, Texture2d};
use gtk::gdk::GLContext;
use gtk::glib;
use gtk::glib::clone;
use gtk::{prelude::*, Inhibit};
use gtk::{ApplicationWindow, GLArea};
use gtk4 as gtk;
use gtk4_glium::GtkFacade;
use std::cell::{RefCell, RefMut};
use std::rc::Rc;
use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};
use std::time::Duration;

const SCALE: u32 = 4;
const WINDOW_WIDTH: u32 = 160; // 160 * SCALE;
const WINDOW_HEIGHT: u32 = 144; // 144 * SCALE;

pub struct Window {}

#[derive(Clone)]
pub struct RenderContext {
    pub index: u32,
    pub render_texture: Option<Rc<RefCell<glium::texture::texture2d::Texture2d>>>,
    pub rx: Arc<Mutex<Receiver<Vec<u32>>>>,
}

/* pub struct RenderContext {
    pub frame: Option<Rc<RefCell<glium::Frame>>>,
    pub render_texture: Option<Rc<RefCell<glium::texture::texture2d::Texture2d>>>,
    pub rx: Rc<RefCell<Receiver<Vec<u32>>>>,
} */

impl Window {
    pub fn new() -> Self {
        Window {}
    }

    fn draw_buffer(render_context: RefMut<RenderContext>, frame: &Frame, buffer: &Vec<u32>) {}

    pub fn init_window(&mut self, rx: Arc<Mutex<Receiver<Vec<u32>>>>) {
        let application = gtk::Application::new(Some("com.nooverflow.ager"), Default::default());
        let mut render_context: Rc<RefCell<RenderContext>> = Rc::new(RefCell::new(RenderContext {
            index: 9,
            render_texture: None,
            rx: rx,
        }));

        application.connect_activate(clone!(
            @weak render_context => move | app | Self::init_window_gtk(render_context, app)
        ));
        application.run();
    }

    fn gl_render(
        render_context: RefMut<RenderContext>,
        facade: &Rc<Context>,
        _glArea: &GLArea,
        _glcontext: &GLContext,
    ) -> Inhibit {
        let context = facade.get_context();
        let mut frame = Frame::new(context.clone(), context.get_framebuffer_dimensions());

        frame.clear_color(0.0, 0.0, 0.0, 1.0);
        println!("{}", render_context.index);

        let mut buffer: Vec<u32> = Vec::new();

        match render_context.rx.lock().unwrap().recv() {
            Ok(rx_buffer) => buffer = rx_buffer,
            Error => (),
        }

        let interpolation_type = glium::uniforms::MagnifySamplerFilter::Nearest;
        let rawimage2d = glium::texture::RawImage2d {
            data: std::borrow::Cow::Borrowed(&buffer),
            width: WINDOW_WIDTH as u32,
            height: WINDOW_HEIGHT as u32,
            format: glium::texture::ClientFormat::U8U8U8U8,
        };

        render_context
            .to_owned()
            .render_texture
            .as_mut()
            .unwrap()
            .borrow_mut()
            .write(
                glium::Rect {
                    left: 0,
                    bottom: 0,
                    width: WINDOW_WIDTH as u32,
                    height: WINDOW_HEIGHT as u32,
                },
                rawimage2d,
            );

        let (target_w, target_h) = frame.get_dimensions();

        render_context
            .to_owned()
            .render_texture
            .as_mut()
            .unwrap()
            .borrow_mut()
            .as_surface()
            .blit_whole_color_to(
                &frame,
                &glium::BlitTarget {
                    left: 0,
                    bottom: target_h,
                    width: target_w as i32,
                    height: -(target_h as i32),
                },
                interpolation_type,
            );

        frame.finish().unwrap();
        gtk::Inhibit(true)
    }

    fn init_window_gtk(render_context: Rc<RefCell<RenderContext>>, application: &gtk::Application) {
        let window = ApplicationWindow::builder()
            .application(application)
            .default_width(WINDOW_WIDTH as i32 * SCALE as i32)
            .default_height(WINDOW_HEIGHT as i32 * SCALE as i32)
            .title("AGER")
            .build();
        let glarea = GLArea::builder().vexpand(true).build();

        window.set_child(Some(&glarea));
        window.show();

        let facade: gtk4_glium::GtkFacade = gtk4_glium::GtkFacade::from_glarea(&glarea).unwrap();

        render_context.borrow_mut().index = 3;
        render_context.borrow_mut().render_texture = Some(Rc::new(RefCell::new(
            glium::texture::texture2d::Texture2d::empty_with_format(
                &facade,
                glium::texture::UncompressedFloatFormat::U8U8U8U8,
                glium::texture::MipmapsOption::NoMipmap,
                WINDOW_WIDTH as u32,
                WINDOW_HEIGHT as u32,
            )
            .unwrap(),
        )));

        glarea.connect_render(clone!(@strong render_context => move |_glarea, _glcontext|
            Self::gl_render(render_context.borrow_mut(), facade.get_context(), _glarea, _glcontext)
        ));

        let frame_time = Duration::new(0, 1_000_000_000 / 60);
        glib::source::timeout_add_local(frame_time, move || {
            glarea.queue_draw();
            glib::source::Continue(true)
        });
    }
}
