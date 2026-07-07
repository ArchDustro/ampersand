use gtk::prelude::*;
use gtk::*;
use std::boxed::Box;

pub struct Image {
    pub image: gtk::Image,
    pub path: String,
}

impl Image {
    pub fn new_from_file(path: &str) -> Image {
        let image = gtk::Image::from_file(path);
        Image {
            image: image,
            path: path.to_string(),
        }
    }
    pub fn new_from_icon(icon: &str) -> Image {
        let image = gtk::Image::from_icon_name(Some(icon), IconSize::LargeToolbar);
        Image {
            image: image,
            path: icon.to_string(),
        }
    }
    pub fn new_from_bytes(bytes: &[u8]) -> Image {
        let pixbuf = gdk_pixbuf::Pixbuf::from_bytes(bytes);
        let image = gtk::Image::from_pixbuf(Some(&pixbuf));
        Image {
            image: image,
            path: "bytes".to_string(),
        }
    }
}