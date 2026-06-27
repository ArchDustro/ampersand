use gtk::prelude::*;
use gtk::{Box,Orientation};

// I obviously don't really understand how importing works in Rust.
// This just looks weird af to me.
use crate::box_items::*;
use label::*;

// Now I'll tell you now,
// out of everything I have coded or done coding for,
// there is nothing worse than error handling.
// I fucking hate error handling.
use crate::Error;

#[derive(Copy, Clone)]
enum ContainerOrientation {
    Vertical,
    Horizontal,
}

#[derive( Clone)]
struct Container {
    container: Box,
    orientation: ContainerOrientation,
}

impl Container {
    fn new(orientation: ContainerOrientation, child_spacing: u16) -> Container {
        Container {
            container: Box::new(match orientation {
                ContainerOrientation::Vertical => Orientation::Vertical,
                ContainerOrientation::Horizontal => Orientation::Horizontal
            }, child_spacing.into()),
            orientation: orientation,
        }
    }
    fn get_orientation(&self) -> ContainerOrientation {
        self.orientation
    }
    fn pad(&self, north: u16, east: u16, south: u16, west: u16) {
        self.container.set_margin_top(north.into());
        self.container.set_margin_end(east.into());
        self.container.set_margin_bottom(south.into());
        self.container.set_margin_start(west.into());
    }
    fn add_label(self, text: &str, wrap_mode: AmpersandWrapMode, chars_per_line: i32) -> Result<AmpersandLabel, Error> {
        let label = AmpersandLabel::new(text, wrap_mode, chars_per_line);
        Ok(label)
    }
}
