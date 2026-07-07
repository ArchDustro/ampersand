use gdk;
use gtk::builders;
use gtk::Application;
use gtk::ApplicationWindow;

use crate::app::*;
use crate::window::*;


pub struct AppBuilder {
    pub builder: builders::ApplicationBuilder,
    pub app_id: String,
}

pub struct WindowBuilder {
    pub builder: builders::ApplicationWindowBuilder,
    pub title: String,
    pub behavior: crate::window::WindowBehavior,
}

impl AppBuilder {
    pub fn new(app_id: &str) -> AppBuilder {
        // Set the program class before creating windows so WM_CLASS is stable.
        gdk::set_program_class("tilde-ampersand");

        AppBuilder {
            builder: Application::builder().application_id(app_id),
            app_id: app_id.to_string()
        }
    }
    pub fn build(self) -> App {
        App {
            application: self.builder.build(),
        }
    }
}

impl WindowBuilder {
    pub fn new(app: &App, title: String) -> WindowBuilder {
        Self::new_from_app(&app.application, title)
    }

    pub fn new_from_app(gtk_app: &gtk::Application, title: String) -> WindowBuilder {
        let window = ApplicationWindow::builder()
            .application(gtk_app)
            .title(&title)
            .type_hint(gdk::WindowTypeHint::Normal);
        WindowBuilder { 
            builder: window, 
            title,
            behavior: crate::window::WindowBehavior::Tiled, // Default to tiled for BSPWM
        }
    }
    
    pub fn set_behavior(mut self, behavior: crate::window::WindowBehavior) -> Self {
        self.behavior = behavior;
        self
    }
    
    pub fn set_decorated(mut self, is_decorated:bool, show_in_taskbar:bool) -> Self {
        self.builder = self.builder
            .decorated(is_decorated)
            .skip_taskbar_hint(show_in_taskbar);

        self
    }
    pub fn set_title(mut self, title: String) -> Self {
        self.builder = self.builder.title(title);
        self
    }
    pub fn set_resizable(mut self, can_resize:bool) -> Self {
        self.builder = self.builder.resizable(can_resize);
        self
    }

    pub fn build(self) -> Window {
        let window = Window {
            window: self.builder.build(),
            behavior: self.behavior,
        };
        window.apply_behavior();
        window
    }
    
}
