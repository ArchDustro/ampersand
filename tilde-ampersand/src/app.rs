use gdk::gio::traits::ApplicationExt;
use gtk::{Application,prelude::*, ApplicationWindow};
use crate::{builders, container, window};

pub struct App {
    pub application: Application,
    pub windows: Vec<window::Window>
}

impl App {
    pub fn add_window(mut self, window: window::Window) {
        self.windows.push(window);
    }
    pub fn run(self) {
        self.application.connect_activate(move |_| {
            for win in &self.windows {
                win.window.show_all();
            }
        });
    }
}