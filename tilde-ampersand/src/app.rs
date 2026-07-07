use gdk::gio::traits::ApplicationExt;
use gtk::{Application,prelude::*};
use crate::{window};

#[derive(Clone)]
pub struct App {
    pub application: Application,
}

impl App {
    pub fn run<F>(self, window_builder: F) 
    where
        F: Fn(&Application) -> Vec<window::Window> + 'static
    {
        self.application.connect_activate(move |app| {
            let windows = window_builder(app);
            
            for win in windows.iter() {
                win.apply_behavior();
                win.window.show_all();
                win.window.present();
            }
            
            // Keep windows alive for the lifetime of the app
            // by leaking them into a Box that persists
            Box::leak(Box::new(windows));
        });

        self.application.run();
    }
}