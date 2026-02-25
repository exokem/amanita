
use raylib::{color::Color, ffi::{BeginTextureMode, Rectangle, RenderTexture}, math::Vector2, prelude::{RaylibDraw, RaylibScissorModeExt, RaylibTextureModeExt}, texture::{RaylibRenderTexture2D, RenderTexture2D}};

mod ray;

#[allow(unused)]
mod vld;

mod vec2;

#[allow(unused)]
mod style;

#[allow(unused)]
mod tag;

#[allow(unused)]
mod layout;

#[allow(unused)]
mod element;

use crate::{element::Element, layout::{Alignment, Flex, Layout, Spacing}, ray::context::{RayInputContext, RayViewContext}, style::{Border, Font, Padding, SidedSize, Style}, tag::Tag, vec2::Vec2i, vld::{InputContext, RenderContext}};
use crate::element::{ElementSet, ElementState};
use crate::vld::MeasureContext;

#[derive(Debug, Clone, Copy)]
struct RenderBox {
    x: i32,
    y: i32,
    w: i32,
    h: i32,
}

impl RenderBox {
	pub fn zero() -> Self {
		RenderBox {x: 0, y: 0, w: 0, h:0}
	}
}

#[allow(unused)]
impl RenderBox {
    pub fn shrink(&self, t: i32, r: i32, b: i32, l: i32) -> RenderBox {
        RenderBox { x: self.x + l, y: self.y + t, w: self.w - (l + r), h: self.h - (t + b) }
    }

    pub fn split_x(&self, cols: usize, gap: i32) -> Vec<RenderBox> {
        let mut vec = Vec::with_capacity(cols);
        let cols = cols as i32;

        let w = (self.w - (gap * (cols - 1))) / cols;

        let mut x = self.x;

        for _ in 0..cols {
            vec.push(RenderBox { x: x, y: self.y, w: w, h: self.h });
            x += gap + w;
        }

        vec
    }

	pub fn size(&self) -> Vec2i {
		Vec2i {x: self.w, y: self.h}
	}

	pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
		Self {
			x, y, w, h
		}
	}
}

fn main() {
	let screen_w = 800;
	let screen_h = 450;

	let mut ray = RayViewContext::new("amanita", screen_w, screen_h);
	let input = RayInputContext {};

	let mut set = ElementSet::new(|s| {
		s.new_frame(
			Flex::row().item_gap(6)
				.axis_align(Alignment::Center)
				.spacing(Spacing::Between)
				.build(),
			Style::default().padding(Padding::new_uniform(3)).min_size(600, 200),
			|s| {
				vec![
					s.new_label(String::from("one"), Font {size: 32}, Style::default().padding(Padding::new_uniform(3))),
					s.new_scroll_frame(
						Style::default().padding(Padding::new_uniform(3)).max_size(300, 400),
						|s| {
							s.new_frame(
								Flex::col().item_gap(6)
									.axis_align(Alignment::Center)
									.cross_align(Alignment::Center)
									.spacing(Spacing::Between).build(),
								Style::default().padding(Padding::new_uniform(3)),
								|s| {
									vec![
										s.new_label(String::from("two thousand"), Font { size: 48 }, Style::default().padding(Padding::new_uniform(3)).background(Color::YELLOW)),
										s.new_label(String::from("balls"), Font { size: 48 }, Style::default().padding(Padding::new_uniform(3)).background(Color::ORANGE)),
										s.new_label(String::from("two"), Font { size: 48 }, Style::default().padding(Padding::new_uniform(3)).background(Color::RED)),
									]
								}
							)
						}
					),
				]
			},
		)
	});

	set.sort();

    while !ray.handle.window_should_close() {
        let w = ray.handle.get_screen_width();
        let h = ray.handle.get_screen_height();
        let mut d = ray.handle.begin_drawing(&ray.thread);
        {
            d.clear_background(Color::RAYWHITE);
            let _ = RenderBox {x: 10, y: 10, w: w - 20, h: h - 20};

			set.process_frame(None, &input, &d.get_font_default(), &mut d);

			// Element::render(&ui.root, None, &input, &d.get_font_default(), &mut d, );
			// Element::update(&mut ui.root, &input, &d.get_font_default());
        }
    }
}
