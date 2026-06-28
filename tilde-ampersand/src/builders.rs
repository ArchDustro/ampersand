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
}

impl AppBuilder {
    pub fn new(app_id: &str) -> AppBuilder {
        AppBuilder {
            builder: Application::builder().application_id(app_id),
            app_id: app_id.to_string()
        }
    }
    pub fn build(self) -> App {
        App {
            application: self.builder.build(),
            windows: Vec::new(),
        }
    }
}

impl WindowBuilder {
    pub fn new(app: &App, title: String) -> WindowBuilder {
        let window = ApplicationWindow::builder()
            .application(&app.application)
            .title(&title);
        WindowBuilder { builder: window, title }

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
        Window {
            window: self.builder.build(),
        }
    }
    
}
