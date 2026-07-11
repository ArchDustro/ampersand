use gtk::prelude::*;
use gtk::{Box,Orientation};

// I obviously don't really understand how importing works in Rust.
// This just looks weird af to me.
use crate::container_items::*;
use label::*;
use button::*;

// Now I'll tell you now,
// out of everything I have coded or done coding for,
// there is nothing worse than error handling.
// I fucking hate error handling.

#[derive(Copy, Clone)]
pub enum ContainerOrientation {
    Vertical,
    Horizontal,
}

pub enum PackOrder {
    Start,
    End,
}

#[derive(Clone)]
pub struct Container {
    pub container: Box,
    pub orientation: ContainerOrientation,
}

impl Container {
    pub fn new(orientation: ContainerOrientation, child_spacing: u16) -> Container {
        Container {
            container: Box::new(match orientation {
                ContainerOrientation::Vertical => Orientation::Vertical,
                ContainerOrientation::Horizontal => Orientation::Horizontal
            }, child_spacing.into()),
            orientation: orientation,
        }
    }

    // Set/get the spacing between the children.
    pub fn set_spacing(&self, spacing_px: u16){
        self.container.set_spacing(spacing_px.into());
    }
    pub fn get_spacing(&self) -> i32{
        self.container.spacing()
    }

    // Get the container orientation.
    pub fn get_orientation(&self) -> ContainerOrientation {
        self.orientation
    }

    // Pads children from the sides of the box.
    pub fn pad(&self, north: u16, east: u16, south: u16, west: u16) {
        self.container.set_margin_top(north.into());
        self.container.set_margin_end(east.into());
        self.container.set_margin_bottom(south.into());
        self.container.set_margin_start(west.into());
    }

    // Adds a text label.
    pub fn add_label(
            &self, 
            text: &str, 
            wrap_mode: WrapMode, 
            chars_per_line: i32, 
            padding_px: u32,
        ) -> label::Label{
        let label = Label::new(text, wrap_mode, chars_per_line);
        self.container.pack_start(&label.label, true, true, padding_px);
        label
    }

    // Adds a interactable button
    pub fn add_buttom(&self,
        text: &str,
        on_click: std::boxed::Box<dyn Fn()>,
        padding_px: u32) -> button::Button {
            let button = Button::new(text, on_click);
            self.container.pack_start(&button.button, true, true, padding_px);
            button
        }
}
