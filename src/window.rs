use ::gdk::EventKey;
use cgmath::Matrix4;
use gio::prelude::*;
use glium::backend::{Context, Facade};
use glium::glutin::event;
use glium::uniforms::{MagnifySamplerFilter, MinifySamplerFilter};
use glium::{implement_vertex, uniform, Frame, Program, Surface, VertexBuffer};
use gtk::builders::{
    EventControllerKeyBuilder, PanedBuilder, ScrolledWindowBuilder, TextViewBuilder,
};
use gtk::gdk::{self, GLContext};
use gtk::glib::clone;
use gtk::{glib, Paned, ScrolledWindow, TextMark, TextView};
use gtk::{prelude::*, Inhibit};
use gtk::{ApplicationWindow, GLArea};
use gtk4 as gtk;
use std::cell::{RefCell, RefMut};
use std::rc::Rc;
use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use crate::gameboy::Gameboy;

const SCALE: u32 = 4;
const WINDOW_WIDTH: u32 = 160; // 160 * SCALE;
const WINDOW_HEIGHT: u32 = 144; // 144 * SCALE;

pub struct Window {}

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

impl Window {
    pub fn new() -> Self {
        Window {}
    }

    pub fn init_window(&mut self, gb: Arc<Mutex<Gameboy>>, rx: Arc<Mutex<Receiver<Vec<u32>>>>) {
        let application = gtk::Application::new(Some("com.nooverflow.ager"), Default::default());
        let render_context: Rc<RefCell<RenderContext>> = Rc::new(RefCell::new(RenderContext {
            render_texture: None,
            rx,
            program: None,
        }));

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
            &(render_context
                .program
                .as_ref()
                .unwrap()
                .to_owned()
                .as_ref()
                .borrow()),
            &uniforms,
            &Default::default(),
        ) {
            Ok(_) => (),
            _ => panic!("Couldn't render"),
        }
        frame.finish().unwrap();
        gtk::Inhibit(true)
    }

    fn debugger_draw(gb: Arc<Mutex<Gameboy>>, text_view: &TextView, _text_mark_end: &TextMark) {
        let gb_ref = gb.lock().unwrap();
        let mut current_instruction = gb_ref.mem_map.read_u8(gb_ref.cpu.registers.pc as usize);
        let extended_instruction: bool = current_instruction == 0xCB;

        if extended_instruction {
            current_instruction = gb_ref.mem_map.read_u8(gb_ref.cpu.registers.pc as usize + 1);
        }
        let str_instruction = match if extended_instruction {
            gb_ref
                .debugger
                .translation_table_extended
                .get(&current_instruction)
        } else {
            gb_ref.debugger.translation_table.get(&current_instruction)
        } {
            Some(instruction) => instruction,
            None => "Unknown instruction",
        };

        let debug_text = format!("{:x?}: {:x?}                            A:{:x?} F:{:x?} B:{:x?} C:{:x?} D:{:x?} E:{:x?} H:{:x?} L:{:x?} LY:{:x?} SP:{:x?}",
            gb_ref.cpu.registers.pc - 1,
            str_instruction,
            gb_ref.cpu.registers.a,
            u8::from(gb_ref.cpu.registers.f),
            gb_ref.cpu.registers.b,
            gb_ref.cpu.registers.c,
            gb_ref.cpu.registers.d,
            gb_ref.cpu.registers.e,
            gb_ref.cpu.registers.h,
            gb_ref.cpu.registers.l,
            gb_ref.mem_map.ly,
            gb_ref.cpu.registers.sp
        );

        text_view.buffer().set_text(&debug_text);
    }

    fn init_window_gtk(
        gb: Arc<Mutex<Gameboy>>,
        render_context: Rc<RefCell<RenderContext>>,
        application: &gtk::Application,
    ) {
        let window = ApplicationWindow::builder()
            .application(application)
            .default_width(WINDOW_WIDTH as i32 * SCALE as i32 * 2)
            .default_height(WINDOW_HEIGHT as i32 * SCALE as i32)
            .title("AGER")
            .build();
        let glarea = GLArea::builder()
            .width_request(50)
            .hexpand(false)
            .hexpand_set(false)
            .build();
        let paned: Paned = PanedBuilder::new()
            .orientation(gtk::Orientation::Horizontal)
            .build();
        let text_view: TextView = TextViewBuilder::new().hexpand_set(true).build();
        let scroll_window: ScrolledWindow = ScrolledWindowBuilder::new().hexpand_set(true).build();

        scroll_window.set_child(Some(&text_view));
        paned.set_start_child(Some(&glarea));
        paned.set_end_child(Some(&scroll_window));

        /*window.connect("key_press_event", false, |values| {
            // Get the key pressed code from value
            let keyval = values[1].get::<u32>().unwrap();

            println!("Keyval: {}", keyval);
            Some(glib::value::Value::from_type(glib::types::Type::BOOL))
        });*/

        let event_controller = EventControllerKeyBuilder::new().build();

        event_controller.connect_key_pressed(
            clone!(@strong gb => move |_event_controller, keyval, _keycode, _state| {
                let mut gb = gb.lock().unwrap();

                match keyval {
                    gdk::Key::p => {
                        gb.debugger.state.paused = !gb.debugger.state.paused;
                        println!("Paused: {}", gb.debugger.state.paused);
                    },
                    gdk::Key::m => {
                        gb.debugger.state.step = true;
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
        let vertex_shader_src = include_str!("../res/shaders/default.vs");
        let fragment_shader_src = include_str!("../res/shaders/default.fs");

        render_context.borrow_mut().program = Some(Rc::new(RefCell::new(
            glium::Program::from_source(&facade, vertex_shader_src, fragment_shader_src, None)
                .unwrap(),
        )));

        glarea.connect_render(clone!(@strong render_context => move |_glarea, _glcontext|
            Self::gl_render(render_context.borrow_mut(), facade.get_context(), _glarea, _glcontext)
        ));

        let frame_time = Duration::new(0, 1_000_000_000 / 60);
        let debugger_interval = Duration::new(0, 1_000_000_000 / 60);
        let gb_clone = gb.clone();
        let text_mark_end =
            text_view
                .buffer()
                .create_mark(None, &text_view.buffer().end_iter(), false);

        glib::source::timeout_add_local(
            debugger_interval,
            clone!(@weak gb_clone => @default-return glib::source::Continue(true), move || {
                Window::debugger_draw(gb_clone, &text_view, &text_mark_end);
                glib::source::Continue(true)
            }),
        );

        glib::source::timeout_add_local(
            frame_time,
            clone!(@weak gb_clone => @default-return glib::source::Continue(true), move || {
                glarea.queue_draw();
                glib::source::Continue(true)
            }),
        );
    }
}
