use gtk::prelude::*;
use gtk::{Box,Orientation};
use crate::box_items::*;

enum AmpContainerOrientation {
    Vertical,
    Horizontal,
}

struct AmpContainer {
    container: Box,
    orientation: AmpContainerOrientation,
}

impl AmpContainer {
    fn new(orientation: AmpContainerOrientation, child_spacing: u16) -> AmpContainer {
        AmpContainer {
            container: Box::new(match orientation {
                AmpContainerOrientation::Vertical => Orientation::Vertical,
                AmpContainerOrientation::Horizontal => Orientation::Horizontal
            }, child_spacing.into()),
            orientation: orientation,
        }
    }
    fn get_orientation(&self) -> AmpContainerOrientation {
        self.orientation
    }
    fn pad(&self, north: u16, east: u16, south: u16, west: u16) {
        self.container.set_margin_top(north.into());
        self.container.set_margin_end(east.into());
        self.container.set_margin_bottom(south.into());
        self.container.set_margin_start(west.into());
    }
    fn add_label(&self, text: &str) {
        
    }
}
