use std::{collections::VecDeque, ffi::CString, i32::MAX, ops::{Add, Sub}};

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

	min_width: i32,
	min_height: i32,
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

impl RenderBox {
	pub fn zero() -> Self {
		RenderBox {x: 0, y: 0, w: 0, h:0}
	}
}

#[derive(Debug, Clone, Copy)]
struct Vec2i {
	x: i32,
	y: i32,
}

impl Add for Vec2i {
	type Output = Vec2i;

	fn add(self, rhs: Self) -> Self::Output {
		Vec2i::new(self.x + rhs.x, self.y + rhs.y)
	}
}

impl Sub for Vec2i {
	type Output = Vec2i;
	
	fn sub(self, rhs: Self) -> Self::Output {
		Vec2i::new(self.x - rhs.x, self.y - rhs.y)
	}
}

impl Into<Vec2i> for (i32, i32) {
	fn into(self) -> Vec2i {
		Vec2i::new(self.0, self.1)
	}
}

impl Vec2i {
	pub fn new(x: i32, y: i32) -> Self {
		Self {x: x, y: y}
	}

	pub fn zero() -> Self {
		Self {x: 0, y: 0}
	}

	pub fn max(self, other: Self) -> Self {
		Self {x: self.x.max(other.x), y: self.y.max(other.y)}
	}

	pub fn min(self, other: Self) -> Self {
		Self {x: self.x.min(other.x), y: self.y.min(other.y)}
	}
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
    elements: Vec<Element>,
    style: Option<Style>,
	layout: Layout,
}

struct MeasuredElement<'a> {
	// Exact render area of the element
	element: &'a Element,
	rb: RenderBox,
}

struct RenderRef {
	render_index: usize,
	element_render_indices: Option<Vec<usize>>
}

impl Element {
	// Breadth-first traversal to determine render order of all elements in hierarchy
	pub fn render_order(root: &Element) -> Vec<&Element> {
		let mut elements: Vec<&Element> = vec![];
		let mut queue = VecDeque::from([root]);

		while queue.len() != 0 && let Some(next) = queue.pop_front() {
			elements.push(next);

			// Add contents to queue
			for el in &next.elements {
				queue.push_back(&el);
			}
		}

		elements
	}

	pub fn measure_element(element: &Element, sub_element_boxes: &Vec<RenderBox>, context: &impl RenderContext) -> RenderBox {
		// Border + padding
		let mut frame_size = Vec2i::zero();
		let mut min_size = Vec2i::zero();

		if let Some(style) = &element.style {
			if let Some(border) = &style.border {
				frame_size.x += (border.width.l + border.width.r) as i32;
				frame_size.y += (border.width.t + border.width.b) as i32;
            }

            if let Some(padding) = &style.padding {
				frame_size.x += (padding.l + padding.r) as i32;
				frame_size.y += (padding.t + padding.b) as i32;
            }

			min_size.x = style.min_width;
			min_size.y = style.min_height;
		}

		let content_size = match &element.tag {
			Tag::Frame => {
				match &element.layout {
					Layout::Sequential => {
						let mut w = 0;
						let mut h = 0;

						for size in sub_element_boxes {
							w = w.max(size.w);
							h += size.h;
						}

						Vec2i::new(w, h)
					},
					Layout::Flex { gap, direction, is_reverse, axis_alignment, cross_alignment, spacing } => {
						match direction {
							FlexDirection::Row => {
								let mut w = ((sub_element_boxes.len() - 1) * gap) as i32;
								let mut h = 0;

								for size in sub_element_boxes {
									w += size.w;
									h = h.max(size.h);
								}

								Vec2i::new(w, h)
							},
							FlexDirection::Col => {
								let mut w = 0;
								let mut h = ((sub_element_boxes.len() - 1) * gap) as i32;

								for size in sub_element_boxes {
									w = w.max(size.w);
									h += size.h;
								}

								Vec2i::new(w, h)
							},
						}
					},
					Layout::Grid { gap_row, gap_col, rows, cols } => todo!(),
				}
			},
			Tag::Label { text, font } => {
				context.calculate_text_size(text, font.size).into()
			},
		};

		let measured_size = frame_size + content_size;
		let size = measured_size.max(min_size);

		RenderBox {
			x: 0,
			y: 0,
			w: size.x,
			h: size.y,
		}
	}

	// Position sub elements within the element
	pub fn arrange_element_content(element: &Element, sub_element_boxes: &mut Vec<RenderBox>) {

	}

