use gdk::traits::MonitorExt;
use gtk::{ApplicationWindow, traits::{ContainerExt, GtkWindowExt, WidgetExt}};
use crate::container;
use gtk::gdk;
use std::process::Command;
use glib::ControlFlow;

#[derive(Clone, Copy, Debug)]
pub enum WindowBehavior {
    Tiled,
    Floating,
}

#[derive(Clone)]
pub struct Window {
    pub window: ApplicationWindow,
    pub behavior: WindowBehavior,
}

pub enum WindowPosition {
    TopLeft,
    TopMiddle,
    TopRight,
    MiddleLeft,
    MiddleMiddle,
    MiddleRight,
    BottomLeft,
    BottomMiddle,
    BottomRight,
}

impl Window {
    pub fn add_container(&self, orientation: container::ContainerOrientation, child_spacing: u16) -> container::Container {
        let container = container::Container::new(orientation, child_spacing);
        self.window.add(&container.container);
        container
    }
    pub fn set_position(&self, position: WindowPosition, margin: i32) {
        let geometry = gdk::Display::default().unwrap()
            .primary_monitor().unwrap()
            .geometry();

        let window_width = self.window.allocated_width();
        let window_height = self.window.allocated_height();

        let (x, y) = match position {
            WindowPosition::TopLeft => (
                geometry.x() + margin,
                geometry.y() + margin,
            ),

            WindowPosition::TopMiddle => (
                geometry.x() + (geometry.width() - window_width) / 2,
                geometry.y() + margin,
            ),

            WindowPosition::TopRight => (
                geometry.x() + geometry.width() - window_width - margin,
                geometry.y() + margin,
            ),

            WindowPosition::MiddleLeft => (
                geometry.x() + margin,
                geometry.y() + (geometry.height() - window_height) / 2,
            ),

            WindowPosition::MiddleMiddle => (
                geometry.x() + (geometry.width() - window_width) / 2,
                geometry.y() + (geometry.height() - window_height) / 2,
            ),

            WindowPosition::MiddleRight => (
                geometry.x() + geometry.width() - window_width - margin,
                geometry.y() + (geometry.height() - window_height) / 2,
            ),

            WindowPosition::BottomLeft => (
                geometry.x() + margin,
                geometry.y() + geometry.height() - window_height - margin,
            ),

            WindowPosition::BottomMiddle => (
                geometry.x() + (geometry.width() - window_width) / 2,
                geometry.y() + geometry.height() - window_height - margin,
            ),

            WindowPosition::BottomRight => (
                geometry.x() + geometry.width() - window_width - margin,
                geometry.y() + geometry.height() - window_height - margin,
            ),
        };

        self.window.move_(x, y);
    }
    pub fn close(&self) {
        self.window.close();
    }
    pub fn set_title(&self, title: &str) -> &Self{
        self.window.set_title(title);
        self
    }
    pub fn show(&self) {
        self.window.show();
    }
    pub fn hide(&self) {
        self.window.hide();
    }
    pub fn set_default_size(&self, width: i32, height: i32) -> &Self {
        self.window.set_default_size(width, height);
        self
    }
    
    // basically since i uses bspwm i want ampersand to work with bspwm
    pub fn apply_behavior(&self) {
        use gtk::gdk::WindowTypeHint;
        
        match self.behavior {
            WindowBehavior::Tiled => {
                self.window.set_type_hint(WindowTypeHint::Normal);
                
                // i dont know how i feel about cloning the window
                let window_clone = self.window.clone();
                glib::timeout_add_local(
                    std::time::Duration::from_millis(100),
                    move || {
                        Self::make_tiled(&window_clone);
                        ControlFlow::Break
                    }
                );
            }
            WindowBehavior::Floating => {
                // i love how we cant tell bspwm to make the window tile we just give it a hint that it should be tiled
                self.window.set_type_hint(WindowTypeHint::Dialog);
            }
        }
    }
    fn make_tiled(window: &ApplicationWindow) {
        if let Ok(output) = Command::new("bspc")
            .args(&["query", "-N", "-n", "focused"])
            .output()
        {
            if output.status.success() {
                if let Ok(xid_str) = String::from_utf8(output.stdout) {
                    let xid = xid_str.trim();
                    if !xid.is_empty() {
                        let _ = Command::new("bspc")
                            .args(&["node", xid, "-t", "tiled"])
                            .output();
                    }
                }
            }
        }
    }

}

