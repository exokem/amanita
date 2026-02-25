use core::panic;
use std::collections::{HashMap, VecDeque};
use std::iter::Rev;
use std::ops::{Index, IndexMut};
use std::slice::Iter;
use std::sync::atomic::{AtomicUsize, Ordering};
use raylib::color::Color;

use crate::{layout::{Alignment, FlexDirection, Layout, Spacing}, style::{Border, Padding, Style}, tag::Tag, vec2::Vec2i, vld::{InputContext, MeasureContext, RenderContext}, RenderBox};
use crate::style::Font;

static ELEMENT_COUNTER: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug)]
pub struct Element {
	index: usize,
	tag: Tag,
	elements: Vec<usize>,
	style: Style,
	layout: Layout,
	state: ElementState,
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

pub struct ElementSet {
	elements: Vec<Element>,

	render_order: Vec<usize>,
}

impl Index<usize> for ElementSet {
	type Output = Element;

	fn index(&self, index: usize) -> &Self::Output {
		&self.elements[index]
	}
}

impl IndexMut<usize> for ElementSet {
	fn index_mut(&mut self, index: usize) -> &mut Self::Output {
		&mut self.elements[index]
	}
}

impl ElementSet {
	pub fn new(mut root: impl FnMut(&mut ElementSet) -> usize) -> Self {
		let mut set = Self {
			elements: vec![],
			render_order: vec![],
		};

		root(&mut set);

		set
	}

	pub fn len(&self) -> usize {
		self.elements.len()
	}

	pub fn root(&self) -> &Element {
		&self.elements[0]
	}

	pub fn new_frame(&mut self, layout: Layout, style: Style, mut sub_elements: impl FnMut(&mut ElementSet) -> Vec<usize>) -> usize {
		let e = Element {
			index: ELEMENT_COUNTER.fetch_add(1, Ordering::Relaxed),
			tag: Tag::Frame,
			state: ElementState::None,
			layout,
			style,
			elements: sub_elements(self)
		};

		let index = e.index;
		self.elements.push(e);

		index
	}

	pub fn new_scroll_frame(&mut self, style: Style, mut body: impl FnMut(&mut ElementSet) -> usize) -> usize {
		let mut e = Element {
			index: ELEMENT_COUNTER.fetch_add(1, Ordering::Relaxed),
			tag: Tag::Frame,
			state: ElementState::Scroll { offset_y: 0.0 },
			layout: Layout::Scroll,
			elements: vec![body(self)],
			style,
		};

		let index = e.index;
		self.elements.push(e);

		index
	}

	// TODO: font should be a style property most likely
	pub fn new_label(&mut self, text: String, font: Font, style: Style) -> usize {
		let e = Element {
			index: ELEMENT_COUNTER.fetch_add(1, Ordering::Relaxed),
			tag: Tag::Label {text, font},
			state: ElementState::None,
			layout: Layout::Sequential,
			elements: vec![],
			style,
		};

		let index = e.index;
		self.elements.push(e);

		index
	}

	pub fn sort(&mut self) -> &mut Self {
		self.elements.sort_by(|a, b| a.index.cmp(&b.index));

		self
	}

	fn update_render_ordering(&mut self) {
		let mut render_order: Vec<usize> = vec![];

		let mut queue = VecDeque::from([0usize]);

		let mut render_index = 0;

		// Determine render order with linkages between elements and their sub-elements
		// Linkages essentially map the render index of an element to the render indices
		// of each of its contained sub-elements
		// That way, when measuring elements non-recursively in reverse render order,
		// Elements that contain sub-elements will have a way to retrieve the measured
		// sizes of their sub-elements
		while queue.len() != 0 && let Some(next) = queue.pop_front() {

			let next = &self[next];

			// Sub-element render indices will start at current + queue size + 1

			if next.elements.len() != 0 {
				// Add contents to queue
				for i in &next.elements {
					queue.push_back(*i);
				}
			}

			render_order.push(next.index);

			render_index += 1;
		}

		self.render_order = render_order;
	}

	fn update(&mut self, input_context: &impl InputContext, measure_context: &impl MeasureContext) {
		// Calculate render order as needed
		// Will be cleared when an element needs to trigger ordering changes
		if self.render_order.is_empty() {
			self.update_render_ordering();
		}

		for element_index in self.render_order.iter().rev() {
			let el = &mut self.elements[*element_index];
			el.update(input_context, measure_context);
		}
	}

	// Output order MUST match sorted element order
	// Note that this is NOT the same as the render order, but rather the order of elements after
	// being sorted by their index
	fn measure(&self, measure_context: &impl MeasureContext) -> Vec<ElementMeasure> {
		let mut measures = Vec::from_iter(self.elements.iter().map(|_| {
			ElementMeasure {
				position: Vec2i::zero(),
				size: Vec2i::zero(),
				inner_box: RenderBox::zero(),
			}
		}));

		// Render order contains element indices
		// Element set is sorted by ascending element index
		for element_index in self.render_order.iter().rev() {
			let element = &self[*element_index];

			// Map indices of sub elements to calculated measures
			// This is necessary because sub elements may not have consecutive indices
			let sub_element_measures = element.elements.iter().map(|element_index| {
				measures[*element_index]
			}).collect::<Vec<_>>();

			// Store the measurement under the element's assigned index, not its position in the render order
			measures[*element_index] = Element::measure_element(element, &sub_element_measures, measure_context);
		}

		measures
	}

