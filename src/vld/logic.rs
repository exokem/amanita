use raylib::{ffi::{KeyboardKey, MouseButton}, math::Vector2};

use crate::{vld::CameraContext};


#[allow(dead_code)]
pub struct InputState {
	pub pressed: bool,
	pub released: bool,
	pub up: bool,
	pub down: bool,
}

#[allow(dead_code)]
pub trait InputContext {
	fn button_status(&self, button: MouseButton) -> InputState;

	fn key_status(&self, key: KeyboardKey) -> InputState;

	// Mouse screen delta
	fn cursor_delta(&self) -> Vector2;

	/// Mouse screen position
	fn cursor_pos_screen(&self) -> Vector2;

	fn cursor_pos_world(&self, camera: &impl CameraContext) -> Vector2;

	fn scroll_delta(&self) -> Vector2;

	fn scroll_delta_x(&self) -> f32;
}

#[allow(dead_code)]
pub trait Logic {
	/// Performs stateful update logic before rendering.
	/// Effects may be independent of rendering.
	fn update(&self, _input: &impl super::InputContext, _camera: &mut impl super::CameraContext) {}

	fn update_mut(&mut self, _input: &impl super::InputContext, _camera: &mut impl super::CameraContext) {}

	fn update_world_mut(&mut self, _render_pos: Vector2, _input: &impl super::InputContext, _camera: &mut impl super::CameraContext) {}
}