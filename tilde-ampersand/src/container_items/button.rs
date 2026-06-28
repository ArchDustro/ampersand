use gtk::prelude::*;
use gtk::*;
use std::boxed::Box;

pub struct Button {
    pub button: gtk::Button,
    pub on_click: Box<dyn Fn()>,
}

impl Button {
    pub fn new(label: &str, on_click: Box<dyn Fn()>) -> Button {
        let button = gtk::Button::with_label(label);
        Button { button, on_click }
    }

    pub fn set_on_click(&mut self, on_click: Box<dyn Fn()>) -> &mut Self {
        self.on_click = on_click;
        self
    }

    pub fn clicked(&self) {
        self.button.connect_clicked(move |_| {
            (self.on_click)();
        });
    }
    pub fn set_label(&self, new_label: &str) -> &Self {
        self.button.set_label(new_label);
        self
    }
}

