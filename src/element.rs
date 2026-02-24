use core::panic;
use std::collections::{VecDeque};

use raylib::color::Color;

use crate::{layout::{Alignment, FlexDirection, Layout, Spacing}, style::{Border, Padding, Style}, tag::Tag, vec2::Vec2i, vld::{InputContext, MeasureContext, RenderContext}, RenderBox};

#[derive(Debug)]
pub struct Element {
	pub tag: Tag,
	pub elements: Vec<Element>,
	pub style: Style,
	pub layout: Layout,
	pub state: ElementState,
	// pub state: State,
}

#[derive(Debug)]
pub enum ElementState {
	Scroll { offset_y: f32 },
	None,
}

impl ElementState {
	pub fn update(&mut self, input: &impl InputContext) {
		match self {
			Self::Scroll { offset_y } => {
				*offset_y += input.scroll_delta().y;
			}
			_ => {}
		}
	}
}

struct RenderRef {
	render_index: usize,
	element_render_indices: Option<Vec<usize>>
}

// Boxes here are just relative to the element and are not adjusted during arrangement
// They are purely for reference while arranging the element's contents

#[derive(Debug, Clone, Copy)]
pub struct ElementMeasure {
	pub position: Vec2i,

	// Size of the box containing the entire element
	pub size: Vec2i,

	// Box within the element that contains its contents
	pub inner_box: RenderBox,
}

impl Element {
	fn measure_element_content(element: &Element, sub_element_measures: &[ElementMeasure], context: &impl MeasureContext) -> Vec2i {

		// Sequential (plain column):
		// - Width: largest sub element width
		// - Height: sum of sub element heights
		let measure_sequential = || -> Vec2i {
			let mut w = 0;
			let mut h = 0;

			for measure in sub_element_measures {
				w = w.max(measure.size.x);
				h += measure.size.y;
			}

			Vec2i::new(w, h)
		};

		match &element.tag {
			Tag::Label { text, font } => {
				context.calculate_text_size(text, font.size).into()
			},
			_ => {
				match &element.layout {
					Layout::Scroll => {
						// TODO: probably shouldn't use max values here
						Vec2i::new(
							element.style.max_width.unwrap_or(element.style.min_width),
							element.style.max_height.unwrap_or(element.style.min_height),
						)
					},
					Layout::Sequential => measure_sequential(),
					Layout::Flex { properties } => {
						let total_gap = properties.calc_total_gap(sub_element_measures.len());

						let (w, h) = sub_element_measures.iter().fold((0, 0), |(w, h), measure| {
							match properties.direction {
								FlexDirection::Row => (
									w + measure.size.x,
									h.max(measure.size.y),
								),
								FlexDirection::Col => (
									w.max(measure.size.x),
									h + measure.size.y,
								)
							}
						});

						match properties.direction {
							FlexDirection::Row => Vec2i::new(w + total_gap, h),
							FlexDirection::Col => Vec2i::new(w, h + total_gap),
						}
					},
					Layout::Grid { gap_row, gap_col, rows, cols } => todo!(),
				}
			},
		}
	}

	pub fn measure_element(element: &Element, sub_element_measures: &[ElementMeasure], context: &impl MeasureContext) -> ElementMeasure {
		let content_size = Element::measure_element_content(element, sub_element_measures, context);

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
			Layout::Scroll => {
				let x = 0;
				let mut y = 0;

				for i in 0..element.elements.len() {
					let sub_measure = &mut sub_element_measures[i];
					sub_measure.position.x = x;
					sub_measure.position.y = y;

					y += sub_measure.size.y;
				}
			},
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
			Layout::Flex { properties } => {
				let gap = properties.gap as i32;
				let total_gap = properties.calc_total_gap(sub_element_measures.len());

				let (inner_start, inner_cross_start, inner_size, inner_cross_size, content_size) = match properties.direction {
					FlexDirection::Row => {
						(inner_x, inner_y, measure.inner_box.w, measure.inner_box.h, content_width)
					},
					FlexDirection::Col => {
						(inner_y, inner_x, measure.inner_box.h, measure.inner_box.w, content_height)
					},
				};

				let unused_size = inner_size - (total_gap + content_size);

				let (mut coord, coord_inc) = match properties.spacing {
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
						(match properties.axis_alignment {
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
					let el_cross_size = match properties.direction {
						FlexDirection::Row => el.size.y,
						FlexDirection::Col => el.size.x,
					};

					let cross_coord = match properties.cross_alignment {
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

					match properties.direction {
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

	pub fn render_element(element: &Element, offset: Vec2i, measure: &ElementMeasure, input: &impl InputContext, context: &mut impl RenderContext) {
		context.outline_rect(measure.position.x + offset.x, measure.position.y + offset.y, measure.size.x, measure.size.y, Color::RED);

		if let Some(bg) = element.style.background {
			context.fill_rect(measure.position.x + offset.x, measure.position.y + offset.y, measure.size.x, measure.size.y, bg);
		}
		// context.outline_rect(measure.inner_box.x, measure.inner_box.y, measure.inner_box.w, measure.inner_box.h, Color::GREEN);
	}

	pub fn update(root: &mut Element, input: &impl InputContext, measure_context: &impl MeasureContext) {
		// TODO: determine best order for updating (probably reverse render order, especially so inputs can be consumed at the highest level)

		let mut queue = VecDeque::from([root]);

		while !queue.is_empty() && let Some(el) = queue.pop_front() {
			el.state.update(input);

			for sub in &mut el.elements {
				queue.push_back(sub);
			}
		}
	}

	pub fn render(root: &Element, offset: Option<Vec2i>, input: &impl InputContext, measure_context: &impl MeasureContext, context: &mut impl RenderContext) {
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

			// Do not include scroll layout sub-elements in standard rendering
			let element_indices = if next.elements.len() != 0 && !matches!(next.layout, Layout::Scroll) {
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
				Element::measure_element(element, &measures[start..end], measure_context)
			} else {
				Element::measure_element(element, &measures[0..0], measure_context)
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

		let offset = offset.unwrap_or(Vec2i::zero());

		// 4. Rendering
		for i in 0..order.len() {
			let element = order[i];
			let measure = measures[i];

			match element.layout {
				Layout::Scroll => {
					// need to ensure that a render texture is created here, and
					// that all contained sub-elements are rendered within it
					let body = &element.elements[0];

					

					Element::render_element(element, offset, &measure, input, context);
					if let Some(mut clip_context) = context.clip_region(
						measure.inner_box.x + offset.x + measure.position.x,
						measure.inner_box.y + offset.y + measure.position.y,
						measure.inner_box.w,
						measure.inner_box.h,
					) {
						let (ox, oy) = match element.state {
							ElementState::Scroll { offset_y } => {
								(0.0, offset_y)
							},
							_ => (0.0, 0.0)
						};

						let scrolled_offset = offset + measure.position + (0, oy as i32).into();
						
						Element::render(&body, Some(scrolled_offset), input,  measure_context, &mut clip_context);
					} else {
						panic!("Unsupported clipping operation")
					}
				},
				_ => Element::render_element(element, offset, &measure, input, context),
			}
		}
	}
}
