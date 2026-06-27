use gtk::builders::*;
use gtk::prelude::*;
use gtk::{Application,Box,Orientation};

use crate::box_items::*;
use label::*;

// Now I'll tell you now,
// out of everything I have coded or done coding for,
// there is nothing worse than error handling.
// I fucking hate error handling.

enum AmpersandBoxOrientation {
    Vertical,
    Horizontal,
}

struct AmpersandAppBuilder {
    builder: ApplicationBuilder,
    app_id: String,
}

struct AmpersandBox {
    a_box: Box,
    orientation: AmpersandBoxOrientation,
}

struct AmpersandApp {
    application: Application,
}

impl AmpersandAppBuilder {
    fn new(app_id: &str) -> AmpersandAppBuilder {
        AmpersandAppBuilder {
            builder: Application::builder().application_id(app_id),
            app_id: app_id.to_string()
        }
    }
    fn build(self) -> AmpersandApp {
        AmpersandApp {
            application: self.builder.build()
        }
    }
}

impl AmpersandBox {
    fn new(orientation: AmpersandBoxOrientation, child_spacing: u16) -> AmpersandBox {
        AmpersandBox {
            a_box: Box::new(match orientation {
                AmpersandBoxOrientation::Vertical => Orientation::Vertical,
                AmpersandBoxOrientation::Horizontal => Orientation::Horizontal
            }, child_spacing.into()),
            orientation: orientation,
        }
    }
    fn get_orientation(self) -> AmpersandBoxOrientation {
        self.orientation
    }
    fn pad(self, north: u16, east: u16, south: u16, west: u16) {
        self.a_box.set_margin_top(north.into());
        self.a_box.set_margin_end(east.into());
        self.a_box.set_margin_bottom(south.into());
        self.a_box.set_margin_start(west.into());
    }
    fn add_label(self, text: &str, wrap_mode: AmpersandWrapMode, chars_per_line: i32) -> Result<AmpersandLabel, Error> {
        let label = AmpersandLabel::new(text, wrap_mode, chars_per_line);

        Ok(label)
    }
}

impl AmpersandApp {
    fn add_box() {
        println!("Howdy")
    }
}