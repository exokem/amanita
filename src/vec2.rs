
#[derive(Debug, Clone, Copy)]
pub struct Vec2i {
	pub x: i32,
	pub y: i32,
}

impl std::ops::Add for Vec2i {
	type Output = Vec2i;

	fn add(self, rhs: Self) -> Self::Output {
		Vec2i::new(self.x + rhs.x, self.y + rhs.y)
	}
}

impl std::ops::Sub for Vec2i {
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
