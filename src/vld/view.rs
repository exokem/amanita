use raylib::{color::Color, math::Vector2};

#[allow(dead_code)]
pub trait CameraContext {

	// position (Camera2D target): the point in world space that the camera is placed at (top-left corner)

	// offset (Camera2D offset): screen position; the point on the screen at which the camera target will appear

	fn position(&self) -> &Vector2;
	fn position_mut(&mut self) -> &mut Vector2;

	fn offset(&self) -> &Vector2;
	fn offset_mut(&mut self) -> &mut Vector2;

	fn zoom(&self) -> &f32;
	fn zoom_mut(&mut self) -> &mut f32;

	fn world_to_screen(&self, world: Vector2) -> Vector2;
	fn screen_to_world(&self, screen: Vector2) -> Vector2;
}

#[allow(dead_code)]
pub trait RenderContext {
	fn fill_rect(&mut self, x: i32, y: i32, w: i32, h: i32, color: impl Into<Color>);

	fn outline_rect(&mut self, x: i32, y: i32, w: i32, h: i32, color: impl Into<Color>);

	fn calculate_text_size(&self, text: &str, size: i32) -> (i32, i32);

	fn fill_text(&mut self, x: i32, y: i32, text: &str, size: i32, color: impl Into<Color>);
}

#[allow(dead_code)]
pub struct RenderArea {
	pub rx: i32,
	pub ry: i32,

	pub rw: i32,
	pub rh: i32,
}

#[allow(dead_code)]
pub trait View {
	/// Render relative to the viewport camera (fixed within the screen).
	fn render_overlay(&self, _area: RenderArea, _context: &mut impl super::RenderContext) {}

	/// Render relative to global coordinates (will not move with the camera).
	fn render_world(&self, _area: RenderArea, _context: &mut impl super::RenderContext) {}
}