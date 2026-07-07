use gtk::prelude::*;
use gtk::*;
use std::boxed::Box;

pub struct Button {
    pub button: gtk::Button,
    pub on_click: Box<dyn Fn()>,
}

pub enum Position {
    Left,
    Right,
    Top,
    Bottom,
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

    pub fn clicked(&mut self) {
        let callback = std::mem::replace(&mut self.on_click, Box::new(|| {}));
        self.button.connect_clicked(move |_| {
            callback();
        });
    }
    pub fn set_label(&self, new_label: &str) -> &Self {
        self.button.set_label(new_label);
        self
    }
    pub fn set_image(&self, path: &str) -> &Self {
        let image = gtk::Image::from_file(path);
        self.button.set_image(Some(&image));
        self
    }
    fn set_image_position(&self, position: Position) -> &Self {
        let position_type = match position {
            Position::Left => gtk::PositionType::Left,
            Position::Right => gtk::PositionType::Right,
            Position::Top => gtk::PositionType::Top,
            Position::Bottom => gtk::PositionType::Bottom,
        };
        self.button.set_image_position(position_type);
        self
    }
    fn add_widget(&self, widget: &impl IsA<gtk::Widget>) -> &Self {
        self.button.add(widget);
        self
    }
}