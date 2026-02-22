use std::{collections::VecDeque, ffi::CString, i32::MAX, ops::{Add, Sub}};

use raylib::{color::Color, ffi::{self, Vector2}, prelude::RaylibDraw, RaylibHandle};
use crate::{ray::context, vld::RenderContext, vec2::Vec2i};

mod ray;
mod vld;
mod vec2;

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

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone, Copy)]
struct Border {
    color: Color,
    width: SidedSize,
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
    style: Style,
	layout: Layout,
}

struct RenderRef {
	render_index: usize,
	element_render_indices: Option<Vec<usize>>
}

// Boxes here are just relative to the element and are not adjusted during arrangement
// They are purely for reference while arranging the element's contents

#[derive(Debug, Clone, Copy)]
struct ElementMeasure {
	position: Vec2i,

	// Size of the box containing the entire element
	size: Vec2i,

	// Box within the element that contains its contents
	inner_box: RenderBox,
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

	pub fn measure_element(element: &Element, sub_element_measures: &[ElementMeasure], context: &impl RenderContext) -> ElementMeasure {
		let content_size = match &element.tag {
			Tag::Frame => {
				match &element.layout {
					Layout::Sequential => {
						let mut w = 0;
						let mut h = 0;

						for measure in sub_element_measures {
							w = w.max(measure.size.x);
							h += measure.size.y;
						}

						Vec2i::new(w, h)
					},
					Layout::Flex { gap, direction, is_reverse, axis_alignment, cross_alignment, spacing } => {
						match direction {
							FlexDirection::Row => {
								let mut w = ((sub_element_measures.len() - 1) * gap) as i32;
								let mut h = 0;

								for measure in sub_element_measures {
									w += measure.size.x;
									h = h.max(measure.size.y);
								}

								Vec2i::new(w, h)
							},
							FlexDirection::Col => {
								let mut w = 0;
								let mut h = ((sub_element_measures.len() - 1) * gap) as i32;

								for measure in sub_element_measures {
									w = w.max(measure.size.x);
									h += measure.size.y;
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

		// Border + padding

		let border = element.style.border.unwrap_or_else(Border::none);
		let padding = element.style.padding.unwrap_or(Padding::new_uniform(0));

		// size is content size plus frame size

		let frame_size_x = (border.width.l + border.width.r + padding.l + padding.r) as i32;
		let frame_size_y = (border.width.t + border.width.b + padding.t + padding.b) as i32;

		let size = Vec2i {
			x: element.style.min_width.max(content_size.x + frame_size_x),
			y: element.style.min_height.max(content_size.y + frame_size_y),
		};

		// inner box is size minus frame size

		let inner_box = RenderBox {
			x: (border.width.l + padding.l) as i32,
			y: (border.width.t + padding.t) as i32,
			w: size.x - frame_size_x,
			h: size.y - frame_size_y,
		};

		ElementMeasure { position: Vec2i::zero(), size: size, inner_box: inner_box }
	}

	// Position sub elements within the element
	pub fn arrange_element_content(element: &Element, measure: &ElementMeasure, sub_element_measures: &mut [ElementMeasure]) {
		if sub_element_measures.is_empty() {
			return
		}

		let content_width: i32 = sub_element_measures.iter().map(|m| m.size.x).sum();
		let content_height: i32 = sub_element_measures.iter().map(|m| m.size.y).sum();

		let inner_x = measure.position.x + measure.inner_box.x;
		let inner_y = measure.position.y + measure.inner_box.y;

		match &element.layout {
			Layout::Sequential => {
				let x = measure.position.x + measure.inner_box.x;
				let mut y = measure.position.y + measure.inner_box.y;

				for i in 0..element.elements.len() {
					let sub_measure = &mut sub_element_measures[i];

					sub_measure.position.x = x;
					sub_measure.position.y = y;

					y += sub_measure.size.y;
				}
			},
			Layout::Flex { gap, direction, is_reverse, axis_alignment, cross_alignment, spacing } => {
				let gap = *gap as i32;
				let total_gap = gap * (sub_element_measures.len() as i32 - 1);

				let (inner_start, inner_cross_start, inner_size, inner_cross_size, content_size) = match *direction {
					FlexDirection::Row => {
						(inner_x, inner_y, measure.inner_box.w, measure.inner_box.h, content_width)
					},
					FlexDirection::Col => {
						(inner_y, inner_x, measure.inner_box.h, measure.inner_box.w, content_height)
					},
				};

				let unused_size = inner_size - (total_gap + content_size);

				let (mut coord, coord_inc) = match spacing {
					Spacing::Around => {
						let space = unused_size / (sub_element_measures.len() as i32 * 2);
						(inner_start + space, 2 * space)
					},
					Spacing::Between => {
						let space = unused_size / (sub_element_measures.len() as i32 - 1);
						(inner_start, space)
					},
					Spacing::Even => {
						let space = unused_size / (sub_element_measures.len() as i32 + 1);
						(inner_start + space, space)
					},
					Spacing::Stretch => todo!(),
					Spacing::None => {
						(match axis_alignment {
							Alignment::Start => {
								inner_start
							},
							Alignment::Center => {
								inner_start + (inner_size - total_gap - content_size) / 2
							},
							Alignment::End => {
								inner_start + inner_size - total_gap - content_size
							},
						}, 0)
					},
				};

				for el in sub_element_measures {
					let el_cross_size = match direction {
						FlexDirection::Row => el.size.y,
						FlexDirection::Col => el.size.x,
					};

					let cross_coord = match cross_alignment {
						Alignment::Start => {
							inner_cross_start
						},
						Alignment::Center => {
							inner_cross_start + (inner_cross_size - el_cross_size) / 2
						},
						Alignment::End => {
							inner_cross_start + inner_cross_size - el_cross_size
						},
					};

					match direction {
						FlexDirection::Row => {
							el.position.x = coord;
							el.position.y = cross_coord;

							coord += gap + coord_inc + el.size.x;
						},
						FlexDirection::Col => {
							el.position.x = cross_coord;
							el.position.y = coord;

							coord += gap + coord_inc + el.size.y;
						},
					}
				}
			},
			Layout::Grid { gap_row, gap_col, rows, cols } => {},
		}
	}

	pub fn render_element(element: &Element, measure: &ElementMeasure, offset: Vec2i, context: &mut impl RenderContext) {
		context.outline_rect(measure.position.x + offset.x, measure.position.y + offset.y, measure.size.x, measure.size.y, Color::RED);
		// context.outline_rect(measure.inner_box.x, measure.inner_box.y, measure.inner_box.w, measure.inner_box.h, Color::GREEN);
	}

	// Calculate minimum sizes based on styles & composition
	pub fn render(root: &Element, context: &mut impl RenderContext) {
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

		let mut measures = Vec::from_iter(order.iter().map(|_| {
			ElementMeasure {
				position: Vec2i::zero(),
				size: Vec2i::zero(),
				inner_box: RenderBox::zero(),
			}
		}));

		// 2. Size Measuring
		for i in (0..order.len()).rev() {
			let element = order[i];
			let linkage = &linkages[i];

			measures[i] = if let Some(indices) = &linkage.element_render_indices {
				let start = *indices.first().unwrap();
				let end = *indices.last().unwrap() + 1;
				Element::measure_element(element, &measures[start..end], context)
			} else {
				Element::measure_element(element, &measures[0..0], context)
			};

			// measures[i] = Element::measure_element(element, &sub_element_measures, context);
		}

		// 3. Position Arrangement
		// All positions are relative to the root element
		// Absolute positions are determined during rendering by applying an offset
		for i in 0..order.len() {
			let element = order[i];
			let measure = measures[i];
			let linkage = &linkages[i];

			if let Some(indices) = &linkage.element_render_indices {
				let start = *indices.first().unwrap();
				let end = *indices.last().unwrap() + 1;
				Element::arrange_element_content(element, &measure, &mut measures[start..end])
			} else {
				Element::arrange_element_content(element, &measure, &mut measures[0..0])
			}
		}

		// 4. Rendering
		for i in 0..order.len() {
			let element = order[i];
			let measure = measures[i];

			Element::render_element(element, &measure, Vec2i::new(10, 10), context);
		}
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
		layout: Layout::Flex { gap: 6, direction: FlexDirection::Row, is_reverse: false, axis_alignment: Alignment::Center, cross_alignment: Alignment::Center, spacing: Spacing::Between },
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
            let rb = RenderBox {x: 10, y: 10, w: w - 20, h: h - 20};

			Element::render(&ui, &mut d);
        }
    }
}