	// Calculate minimum sizes based on styles & composition
	pub fn measure_elements<'a>(root: &'a Element, context: &impl RenderContext) -> (Vec<&'a Element>, Vec<RenderBox>) {
		let mut order: Vec<&Element> = vec![];
		let mut linkages: Vec<RenderRef> = vec![];

		let mut queue = VecDeque::from([root]);

		let mut render_index = 0;

		// 1. Render Ordering
		// Determine render order with linkages between elements and their sub-elements
		// Linkages essentially map the render index of an element to the render indices
		// of each of its contained sub-elements
		// That way, when measuring elements non-recursively in reverse render order,
		// Elements that contain sub-elements will have a way to retrieve the measured
		// sizes of their sub-elements
		while queue.len() != 0 && let Some(next) = queue.pop_front() {

			// Sub-element render indices will start at current + queue size + 1

			let element_indices = if next.elements.len() != 0 {
				let mut element_indices = Vec::with_capacity(next.elements.len());

				// Add contents to queue
				for el in &next.elements {
					element_indices.push(render_index + queue.len() + 1);
					queue.push_back(&el);
				}

				Some(element_indices)
			} else {
				None
			};

			order.push(next);
			linkages.push(RenderRef { render_index: render_index, element_render_indices: element_indices });

			render_index += 1;
		}

		// Measured elements in reverse order
		// let mut boxes 

		let mut boxes = Vec::from_iter(order.iter().map(|_| {
			RenderBox::zero()
		}));

		// 2. Size Measuring
		for i in (0..order.len()).rev() {
			let element = order[i];
			let linkage = &linkages[i];

			let sub_element_boxes = if let Some(indices) = &linkage.element_render_indices {
				Vec::from_iter(indices.iter().map(|ref_i| {
					boxes[*ref_i]
				}))
			} else {
				Vec::with_capacity(0)
			};

			boxes[i] = Element::measure_element(element, &sub_element_boxes, context);
		}

		// 3. Position Arrangement
		// All positions are relative to the root element
		// Absolute positions are determined during rendering by applying an offset
		for i in 0..order.len() {
			let element = order[i];
			let linkage = &linkages[i];

			let mut sub_element_boxes = if let Some(indices) = &linkage.element_render_indices {
				Vec::from_iter(indices.iter().map(|ref_i| {
					boxes[*ref_i]
				}))
			} else {
				Vec::with_capacity(0)
			};

			Element::arrange_element_content(element, &mut sub_element_boxes);
		}

		(order, boxes)
	}

	// pub fn render_elements(order: Vec<&Element>, sizes: Vec<Vec2i>, rb: &RenderBox, context: &mut impl RenderContext) {
	// 	for i in 0..order.len() {
	// 		let element = *order.get(i).unwrap();
	// 		let size = *sizes.get(i).unwrap();


	// 	}
	// }

	// pub fn measure_struct<'a>(root: &'a Element, b: &'a RenderBox, context: &'a impl RenderContext) -> Vec<MeasuredElement<'a>> {
	// 	// 1. Breadth-first traversal: collect all elements into an ordered list (render order)

	// 	let mut elements: Vec<MeasuredElement> = vec![];
	// 	let mut queue = VecDeque::from([root]);

	// 	while queue.len() != 0 && let Some(next) = queue.pop_front() {
	// 		elements.push(MeasuredElement { b: RenderBox, element: () });


	// 		// Add contents to queue
	// 		for el in &next.content {
	// 			queue.push_back(&el);
	// 		}
	// 	}



	// 	// 2. Measuring: calculate element sizes in reverse render order

	// 	let measures = vec![];

		

	// 	measures
	// }

	pub fn measure(&self, b: &RenderBox, context: &impl RenderContext) -> Vec2i {
		let mut b = *b;
		let mut measured_width: i32 = 0;
		let mut measured_height: i32 = 0;

		let mut min_size: Vec2i = Vec2i::zero();

		// Apply padding & border to get content area
        if let Some(style) = &self.style {
            if let Some(border) = &style.border {
				measured_width += (border.width.l + border.width.r) as i32;
				measured_height += (border.width.t + border.width.b) as i32;
				b = border.internal_box(&b);
            }

            if let Some(padding) = &style.padding {
				measured_width += (padding.l + padding.r) as i32;
				measured_height += (padding.t + padding.b) as i32;
				b = padding.internal_box(&b);
            }

			min_size.x = style.min_width;
			min_size.y = style.min_height;
        }

		let tag_size = match &self.tag {
			Tag::Frame => {
				match &self.layout {
					Layout::Sequential => {
						let mut w = 0;
						for el in &self.elements {
							let size = el.measure(&b, context);
							measured_height += size.y;
							w = w.max(size.x);
						}

						Vec2i {x: measured_width + w, y: measured_height}
					},
					Layout::Flex { gap, direction, is_reverse, axis_alignment, cross_alignment, spacing } => {
						match direction {
							FlexDirection::Row => {
								let mut w = (gap * (self.elements.len() - 1)) as i32;
								let mut h = 0;
								for el in &self.elements {
									let size = el.measure(&b, context);
									w += size.x;
									h = h.max(size.y);
								}

								Vec2i {x: measured_width + w, y: measured_height + h}
							},
							FlexDirection::Col => {
								let mut w = 0;
								let mut h = (gap * (self.elements.len() - 1)) as i32;
								for el in &self.elements {
									let size = el.measure(&b, context);
									h += size.y;
									w = w.max(size.x);
								}

								Vec2i {x: measured_width + w, y: measured_height + h}
							},
						}
					},
					Layout::Grid { gap_row, gap_col, rows, cols } => todo!(),
				}
			},
			Tag::Label { text, font } => {
				let (content_width, content_height) = context.calculate_text_size(text, font.size);

				Vec2i {x: measured_width + content_width, y: measured_height + content_height}
			},
		};

		Vec2i {x: tag_size.x.max(min_size.x), y: tag_size.y.max(min_size.y)}
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
						for el in &self.elements {
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
								for el in &self.elements {
									let size = el.measure(&b, context);

									let y = match cross_alignment {
										Alignment::Start => b.y,
										Alignment::Center => b.y + (b.h - size.y) / 2,
										Alignment::End => b.y + b.h - size.y,
									};

									let el_rb = RenderBox {x: x, y: y, w: size.x, h: size.y};
									el.render(&el_rb, context);

									x += size.x + (*gap as i32);
								}
							},
							FlexDirection::Col => {
								let mut y = b.y;
								for el in &self.elements {
									let size = el.measure(&b, context);

									let x = match cross_alignment {
										Alignment::Start => b.x,
										Alignment::Center => b.x + (b.w - size.x) / 2,
										Alignment::End => b.x + b.w - size.x,
									};

									let el_rb = RenderBox {x: x, y: y, w: size.x, h: size.y};
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
		layout: Layout::Flex { gap: 6, direction: FlexDirection::Row, is_reverse: false, axis_alignment: Alignment::Start, cross_alignment: Alignment::End, spacing: Spacing::None },
        elements: vec![
            Element {
                tag: Tag::Label {
                    text: String::from("one"),
					font: Font { size: 32 },
                },
				layout: Layout::Sequential,
                elements: vec![],
                style: Some(Style {
                    padding: Some(Padding::new_uniform(3)),
                    border: Some(Border { color: Color::ORANGE, width: SidedSize::new_uniform(3) }),
					min_width: 0,
					min_height: 0,
                }),
            },
            Element {
                tag: Tag::Label {
                    text: String::from("two"),
					font: Font { size: 16 },
                },
				layout: Layout::Sequential,
                elements: vec![],
                style: Some(Style {
                    padding: Some(Padding::new_uniform(3)),
                    border: Some(Border { color: Color::ORANGE, width: SidedSize::new_uniform(3) }),
					min_width: 0,
					min_height: 0,
                }),
            },
			Element {
				tag: Tag::Frame,
				layout: Layout::Flex { gap: 6, direction: FlexDirection::Row, is_reverse: false, axis_alignment: Alignment::Start, cross_alignment: Alignment::End, spacing: Spacing::None },
				elements: vec![
					Element {
						tag: Tag::Label {
							text: String::from("three"),
							font: Font { size: 24 },
						},
						layout: Layout::Sequential,
						elements: vec![],
						style: Some(Style {
							padding: Some(Padding::new_uniform(3)),
							border: Some(Border { color: Color::ORANGE, width: SidedSize::new_uniform(3) }),
							min_width: 0,
							min_height: 0,
						}),
					},
				],
				style: Some(Style {
					padding: Some(Padding::new_uniform(3)),
					border: Some(Border { color: Color::GREEN, width: SidedSize::new_uniform(3) }),
					min_width: 0,
					min_height: 0,
				}),
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
        style: Some(Style {
            padding: Some(Padding::new_uniform(3)),
            border: Some(Border { color: Color::GREEN, width: SidedSize::new_uniform(3) }),
			min_width: 400,
			min_height: 0,
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

			let measures = Element::measure_elements(&ui, &d);
			println!("TEST");
			// Element::render_elements(order, sizes, &rb, &mut d);
        }
    }
}
