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

#[derive(Debug, Clone, Copy)]
pub struct Style {
	pub padding: Option<Padding>,
	pub border: Option<Border>,

	pub background: Option<Color>,

	pub min_width: i32,
	pub max_width: Option<i32>,

	pub min_height: i32,
	pub max_height: Option<i32>,
}

impl Style {
	pub fn padding(mut self, padding: Padding) -> Self {
		self.padding = Some(padding);
		self
	}

	pub fn border(mut self, border: Border) -> Self {
		self.border = Some(border);
		self
	}

	pub fn background(mut self, background: Color) -> Self {
		self.background = Some(background);
		self
	}

	pub fn min_width(mut self, min_width: i32) -> Self {
		self.min_width = min_width;
		self
	}

	pub fn max_width(mut self, max_width: i32) -> Self {
		self.max_width = Some(max_width);
		self
	}

	pub fn min_height(mut self, min_height: i32) -> Self {
		self.min_height = min_height;
		self
	}

	pub fn max_height(mut self, max_height: i32) -> Self {
		self.max_height = Some(max_height);
		self
	}

	pub fn min_size(mut self, width: i32, height: i32) -> Self {
		self.min_width = width;
		self.min_height = height;
		self
	}

	pub fn max_size(mut self, width: i32, height: i32) -> Self {
		self.max_width = Some(width);
		self.max_height = Some(height);
		self
	}
}

impl Default for Style {
	fn default() -> Self {
		Self {
			padding: None,
			border: None,
			background: None,

			min_width: 0,
			max_width: None,

			min_height: 0,
			max_height: None,
		}
	}
}
