use gtk::prelude::*;
use gtk::{Label,pango,TextDirection};

// I don't know if I should be doing this Copy/Clone thing but its useful to me and I don't know of a better alternative.
#[derive(Copy, Clone)]
pub enum AmpersandWrapMode {
    Word,
    Char,
    WordChar,
    None
}

#[derive(Copy, Clone)]
pub enum AmpersandLabelDirection {
    RightToLeft,
    LeftToRight,
    None,
}

#[derive(Clone)]
pub struct AmpersandLabel {
    label: Label,
    wrap_mode: AmpersandWrapMode,
}

impl AmpersandLabel {
    pub fn new(text: &str, wrap_mode: AmpersandWrapMode, num_chars_per_line: i32) -> AmpersandLabel {
        let label = Label::new(Some(text));

        label.set_max_width_chars(num_chars_per_line);

        match wrap_mode {
            AmpersandWrapMode::Word => {
                label.set_line_wrap(true);
                label.set_line_wrap_mode(pango::WrapMode::Word);
            }
            AmpersandWrapMode::Char => {
                label.set_line_wrap(true);
                label.set_line_wrap_mode(pango::WrapMode::Char);
            }
            AmpersandWrapMode::WordChar => {
                label.set_line_wrap(true);
                label.set_line_wrap_mode(pango::WrapMode::WordChar);
            }
            AmpersandWrapMode::None => {
                label.set_line_wrap(false);
            }
        }

        AmpersandLabel {
            label: label,
            wrap_mode: wrap_mode,
        }
    }
    pub fn set_line_wrap(&mut self, wrap_mode: AmpersandWrapMode) {
        match wrap_mode {
            AmpersandWrapMode::Word => {
                self.label.set_line_wrap(true);
                self.label.set_line_wrap_mode(pango::WrapMode::Word);
            }
            AmpersandWrapMode::Char => {
                self.label.set_line_wrap(true);
                self.label.set_line_wrap_mode(pango::WrapMode::Char);
            }
            AmpersandWrapMode::WordChar => {
                self.label.set_line_wrap(true);
                self.label.set_line_wrap_mode(pango::WrapMode::WordChar);
            }
            AmpersandWrapMode::None => {
                self.label.set_line_wrap(false);
            }
        }
        self.wrap_mode = wrap_mode;
    }

    // Set or change the text on the label
    pub fn set_text(&mut self, text: &str) {
        self.label.set_text(text);
    }

    // For different languages, glyphs may be drawn in different directions.
    pub fn set_direction(&mut self, direction: AmpersandLabelDirection) {
        match direction {
            AmpersandLabelDirection::LeftToRight => self.label.set_direction(TextDirection::Ltr),
            AmpersandLabelDirection::RightToLeft => self.label.set_direction(TextDirection::Rtl),

            AmpersandLabelDirection::None => self.label.set_direction(TextDirection::None),
        }
    }

}
