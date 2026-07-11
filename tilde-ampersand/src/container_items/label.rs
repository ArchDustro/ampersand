use gtk::prelude::*;
use gtk::*;

// I don't know if I should be doing this Copy/Clone thing but its useful to me and I don't know of a better alternative.
#[derive(Copy, Clone)]
pub enum WrapMode {
    Word,
    Char,
    WordChar,
    None
}

#[derive(Copy, Clone)]
pub enum LabelDirection {
    RightToLeft,
    LeftToRight,
    None,
}
pub enum Alignment {
    // haha im australian so i spell it 'centre'
    Centre,
    End,
    Start
}

#[derive(Clone)]
pub struct Label {
    pub label: gtk::Label,
    pub wrap_mode: WrapMode,
}

pub struct LabelLayout {
    expand: bool,
}

impl Label {
    // We call this function to create or get a new label.
    pub fn new(text: &str, wrap_mode: WrapMode, num_chars_per_line: i32) -> Label {
        let label = gtk::Label::new(Some(text));

        label.set_max_width_chars(num_chars_per_line);

        match wrap_mode {
            WrapMode::Word => {
                label.set_line_wrap(true);
                label.set_line_wrap_mode(pango::WrapMode::Word);
            }
            WrapMode::Char => {
                label.set_line_wrap(true);
                label.set_line_wrap_mode(pango::WrapMode::Char);
            }
            WrapMode::WordChar => {
                label.set_line_wrap(true);
                label.set_line_wrap_mode(pango::WrapMode::WordChar);
            }
            WrapMode::None => {
                label.set_line_wrap(false);
            }
        }

        Label {
            label: label,
            wrap_mode: wrap_mode,
        }
    }

    // This sets the line wrap of the text on the labels.
    pub fn set_line_wrap(&mut self, wrap_mode: WrapMode, chars_per_line: Option<i32>) {
        if let Some(chars) = chars_per_line {
            self.label.set_max_width_chars(chars);
        }
        match wrap_mode {
            WrapMode::Word => {
                self.label.set_line_wrap(true);
                self.label.set_line_wrap_mode(pango::WrapMode::Word);
            }
            WrapMode::Char => {
                self.label.set_line_wrap(true);
                self.label.set_line_wrap_mode(pango::WrapMode::Char);
            }
            WrapMode::WordChar => {
                self.label.set_line_wrap(true);
                self.label.set_line_wrap_mode(pango::WrapMode::WordChar);
            }
            WrapMode::None => {
                self.label.set_line_wrap(false);
            }
        }
        self.wrap_mode = wrap_mode;
    }

    // Set or change the text on the label
    pub fn set_text(&self, text: &str) {
        self.label.set_text(text);
    }

    // For different languages, glyphs may be drawn in different directions.
    pub fn set_direction(&mut self, direction: LabelDirection) {
        match direction {
            LabelDirection::LeftToRight => self.label.set_direction(TextDirection::Ltr),
            LabelDirection::RightToLeft => self.label.set_direction(TextDirection::Rtl),

            LabelDirection::None => self.label.set_direction(TextDirection::None),
        }
    }

    pub fn set_alignment(&self, alignment: Alignment) {
        match alignment {
            Alignment::Centre => {
                self.label.set_justify(gtk::Justification::Center);
                self.label.set_xalign(0.5);
            }
            Alignment::Start => {
                self.label.set_justify(gtk::Justification::Left);
                self.label.set_xalign(0.0);
            }
            Alignment::End => {
                self.label.set_justify(gtk::Justification::Right);
                self.label.set_xalign(1.0);
            }
        }
    }
}
