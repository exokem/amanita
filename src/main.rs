use std::{ffi::CString, i32::MAX};

use raylib::{color::Color, ffi::{self, Vector2}, prelude::RaylibDraw, RaylibHandle};
use crate::{ray::context, vld::RenderContext};

mod ray;
mod vld;

#[derive(Debug)]
struct Font {
	size: i32,
}

#[derive(Debug)]
enum Tag {
    // Structural frame with no semantic meaning
    Frame,

    Label {text: String, font: Font},
}

#[derive(Debug)]
enum FlexDirection {
	Row,
	Col,
}

#[derive(Debug)]
enum Spacing {
	// Place an equal amount of space on either side of each element, without necessarily expanding each element
	// |--X----Y----Z--|
	Around,

	// Place an equal amount of space between each element, without necessarily expanding each element
	// |X------Y------Z|
	Between,

	// Place an equal amount of space between each element AND between first/last elements and the frame sides, without necessarily expanding each element
	// |---X---Y---Z---|
	Even,

	// Allow each element to expand evenly within the frame
	Stretch,

	None,
}

#[derive(Debug)]
enum Alignment {
	// |XYZ------------|
	Start,

	// |------XYZ------|
	Center,

	// |------------XYZ|
	End,
}

#[derive(Debug)]
enum Layout {
	Sequential,
	Flex {
		gap: usize,
		direction: FlexDirection,

		is_reverse: bool,

		// In the layout direciton
		axis_alignment: Alignment,
		// Perpendicular to the layout direction
		cross_alignment: Alignment,

		spacing: Spacing,
	},
	Grid {
		gap_row: usize,
		gap_col: usize,

		rows: usize,
		cols: usize,
	},
}

impl Layout {
	
}

#[derive(Debug)]
struct SidedSize {
    t: usize,
    r: usize,
    b: usize,
    l: usize,
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

#[derive(Debug)]
struct Border {
    color: Color,
    width: SidedSize,
}

impl Border {
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

type Padding = SidedSize;

impl Padding {
	pub fn internal_box(&self, b: &RenderBox) -> RenderBox {
        b.shrink(self.t as i32, self.r as i32, self.b as i32, self.l as i32)
	}
}

#[derive(Debug)]
struct Style {
    padding: Option<Padding>,
    border: Option<Border>,
}

// impl Tag {
//     pub fn render(&self, b: &RenderBox, context: &mut impl RenderContext, element: &Element) {

//         let mut area = *b;

//         if let Some(style) = &element.style {
//             if let Some(border) = &style.border {
//                 area = border.render(&area, context)
//             }

//             if let Some(padding) = &style.padding {
//                 area = padding.render(&area, context)
//             }
//         }        

//         // context.outline_rect(b.x, b.y, b.w, b.h, Color::RED);

//         if element.content.len() != 0 {
//             let item_areas = area.split_x(element.content.len(), 6);

//             for i in 0..element.content.len() {
//                 let area = &item_areas[i];
//                 let el = &element.content[i];

//                 // context.outline_rect(area.x, area.y, area.w, area.h, Color::BLACK);

//                 el.tag.render(area, context, &el);
//             }
//         }
        

//         match self {
//             Self::Frame => {
                
//             },
//             Self::Button => {

//             },
//             Self::Label { text, font } => {

//             }
//         }
//     }
// }

#[derive(Debug, Clone, Copy)]
struct RenderBox {
    x: i32,
    y: i32,
    w: i32,
    h: i32,
}

struct Vec2i {
	x: i32,
	y: i32,
}

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

#[derive(Debug)]
struct Element {
    tag: Tag,
    content: Vec<Element>,
    style: Option<Style>,
	layout: Layout,
}

impl Element {
	pub fn measure(&self, b: &RenderBox, context: &impl RenderContext) -> Vec2i {
		let mut b = *b;
		let mut width: i32 = 0;
		let mut height: i32 = 0;

		// Apply padding & border to get content area
        if let Some(style) = &self.style {
            if let Some(border) = &style.border {
				width += (border.width.l + border.width.r) as i32;
				height += (border.width.t + border.width.b) as i32;
				b = border.internal_box(&b);
            }

            if let Some(padding) = &style.padding {
				width += (padding.l + padding.r) as i32;
				height += (padding.t + padding.b) as i32;
				b = padding.internal_box(&b);
            }
        }

		match &self.tag {
			Tag::Frame => {
				match &self.layout {
					Layout::Sequential => {
						let mut w = 0;
						for el in &self.content {
							let size = el.measure(&b, context);
							height += size.y;
							w = w.max(size.x);
						}

						return Vec2i {x: width + w, y: height}
					},
					Layout::Flex { gap, direction, is_reverse, axis_alignment, cross_alignment, spacing } => {
						match direction {
							FlexDirection::Row => {
								let mut w = (gap * (self.content.len() - 1)) as i32;
								let mut h = 0;
								for el in &self.content {
									let size = el.measure(&b, context);
									w += size.x;
									h = h.max(size.y);
								}

								Vec2i {x: width + w, y: height + h}
							},
							FlexDirection::Col => {
								let mut w = 0;
								let mut h = (gap * (self.content.len() - 1)) as i32;
								for el in &self.content {
									let size = el.measure(&b, context);
									h += size.y;
									w = w.max(size.x);
								}

								Vec2i {x: width + w, y: height + h}
							},
						}
					},
					Layout::Grid { gap_row, gap_col, rows, cols } => todo!(),
				}
			},
			Tag::Label { text, font } => {
				let (content_width, content_height) = context.calculate_text_size(text, font.size);

				Vec2i {x: width + content_width, y: height + content_height}
			},
		}
	}

