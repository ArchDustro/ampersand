use gdk::traits::MonitorExt;
use gtk::{ApplicationWindow, traits::{ContainerExt, GtkWindowExt, WidgetExt}};
use crate::container;
use gtk::gdk;

pub struct Window {
    pub window: ApplicationWindow,
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
    pub fn add_container(&self, orientation: container::ContainerOrientation, child_spacing: u16) {
        let container = container::Container::new(orientation, child_spacing);
        self.window.add(&container.container);
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
}

