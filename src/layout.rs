mod layout_enums {
	#[derive(Debug, Clone, Copy)]
	pub enum FlexDirection {
		Row,
		Col,
	}

	#[derive(Debug, Clone, Copy)]
	pub enum Spacing {
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


	#[derive(Debug, Clone, Copy)]
	pub enum Alignment {
		// |XYZ------------|
		Start,

		// |------XYZ------|
		Center,

		// |------------XYZ|
		End,
	}
}

pub use layout_enums::*;

#[derive(Debug, Clone, Copy)]
pub struct Flex {
	pub gap: usize,
	pub direction: FlexDirection,

	pub is_reverse: bool,

	// In the layout direciton
	pub axis_alignment: Alignment,
	// Perpendicular to the layout direction
	pub cross_alignment: Alignment,

	pub spacing: Spacing,
}

impl Flex {
	pub fn row() -> Self {
		Self {
			gap: 0,
			direction: FlexDirection::Row,
			is_reverse: false,
			axis_alignment: Alignment::Start,
			cross_alignment: Alignment::Center,
			spacing: Spacing::None,
		}
	}

	pub fn col() -> Self {
		Self {
			gap: 0,
			direction: FlexDirection::Col,
			is_reverse: false,
			axis_alignment: Alignment::Start,
			cross_alignment: Alignment::Start,
			spacing: Spacing::None,
		}
	}

	pub fn item_gap(mut self, gap: usize) -> Self {
		self.gap = gap;
		self
	}

	pub fn reverse(mut self) -> Self {
		self.is_reverse = true;
		self
	}

	pub fn axis_align(mut self, alignment: Alignment) -> Self {
		self.axis_alignment = alignment;
		self
	}

	pub fn cross_align(mut self, alignment: Alignment) -> Self {
		self.cross_alignment = alignment;
		self
	}

	pub fn spacing(mut self, spacing: Spacing) -> Self {
		self.spacing = spacing;
		self
	}

	pub fn build(&self) -> Layout {
		Layout::Flex { properties: *self }
	}

	pub fn calc_total_gap(&self, item_count: usize) -> i32 {
		((item_count - 1) * self.gap) as i32
	}
}

#[derive(Debug)]
pub enum Layout {
	Sequential,
	Flex {
		properties: Flex,
		// gap: usize,
		// direction: FlexDirection,

		// is_reverse: bool,

		// // In the layout direciton
		// axis_alignment: Alignment,
		// // Perpendicular to the layout direction
		// cross_alignment: Alignment,

		// spacing: Spacing,
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
