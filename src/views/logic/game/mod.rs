use cgmath::Matrix4;
use gio::prelude::*;
use glium::backend::{Context, Facade};
use glium::uniforms::{MagnifySamplerFilter, MinifySamplerFilter};
use glium::{implement_vertex, uniform, Frame, Program, Surface, VertexBuffer};
use gtk::builders::{EventControllerKeyBuilder, PanedBuilder};
use gtk::gdk::{self, GLContext};
use gtk::glib::clone;
use gtk::{glib, Paned};
use gtk::{prelude::*, Inhibit};
use gtk::{ApplicationWindow, GLArea};
use gtk4 as gtk;
use std::cell::{RefCell, RefMut};
use std::rc::Rc;
use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use crate::gameboy::Gameboy;
use crate::views::logic::debugger::DebuggerWindow;

const SCALE: u32 = 4;
const WINDOW_WIDTH: u32 = 160; // 160 * SCALE;
const WINDOW_HEIGHT: u32 = 144; // 144 * SCALE;

pub struct GameWindow {}

#[derive(Clone)]
pub struct RenderContext {
    pub render_texture: Option<Rc<RefCell<glium::texture::texture2d::Texture2d>>>,
    pub rx: Arc<Mutex<Receiver<Vec<u32>>>>,
    pub program: Option<Rc<RefCell<Program>>>,
}

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}

impl GameWindow {
    pub fn new() -> Self {
        GameWindow {}
    }

    fn create_sub_windows(app: &gtk::Application, gb: Arc<Mutex<Gameboy>>) {
        DebuggerWindow::new(gb, app).present();
    }

    pub fn init_window(&mut self, gb: Arc<Mutex<Gameboy>>, rx: Arc<Mutex<Receiver<Vec<u32>>>>) {
        let application = gtk::Application::new(Some("com.nooverflow.ager"), Default::default());
        let render_context: Rc<RefCell<RenderContext>> = Rc::new(RefCell::new(RenderContext {
            render_texture: None,
            rx,
            program: None,
        }));

        gio::resources_register_include!("debugger.gresource")
            .expect("Failed to register resources.");
        application.connect_activate(clone!(
            @weak render_context, @weak gb => move | app | Self::init_window_gtk(gb, render_context, app)
        ));
        application.run();
    }

    fn gl_render(
        render_context: RefMut<RenderContext>,
        facade: &Rc<Context>,
        _gl_area: &GLArea,
        _gl_context: &GLContext,
    ) -> Inhibit {
        let context = facade.get_context();
        let mut frame = Frame::new(context.clone(), context.get_framebuffer_dimensions());
        let buffer: Vec<u32> = render_context.rx.lock().unwrap().recv().unwrap();

        implement_vertex!(Vertex, position);

        let (rect_vertices, rect_indices) = {
            let ib_data: Vec<u16> = vec![0, 1, 2, 1, 3, 2];
            let vb: VertexBuffer<Vertex> = glium::VertexBuffer::empty_dynamic(facade, 4).unwrap();
            let ib = glium::IndexBuffer::new(
                context,
                glium::index::PrimitiveType::TrianglesList,
                &ib_data,
            )
            .unwrap();

            (vb, ib)
        };

        {
            let vb_data = vec![
                Vertex {
                    position: [0.0, 0.0],
                },
                Vertex {
                    position: [WINDOW_WIDTH as f32, 0.0],
                },
                Vertex {
                    position: [0.0, WINDOW_HEIGHT as f32],
                },
                Vertex {
                    position: [WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32],
                },
            ];
            rect_vertices.write(&vb_data);
        }

        let perspective = {
            let matrix: Matrix4<f32> = cgmath::ortho(
                0.0,
                WINDOW_WIDTH as f32,
                0.0,
                WINDOW_HEIGHT as f32,
                -1.0,
                1.0,
            );
            Into::<[[f32; 4]; 4]>::into(matrix)
        };

        let rawimage2d = glium::texture::RawImage2d {
            data: std::borrow::Cow::Borrowed(&buffer),
            width: WINDOW_WIDTH as u32,
            height: WINDOW_HEIGHT as u32,
            format: glium::texture::ClientFormat::U8U8U8U8,
        };

        let tex = glium::Texture2d::new(facade, rawimage2d).unwrap();
        let behavior = glium::uniforms::SamplerBehavior {
            minify_filter: MinifySamplerFilter::Nearest,
            magnify_filter: MagnifySamplerFilter::Nearest,
            ..Default::default()
        };
        let uniforms = uniform! {
            projection: perspective,
            tex: glium::uniforms::Sampler(&tex, behavior),
        };

        match frame.draw(
            &rect_vertices,
            &rect_indices,
            &(render_context.program.as_ref().unwrap().as_ref().borrow()),
            &uniforms,
            &Default::default(),
        ) {
            Ok(_) => (),
            _ => panic!("Couldn't render"),
        }
        frame.finish().unwrap();
        gtk::Inhibit(true)
    }