	pub fn render(&self, b: &RenderBox, context: &mut impl RenderContext) {
		let mut b = *b;

		let min_size = self.measure(&b, context);
		b = RenderBox { x: b.x, y: b.y, w: min_size.x, h: min_size.y };
		context.outline_rect(b.x, b.y, min_size.x, min_size.y, Color::RED);

		// Apply padding & border to get content area
        if let Some(style) = &self.style {
            if let Some(border) = &style.border {
				// border.render(&b, context);
				b = border.internal_box(&b);
            }

            if let Some(padding) = &style.padding {
                b = padding.internal_box(&b);
            }
        }

		match &self.tag {
			Tag::Frame => {
				match &self.layout {
					Layout::Sequential => {
						let mut y = b.y;
						for el in &self.content {
							let size = el.measure(&b, context);
							let el_rb = RenderBox {x: b.x, y: y, w: size.x, h: size.y};
							el.render(&el_rb, context);
							context.outline_rect(el_rb.x, el_rb.y, el_rb.w, el_rb.h, Color::REBECCAPURPLE);
							y += size.y;
						}
					},
					Layout::Flex { gap, direction, is_reverse, axis_alignment, cross_alignment, spacing } => {
						match direction {
							FlexDirection::Row => {
								let mut x = b.x;
								for el in &self.content {
									let size = el.measure(&b, context);
									let el_rb = RenderBox {x: x, y: b.y, w: size.x, h: size.y};
									el.render(&el_rb, context);

									x += size.x + (*gap as i32);
								}
							},
							FlexDirection::Col => {
								let mut y = b.y;
								for el in &self.content {
									let size = el.measure(&b, context);
									let el_rb = RenderBox {x: b.x, y: y, w: size.x, h: size.y};
									el.render(&el_rb, context);

									y += size.y + (*gap as i32);
								}
							},
						}
					},
					Layout::Grid { gap_row, gap_col, rows, cols } => todo!(),
				}
			},
			Tag::Label { text, font } => {
				let (content_width, content_height) = context.calculate_text_size(text, font.size);

				context.fill_text(b.x, b.y, &text, font.size, Color::BLACK);
				// context.outline_rect(area.x, area.y, content_width, content_height, Color::RED);
			},
		}

		// Defer to layout for content arrangement
		// match &self.layout {
		// 	Layout::Flex { gap, direction, is_reverse, axis_alignment, cross_alignment, spacing } => {

		// 		match direction {
		// 			FlexDirection::Row => {

		// 				match axis_alignment {
		// 					Alignment::Start => {

		// 					},
		// 					Alignment::Center => todo!(),
		// 					Alignment::End => todo!(),
		// 				}
		// 			},
		// 			FlexDirection::Col => todo!(),
		// 		}
		// 	},
		// 	Layout::Grid { gap_row, gap_col, rows, cols } => todo!(),
		// }
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
		layout: Layout::Flex { gap: 6, direction: FlexDirection::Row, is_reverse: false, axis_alignment: Alignment::Start, cross_alignment: Alignment::Start, spacing: Spacing::None },
        content: vec![
            Element {
                tag: Tag::Label {
                    text: String::from("test label"),
					font: Font { size: 32 },
                },
				layout: Layout::Sequential,
                content: vec![],
                style: Some(Style {
                    padding: Some(Padding::new_uniform(3)),
                    border: Some(Border { color: Color::ORANGE, width: SidedSize::new_uniform(3) })
                }),
            },
            Element {
                tag: Tag::Label {
                    text: String::from("test label"),
					font: Font { size: 32 },
                },
				layout: Layout::Sequential,
                content: vec![],
                style: Some(Style {
                    padding: Some(Padding::new_uniform(3)),
                    border: Some(Border { color: Color::ORANGE, width: SidedSize::new_uniform(3) })
                }),
            },
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
        style: Some(Style {
            padding: Some(Padding::new_uniform(3)),
            border: Some(Border { color: Color::GREEN, width: SidedSize::new_uniform(3) })
        }),
    };

    while !rl.window_should_close() {
        let w = rl.get_screen_width();
        let h = rl.get_screen_height();
        let mut d = rl.begin_drawing(&thread);
        {
            d.clear_background(Color::RAYWHITE);
            let rb = RenderBox {x: 10, y: 10, w: w - 20, h: h - 20};

            ui.render(&rb, &mut d);
        }
    }
}
