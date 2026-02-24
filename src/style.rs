use raylib::color::Color;

use crate::{vld::RenderContext, RenderBox};

#[derive(Debug)]
pub struct Font {
	pub size: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct SidedSize {
	pub t: usize,
	pub r: usize,
	pub b: usize,
	pub l: usize,
}

impl SidedSize {
	pub fn new(top: usize, right: usize, bottom: usize, left: usize) -> SidedSize {
		SidedSize { t: top, r: right, b: bottom, l: left }
	}

	pub fn new_uniform(size: usize) -> SidedSize {
		SidedSize { t: size, r: size, b: size, l: size }
	}

	pub fn new_symmetrical(vertical: usize, horizontal: usize) -> SidedSize {
		SidedSize { t: vertical, r: horizontal, b: vertical, l: horizontal }
	}
}

#[derive(Debug, Clone, Copy)]
pub struct Border {
	pub color: Color,
	pub width: SidedSize,
}

impl Border {
	pub fn none() -> Self {
		Self { color: Color::WHITE.alpha(0.0), width: SidedSize::new_uniform(0) }
	}

	pub fn internal_box(&self, b: &RenderBox) -> RenderBox {
		b.shrink(self.width.t as i32, self.width.r as i32, self.width.b as i32, self.width.l as i32)
	}

	pub fn render(&self, b: &RenderBox, context: &mut impl RenderContext) {

		// Left
		context.fill_rect(b.x, b.y, self.width.l as i32, b.h, self.color);
		// Top
		context.fill_rect(b.x, b.y, b.w, self.width.t as i32, self.color);
		// Right
		context.fill_rect(b.x + b.w - self.width.r as i32, b.y, self.width.r as i32, b.h, self.color);
		// Bottom
		context.fill_rect(b.x, b.y + b.h - self.width.b as i32, b.w, self.width.b as i32, self.color);
	}
}

pub type Padding = SidedSize;

impl Padding {
	pub fn internal_box(&self, b: &RenderBox) -> RenderBox {
		b.shrink(self.t as i32, self.r as i32, self.b as i32, self.l as i32)
	}
}

#[derive(Debug)]
pub struct Style {
	pub padding: Option<Padding>,
	pub border: Option<Border>,

	pub background: Option<Color>,

	pub min_width: i32,
	pub max_width: Option<i32>,

	pub min_height: i32,
	pub max_height: Option<i32>,
}
