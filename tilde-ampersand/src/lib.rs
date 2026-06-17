use gtk::prelude::*;
use gtk::{Application, ApplicationWindow,Label, Button,pango,Box,gdk, CssProvider, StyleContext};
use glib::clone;
use glib::ControlFlow;

pub fn create_window(
        title: &str, 
        borderless: bool, s
        how_in_taskbar: bool, 
        focus: bool,
        resizable: bool,
    ) -> ApplicationWindow {
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