    fn init_window_gtk(
        gb: Arc<Mutex<Gameboy>>,
        render_context: Rc<RefCell<RenderContext>>,
        application: &gtk::Application,
    ) {
        let window = ApplicationWindow::builder()
            .application(application)
            .default_width(WINDOW_WIDTH as i32 * SCALE as i32)
            .default_height(WINDOW_HEIGHT as i32 * SCALE as i32)
            .resizable(false)
            .title("AGER")
            .build();
        let glarea = GLArea::builder()
            .width_request(100)
            .hexpand(true)
            .hexpand_set(false)
            .build();
        let paned: Paned = PanedBuilder::new()
            .orientation(gtk::Orientation::Horizontal)
            .build();

        paned.set_start_child(Some(&glarea));

        let event_controller = EventControllerKeyBuilder::new().build();

        event_controller.connect_key_released(
            clone!(@strong gb => move |_event_controller, keyval, _keycode, _state| {
                let mut gb = gb.lock().unwrap();

                match keyval {
                    gdk::Key::a => {
                        gb.mem_map.jpad.select_action = false;
                        gb.mem_map.jpad.p10_in = false;
                        gb.mem_map.iflag.hi_lo = true;
                    },
                    _ => {}
                }
            }),
        );

        event_controller.connect_key_pressed(
            clone!(@strong gb => move |_event_controller, keyval, _keycode, _state| {
                let mut gb = gb.lock().unwrap();

                match keyval {
                    gdk::Key::a => {
                        gb.mem_map.jpad.select_action = true;
                        gb.mem_map.jpad.p10_in = true;
                        gb.mem_map.iflag.hi_lo = true;
                        println!("Key A pressed");
                    },
                    gdk::Key::p => {
                        gb.debugger.state.paused = !gb.debugger.state.paused;
                        println!("Paused: {}", gb.debugger.state.paused);
                    },
                    gdk::Key::m => {
                        gb.debugger.state.step = true;
                    },
                    gdk::Key::i => {
                        gb.cpu.ime = !gb.cpu.ime;
                    },
                    gdk::Key::o => {
                        gb.debugger.state.dumping = !gb.debugger.state.dumping;
                    },
                    gdk::Key::F1 => {

                    }
                    _ => {}
                }
                Inhibit(false)
            }),
        );
        event_controller.set_propagation_phase(gtk::PropagationPhase::Capture);
        window.add_controller(&event_controller);
        window.set_child(Some(&paned));
        window.present();

        let facade: gtk4_glium::GtkFacade = gtk4_glium::GtkFacade::from_glarea(&glarea).unwrap();
        let vertex_shader_src = include_str!("../../../../res/shaders/default.vs");
        let fragment_shader_src = include_str!("../../../../res/shaders/default.fs");

        render_context.borrow_mut().program = Some(Rc::new(RefCell::new(
            glium::Program::from_source(&facade, vertex_shader_src, fragment_shader_src, None)
                .unwrap(),
        )));

        glarea.connect_render(clone!(@strong render_context => move |_glarea, _glcontext|
            Self::gl_render(render_context.borrow_mut(), facade.get_context(), _glarea, _glcontext)
        ));

        let frame_time = Duration::new(0, 1_000_000_000 / 60);
        let gb_clone = gb.clone();

        glib::source::timeout_add_local(
            frame_time,
            clone!(@weak gb_clone => @default-return glib::source::Continue(true), move || {
                glarea.queue_draw();
                glib::source::Continue(true)
            }),
        );
        GameWindow::create_sub_windows(application, gb);
    }
}
