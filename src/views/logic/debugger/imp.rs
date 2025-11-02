use std::borrow::Borrow;
use std::cell::Ref;
use std::sync::{Arc, Mutex};

use glib::subclass::InitializingObject;
use gtk::ffi::GtkButton;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate};
use gtk::{prelude::*, Button};
use gtk4 as gtk;

use crate::gameboy::Gameboy;

#[derive(CompositeTemplate, Default)]
#[template(resource = "/org/gtk_rs/example/window.ui")]
pub struct DebuggerWindow {
    pub count: u8,
    pub gb: Option<Arc<Mutex<Gameboy>>>, //#[template_child]
                                         //pub button: TemplateChild<Button>,
}

#[glib::object_subclass]
impl ObjectSubclass for DebuggerWindow {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "MyGtkAppWindow";
    type Type = super::DebuggerWindow;
    type ParentType = gtk::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for DebuggerWindow {
    fn constructed(&self, obj: &Self::Type) {
        self.parent_constructed(obj);

        //self.button.connect_clicked(move |button| {});
    }
}

impl WidgetImpl for DebuggerWindow {}
impl WindowImpl for DebuggerWindow {}
impl ApplicationWindowImpl for DebuggerWindow {}
