use crate::style::Font;

#[derive(Debug)]
pub enum Tag {
	// Structural frame with no semantic meaning
	Frame,

	Label {text: String, font: Font},

	Button {},
}

pub type Button = Tag;

impl Button {

}

pub type Label = Tag;

impl Label {
	pub fn new(text: String, font: Font) -> Label {
		Tag::Label {text: text, font: font}
	}
}
