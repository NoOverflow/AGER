use glib::Object;
use glium::Program;
use gtk::glib;
use gtk::prelude::*;
use gtk::TextMark;
use gtk::TextView;
use gtk4 as gtk;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};

use crate::gameboy::Gameboy;
mod imp;

glib::wrapper! {
    pub struct DebuggerWindow(ObjectSubclass<imp::DebuggerWindow>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

#[derive(Clone)]
pub struct RenderContext {
    pub render_texture: Option<Rc<RefCell<glium::texture::texture2d::Texture2d>>>,
    pub rx: Arc<Mutex<Receiver<Vec<u32>>>>,
    pub program: Option<Rc<RefCell<Program>>>,
}

impl DebuggerWindow {
    pub fn new(_: Arc<Mutex<Gameboy>>, app: &gtk::Application) -> Self {
        Object::builder()
            .property("application", app)
            .build()
            .unwrap()
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

        let debug_text = format!("{:x?}: {:x?}  ({:x?})                           A:{:x?} F:{:x?} B:{:x?} C:{:x?} D:{:x?} E:{:x?} H:{:x?} L:{:x?} LY:{:x?} SP:{:x?}\nIME: {}\n*PC: {:x?} *PC+1 {:x?} *PC+2 {:x?} *PC+3 {:x?}\nJoystick {:x?}",
            gb_ref.cpu.registers.pc - 1,
            str_instruction,
            current_instruction,
            gb_ref.cpu.registers.a,
            u8::from(gb_ref.cpu.registers.f),
            gb_ref.cpu.registers.b,
            gb_ref.cpu.registers.c,
            gb_ref.cpu.registers.d,
            gb_ref.cpu.registers.e,
            gb_ref.cpu.registers.h,
            gb_ref.cpu.registers.l,
            gb_ref.mem_map.ly,
            gb_ref.cpu.registers.sp,
            gb_ref.cpu.ime,
            gb_ref.mem_map.read_u8(gb_ref.cpu.registers.pc as usize - 1),
            gb_ref.mem_map.read_u8(gb_ref.cpu.registers.pc as usize ),
            gb_ref.mem_map.read_u8(gb_ref.cpu.registers.pc as usize + 1),
            gb_ref.mem_map.read_u8(gb_ref.cpu.registers.pc as usize + 2),
            u8::from(gb_ref.mem_map.jpad)
        );

        text_view.buffer().set_text(&debug_text);
    }
}
