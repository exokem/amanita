pub mod context {
	use raylib::{prelude::*};

    use crate::vld::{CameraContext, InputContext, InputState, RenderContext};

	impl CameraContext for Camera2D {
		fn position(&self) -> &Vector2 {
			&self.target
		}

		fn position_mut(&mut self) -> &mut Vector2 {
			&mut self.target
		}

		fn offset(&self) -> &Vector2 {
			&self.offset
		}

		fn offset_mut(&mut self) -> &mut Vector2 {
			&mut self.offset
		}

		fn zoom(&self) -> &f32 {
			&self.zoom
		}

		fn zoom_mut(&mut self) -> &mut f32 {
			&mut self.zoom
		}
		
		fn world_to_screen(&self, world: Vector2) -> Vector2 {
			unsafe {
				ffi::GetWorldToScreen2D(world.into(), self.into()).into()
			}
		}
		
		fn screen_to_world(&self, screen: Vector2) -> Vector2 {
			unsafe {
				ffi::GetScreenToWorld2D(screen.into(), self.into()).into()
			}
		}
	}


	impl InputContext for RaylibHandle {
		fn button_status(&self, button: MouseButton) -> InputState {
			InputState {
				up: self.is_mouse_button_up(button),
				down: self.is_mouse_button_down(button),
				pressed: self.is_mouse_button_pressed(button),
				released: self.is_mouse_button_released(button),
			}
		}

		fn key_status(&self, key: KeyboardKey) -> InputState {
			InputState {
				up: self.is_key_up(key),
				down: self.is_key_down(key),
				pressed: self.is_key_pressed(key),
				released: self.is_key_released(key),
			}
		}

		fn cursor_delta(&self) -> Vector2 {
			self.get_mouse_delta()
		}

		fn cursor_pos_screen(&self) -> Vector2 {
			self.get_mouse_position()
		}

		fn scroll_delta(&self) -> Vector2 {
			self.get_mouse_wheel_move_v().into()
		}
		
		fn scroll_delta_x(&self) -> f32 {
			self.get_mouse_wheel_move()
		}
	}

	impl RenderContext for RaylibDrawHandle<'_> {
		fn fill_rect(&mut self, x: i32, y: i32, w: i32, h: i32, color: impl Into<Color>) {
			self.draw_rectangle(x, y, w, h, color.into());
		}

		fn outline_rect(&mut self, x: i32, y: i32, w: i32, h: i32, color: impl Into<Color>) {
			self.draw_rectangle_lines(x, y, w, h, color.into());
		}
		
		fn calculate_text_size(&self, text: &str, size: i32) -> (i32, i32) {
			(self.measure_text(&text, size), size)
		}
		
		fn fill_text(&mut self, x: i32, y: i32, text: &str, size: i32, color: impl Into<Color>) {
			self.draw_text(text, x, y, size, color.into());
		}
	}

	impl RenderContext for RaylibMode2D<'_, RaylibDrawHandle<'_>> {
		fn fill_rect(&mut self, x: i32, y: i32, w: i32, h: i32, color: impl Into<Color>) {
			self.draw_rectangle(x, y, w, h, color.into());
		}

		fn outline_rect(&mut self, x: i32, y: i32, w: i32, h: i32, color: impl Into<Color>) {
			self.draw_rectangle_lines(x, y, w, h, color.into());
		}
		
		fn calculate_text_size(&self, text: &str, size: i32) -> (i32, i32) {
			(self.measure_text(&text, size), size)
		}
		
		fn fill_text(&mut self, x: i32, y: i32, text: &str, size: i32, color: impl Into<Color>) {
			self.draw_text(text, x, y, size, color.into());
		}
	}
}