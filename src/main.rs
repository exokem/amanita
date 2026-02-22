
use raylib::{color::Color, prelude::RaylibDraw};

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

use crate::{element::Element, layout::{Alignment, Flex, Layout, Spacing}, style::{Border, Font, Padding, SidedSize, Style}, tag::Tag, vec2::Vec2i};

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
}


// pub fn measure_text(text: &str, font_size: i32) -> Vector2 {
// 	let c_text = CString::new(text).unwrap();
// 	unsafe { ffi::MeasureTextEx(ffi::GetFontDefault(), c_text.as_ptr(), font_size as f32, 0.0) }
// }

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(800, 450)
        .title("cascading-oxide")
        .resizable()
        .vsync()
        .build();

    rl.set_target_fps(60);

    let ui = Element {
        tag: Tag::Frame,
		// layout: Layout::Sequential,
		layout: Flex::row()
			.item_gap(6)
			.axis_align(Alignment::Center)
			.spacing(Spacing::Between)
			.build(),
        elements: vec![
            Element {
                tag: Tag::Label {
                    text: String::from("one"),
					font: Font { size: 32 },
                },
				layout: Layout::Sequential,
                elements: vec![],
                style: Style {
                    padding: Some(Padding::new_uniform(3)),
                    border: Some(Border { color: Color::ORANGE, width: SidedSize::new_uniform(3) }),
					min_width: 0,
					min_height: 0,
                },
            },
            Element {
                tag: Tag::Label {
                    text: String::from("two"),
					font: Font { size: 16 },
                },
				layout: Layout::Sequential,
                elements: vec![],
                style: Style {
                    padding: Some(Padding::new_uniform(3)),
                    border: Some(Border { color: Color::ORANGE, width: SidedSize::new_uniform(3) }),
					min_width: 0,
					min_height: 0,
                },
            },
			Element {
				tag: Tag::Frame,
				layout: Layout::Sequential,
				// layout: Layout::Flex { gap: 6, direction: FlexDirection::Row, is_reverse: false, axis_alignment: Alignment::Start, cross_alignment: Alignment::End, spacing: Spacing::None },
				elements: vec![
					Element {
						tag: Tag::Label {
							text: String::from("three"),
							font: Font { size: 24 },
						},
						layout: Layout::Sequential,
						elements: vec![],
						style: Style {
							padding: Some(Padding::new_uniform(3)),
							border: Some(Border { color: Color::ORANGE, width: SidedSize::new_uniform(3) }),
							min_width: 0,
							min_height: 0,
						},
					},
				],
				style: Style {
					padding: Some(Padding::new_uniform(3)),
					border: Some(Border { color: Color::GREEN, width: SidedSize::new_uniform(3) }),
					min_width: 0,
					min_height: 0,
				},
			}
            // Element {
            //     tag: Tag::Button,
            //     content: vec![

            //     ],
            //     style: Some(Style {
            //         padding: Some(Padding::new_uniform(3)),
            //         border: Some(Border { color: Color::RED, width: SidedSize::new_uniform(1) })
            //     }),
            // }
        ],
        style: Style {
            padding: Some(Padding::new_uniform(3)),
            border: Some(Border { color: Color::GREEN, width: SidedSize::new_uniform(3) }),
			min_width: 400,
			min_height: 200,
        },
    };

    while !rl.window_should_close() {
        let w = rl.get_screen_width();
        let h = rl.get_screen_height();
        let mut d = rl.begin_drawing(&thread);
        {
            d.clear_background(Color::RAYWHITE);
            let _ = RenderBox {x: 10, y: 10, w: w - 20, h: h - 20};

			Element::render(&ui, &mut d);
        }
    }
}
