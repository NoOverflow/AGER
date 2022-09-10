use cgmath::Matrix4;
use gio::prelude::*;
use glium::backend::{Backend, Context, Facade};
use glium::texture::{RawImage2d, Texture2dDataSink};
use glium::uniforms::MagnifySamplerFilter;
use glium::{implement_vertex, uniform, Frame, Program, Surface, Texture2d, VertexBuffer};
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

    pub fn init_window(&mut self, rx: Arc<Mutex<Receiver<Vec<u32>>>>) {
        let application = gtk::Application::new(Some("com.nooverflow.ager"), Default::default());
        let mut render_context: Rc<RefCell<RenderContext>> = Rc::new(RefCell::new(RenderContext {
            render_texture: None,
            rx: rx,
            program: None,
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
        let buffer: Vec<u32>;

        match render_context.rx.lock().unwrap().recv() {
            Ok(rx_buffer) => buffer = rx_buffer,
            _ => panic!("Couldn't receive render buffer"),
        }

        implement_vertex!(Vertex, position);

        let (rect_vertices, rect_indices) = {
            let ib_data: Vec<u16> = vec![0, 1, 2, 1, 3, 2];
            let vb: VertexBuffer<Vertex> = glium::VertexBuffer::empty_dynamic(facade, 4).unwrap();
            // Creates an index buffer showing how the triangles would be made from the four points.
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
        tex.as_surface().fill(&frame, MagnifySamplerFilter::Nearest);

        let (target_w, target_h) = frame.get_dimensions();

        frame.blit_whole_color_to(
            &frame,
            &glium::BlitTarget {
                left: 0,
                bottom: target_h,
                width: -(target_w as i32),
                height: (target_h as i32),
            },
            MagnifySamplerFilter::Nearest,
        );

        let uniforms = uniform! {
            projection: perspective,
            tex: &tex,
        };

        frame.draw(
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
        let glarea = GLArea::builder().build();

        window.set_child(Some(&glarea));
        window.show();

        let facade: gtk4_glium::GtkFacade = gtk4_glium::GtkFacade::from_glarea(&glarea).unwrap();

        //
        let vertex_shader_src = r#"
            #version 140
            in vec2 position;
            uniform mat4 projection;
            out vec2 v_tex_coords;

            void main() {
                if (gl_VertexID % 4 == 0) { // First vertex
                    v_tex_coords = vec2(0.0, 1.0);
                } else if (gl_VertexID % 4 == 1) { // Second vertex
                    v_tex_coords = vec2(1.0, 1.0);
                } else if (gl_VertexID % 4 == 2) { // Third vertex
                    v_tex_coords = vec2(0.0, 0.0);
                } else { // Fourth vertex
                    v_tex_coords = vec2(1.0, 0.0);
                }
                gl_Position = projection * vec4(position, 0.0, 1.0);
            }
        "#;

        let fragment_shader_src = r#"
            #version 140
            in vec2 v_tex_coords;
            out vec4 color;
            uniform sampler2D tex;
            void main() {
                color = texture(tex, v_tex_coords);
            }
        "#;

        render_context.to_owned().borrow_mut().program = Some(Rc::new(RefCell::new(
            glium::Program::from_source(&facade, vertex_shader_src, fragment_shader_src, None)
                .unwrap(),
        )));

        glarea.connect_render(clone!(@strong render_context => move |_glarea, _glcontext|
            Self::gl_render(render_context.borrow_mut(), facade.get_context(), _glarea, _glcontext)
        ));

        let frame_time = Duration::new(0, 1_000_000_000 / 24);
        glib::source::timeout_add_local(frame_time, move || {
            glarea.queue_draw();
            glib::source::Continue(true)
        });
    }
}