	fn arrange(&self, measures: &mut Vec<ElementMeasure>) {
		// All positions are relative to the root element
		// Absolute positions are determined during rendering by applying an offset
		for element_index in &self.render_order {
			let element = &self[*element_index];
			let measure = measures[*element_index];

			Element::arrange_element_content(element, &measure, measures);
		}
	}

	fn render(&self, offset: Option<Vec2i>, measures: &Vec<ElementMeasure>, input_context: &impl InputContext, render_context: &mut impl RenderContext) {

		let offset = offset.unwrap_or(Vec2i::zero());

		// Bind element index to clip box and render offset
		let mut clip_map: HashMap<usize, (RenderBox, Vec2i)> = HashMap::new();

		for element_index in &self.render_order {
			let element = &self[*element_index];
			let measure = measures[*element_index];

			let clip_context = if let Some(&(clip_region, clip_offset)) = clip_map.get(element_index) {
				if let Some(mut clipped_render_context) =render_context.clip_region(clip_region.x, clip_region.y, clip_region.w, clip_region.h) {
					Element::render_element(element, clip_offset, &measure, input_context, &mut clipped_render_context);
				} else {
					panic!("Unsupported clipping depth")
				}

				Some((clip_region, clip_offset))
			} else {
				Element::render_element(element, offset, &measure, input_context, render_context);
				None
			};

			if let Some((clip_region, clip_offset)) = clip_context {
				if !matches!(element.layout, Layout::Scroll) {
					for sub_index in &element.elements {
						clip_map.insert(*sub_index, (clip_region, clip_offset));
					}
				}
			}

			match element.layout {
				Layout::Scroll => {
					// need to ensure that a render texture is created here, and
					// that all contained sub-elements are rendered within it
					let body = &element.elements[0];

					// Element::render_element(element, offset, &measure, input_context, render_context);

					let scroll_clip = RenderBox::new(
						measure.inner_box.x + offset.x + measure.position.x,
						measure.inner_box.y + offset.y + measure.position.y,
						measure.inner_box.w,
						measure.inner_box.h,
					);

					let (ox, oy) = match element.state {
						ElementState::Scroll { offset_y } => {
							(0.0, offset_y)
						},
						_ => (0.0, 0.0)
					};

					let scrolled_offset = offset + (ox as i32, oy as i32).into();

					for sub_index in &element.elements {
						clip_map.insert(*sub_index, (scroll_clip, scrolled_offset));
					}
				},
				_ => {},
			}
		}
	}

	pub fn process_frame(&mut self, offset: Option<Vec2i>, input_context: &impl InputContext, measure_context: &impl MeasureContext, render_context: &mut impl RenderContext) {
		// 1. Update elements (reverse render order)
		self.update(input_context, measure_context);

		// 2. Measure elements (reverse render order)
		let mut measures = self.measure(measure_context);

		// 3. Arrange elements (render order)
		self.arrange(&mut measures);

		// 4. Render elements (render order)
		self.render(offset, &measures, input_context, render_context);
	}
}

impl Element {
	fn render_element(element: &Element, offset: Vec2i, measure: &ElementMeasure, input: &impl InputContext, context: &mut impl RenderContext) {
		context.outline_rect(measure.position.x + offset.x, measure.position.y + offset.y, measure.size.x, measure.size.y, Color::RED);

		if let Some(bg) = element.style.background {
			context.fill_rect(measure.position.x + offset.x, measure.position.y + offset.y, measure.size.x, measure.size.y, bg);
		}
		// context.outline_rect(measure.inner_box.x, measure.inner_box.y, measure.inner_box.w, measure.inner_box.h, Color::GREEN);
	}

	pub fn update(&mut self, input: &impl InputContext, measure_context: &impl MeasureContext) {
		self.state.update(input);
	}

    //<editor-fold desc="Measuring">
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

	fn measure_element(element: &Element, sub_element_measures: &[ElementMeasure], context: &impl MeasureContext) -> ElementMeasure {
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
    //</editor-fold>

    //<editor-fold desc="Arranging">
    // Position sub elements within the element
	fn arrange_element_content(element: &Element, measure: &ElementMeasure, all_measures: &mut Vec<ElementMeasure>) {
		if element.elements.is_empty() {
			return
		}

		let content_width: i32 = element.elements.iter().map(|sub_i| {
			all_measures[*sub_i].size.x
		}).sum();

		let content_height: i32 = element.elements.iter().map(|sub_i| {
			all_measures[*sub_i].size.y
		}).sum();

		let inner_x = measure.position.x + measure.inner_box.x;
		let inner_y = measure.position.y + measure.inner_box.y;

		match &element.layout {
			Layout::Sequential | Layout::Scroll => {
				let x = measure.position.x + measure.inner_box.x;
				let mut y = measure.position.y + measure.inner_box.y;

				for &sub_index in &element.elements {
					let sub_measure = &mut all_measures[sub_index];

					sub_measure.position.x = x;
					sub_measure.position.y = y;

					y += sub_measure.size.y;
				}
			},
			Layout::Flex { properties } => {
				let gap = properties.gap as i32;
				let total_gap = properties.calc_total_gap(element.elements.len());

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
						let space = unused_size / (element.elements.len() as i32 * 2);
						(inner_start + space, 2 * space)
					},
					Spacing::Between => {
						let space = unused_size / (element.elements.len() as i32 - 1);
						(inner_start, space)
					},
					Spacing::Even => {
						let space = unused_size / (element.elements.len() as i32 + 1);
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

				for sub_index in &element.elements {
					let el = &mut all_measures[*sub_index];

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
}
