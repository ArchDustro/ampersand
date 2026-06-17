use gtk::prelude::*;
use gtk::{Label,pango,TextDirection};

enum AmpersandWrapMode {
    Word,
    Char,
    WordChar,
    None
}
enum AmpersandLabelDirection {
    RightToLeft,
    LeftToRight,
    None,
}

struct AmpersandLabel {
    label: Label,
    wrap_mode: AmpersandWrapMode,
}

impl AmpersandLabel {
    fn new(text: &str, wrap_mode: AmpersandWrapMode, num_chars_per_line: i32) -> AmpersandLabel {
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
    fn set_line_wrap(&mut self, wrap_mode: AmpersandWrapMode) {
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
    fn set_text(&mut self, text: &str) {
        self.label.set_text(text);
    }
    fn set_direction(&mut self, direction: AmpersandLabelDirection) {
        match direction {
            AmpersandLabelDirection::LeftToRight => self.label.set_direction(TextDirection::Ltr),
            AmpersandLabelDirection::RightToLeft => self.label.set_direction(TextDirection::Rtl),

            AmpersandLabelDirection::None => self.label.set_direction(TextDirection::None),
        }
    }

}
