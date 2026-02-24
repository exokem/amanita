pub mod context {
	use raylib::{prelude::*};

    use crate::vld::{CameraContext, InputContext, InputState, MeasureContext, RenderContext};

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

	pub struct RayInputContext {}

	impl InputContext for RayInputContext {
		fn button_status(&self, button: MouseButton) -> InputState {
			InputState {
				up: unsafe { ffi::IsMouseButtonUp(button as i32) },
				down: unsafe { ffi::IsMouseButtonDown(button as i32) },
				pressed: unsafe { ffi::IsMouseButtonPressed(button as i32) },
				released: unsafe { ffi::IsMouseButtonReleased(button as i32) },
			}
		}

		fn key_status(&self, key: KeyboardKey) -> InputState {
			InputState {
				up: unsafe { ffi::IsKeyUp((key as u32) as i32) },
				down: unsafe { ffi::IsKeyDown((key as u32) as i32) },
				pressed: unsafe { ffi::IsKeyPressed((key as u32) as i32) },
				released: unsafe { ffi::IsKeyReleased((key as u32) as i32) },
			}
		}
	
		fn cursor_delta(&self) -> Vector2 {
			unsafe { ffi::GetMouseDelta().into() }
		}
	
		fn cursor_pos_screen(&self) -> Vector2 {
			unsafe { ffi::GetMousePosition().into() }
		}
	
		fn scroll_delta(&self) -> Vector2 {
			(unsafe { ffi::GetMouseWheelMoveV() }).into()
		}
	
		fn scroll_delta_x(&self) -> f32 {
			unsafe { ffi::GetMouseWheelMove() }
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
		
		fn fill_text(&mut self, x: i32, y: i32, text: &str, size: i32, color: impl Into<Color>) {
			self.draw_text(text, x, y, size, color.into());
		}
		
		fn clip_region(&mut self, x: i32, y: i32, w: i32, h: i32) -> Option<impl RenderContext> {
			Some(self.begin_scissor_mode(x, y, w, h))
		}
	}

	impl MeasureContext for RaylibDrawHandle<'_> {
		
		fn calculate_text_size(&self, text: &str, size: i32) -> (i32, i32) {
			(self.measure_text(&text, size), size)
		}
	}

	impl RenderContext for RaylibMode2D<'_, RaylibDrawHandle<'_>> {
		fn fill_rect(&mut self, x: i32, y: i32, w: i32, h: i32, color: impl Into<Color>) {
			self.draw_rectangle(x, y, w, h, color.into());
		}

		fn outline_rect(&mut self, x: i32, y: i32, w: i32, h: i32, color: impl Into<Color>) {
			self.draw_rectangle_lines(x, y, w, h, color.into());
		}
		
		fn fill_text(&mut self, x: i32, y: i32, text: &str, size: i32, color: impl Into<Color>) {
			self.draw_text(text, x, y, size, color.into());
		}
		
		fn clip_region(&mut self, x: i32, y: i32, w: i32, h: i32) -> Option<impl RenderContext> {
			Some(self.begin_scissor_mode(x, y, w, h))
		}
	}

	impl MeasureContext for RaylibMode2D<'_, RaylibDrawHandle<'_>> {
		fn calculate_text_size(&self, text: &str, size: i32) -> (i32, i32) {
			(self.measure_text(&text, size), size)
		}
	}

	// impl <T> RenderContext for RaylibTextureMode<'_, T> {
	// 	fn fill_rect(&mut self, x: i32, y: i32, w: i32, h: i32, color: impl Into<Color>) {
	// 		self.draw_rectangle(x, y, w, h, color.into());
	// 	}

	// 	fn outline_rect(&mut self, x: i32, y: i32, w: i32, h: i32, color: impl Into<Color>) {
	// 		self.draw_rectangle_lines(x, y, w, h, color.into());
	// 	}
		
	// 	fn fill_text(&mut self, x: i32, y: i32, text: &str, size: i32, color: impl Into<Color>) {
	// 		self.draw_text(text, x, y, size, color.into());
	// 	}
		
	// 	fn clip_region(&mut self, x: i32, y: i32, w: i32, h: i32) -> impl RenderContext {
	// 		self.begin_scissor_mode(x, y, w, h)
	// 	}
	// }

	impl MeasureContext for RaylibHandle {
		fn calculate_text_size(&self, text: &str, size: i32) -> (i32, i32) {
			(self.measure_text(text, size), size)
		}
	}

	impl MeasureContext for WeakFont {
		fn calculate_text_size(&self, text: &str, size: i32) -> (i32, i32) {
			let size = self.measure_text(text, size as f32, 1.0);
			(size.x as i32, size.y as i32)
		}
	}

	pub trait AllowClipContext {}
	impl AllowClipContext for RaylibDrawHandle<'_> {}
	impl AllowClipContext for RaylibMode2D<'_, RaylibDrawHandle<'_>> {}
	impl AllowClipContext for RaylibScissorMode<'_, RaylibDrawHandle<'_>> {}
	impl <T> AllowClipContext for RaylibTextureMode<'_, T> {}

	impl RenderContext for RaylibScissorMode<'_, RaylibDrawHandle<'_>> {
		fn fill_rect(&mut self, x: i32, y: i32, w: i32, h: i32, color: impl Into<Color>) {
			self.draw_rectangle(x, y, w, h, color.into());
		}

		fn outline_rect(&mut self, x: i32, y: i32, w: i32, h: i32, color: impl Into<Color>) {
			self.draw_rectangle_lines(x, y, w, h, color.into());
		}
		
		fn fill_text(&mut self, x: i32, y: i32, text: &str, size: i32, color: impl Into<Color>) {
			self.draw_text(text, x, y, size, color.into());
		}
		
		fn clip_region(&mut self, x: i32, y: i32, w: i32, h: i32) -> Option<impl RenderContext> {
			Some(self.begin_scissor_mode(x, y, w, h))
		}
	}

	impl <T> RenderContext for RaylibScissorMode<'_, RaylibMode2D<'_, T>> {
		fn fill_rect(&mut self, x: i32, y: i32, w: i32, h: i32, color: impl Into<Color>) {
			self.draw_rectangle(x, y, w, h, color.into());
		}

		fn outline_rect(&mut self, x: i32, y: i32, w: i32, h: i32, color: impl Into<Color>) {
			self.draw_rectangle_lines(x, y, w, h, color.into());
		}
		
		fn fill_text(&mut self, x: i32, y: i32, text: &str, size: i32, color: impl Into<Color>) {
			self.draw_text(text, x, y, size, color.into());
		}
		
		fn clip_region(&mut self, x: i32, y: i32, w: i32, h: i32) -> Option<impl RenderContext> {
			Some(self.begin_scissor_mode(x, y, w, h))
		}
	}

	impl <T> RenderContext for RaylibScissorMode<'_, RaylibScissorMode<'_, RaylibMode2D<'_, T>>> {
		fn fill_rect(&mut self, x: i32, y: i32, w: i32, h: i32, color: impl Into<Color>) {
			self.draw_rectangle(x, y, w, h, color.into());
		}

		fn outline_rect(&mut self, x: i32, y: i32, w: i32, h: i32, color: impl Into<Color>) {
			self.draw_rectangle_lines(x, y, w, h, color.into());
		}
		
		fn fill_text(&mut self, x: i32, y: i32, text: &str, size: i32, color: impl Into<Color>) {
			self.draw_text(text, x, y, size, color.into());
		}
		
		fn clip_region(&mut self, x: i32, y: i32, w: i32, h: i32) -> Option<impl RenderContext> {
			None::<RaylibDrawHandle<'_>>
		}
	}

	impl RenderContext for RaylibScissorMode<'_, RaylibScissorMode<'_, RaylibDrawHandle<'_>>> {
		fn fill_rect(&mut self, x: i32, y: i32, w: i32, h: i32, color: impl Into<Color>) {
			self.draw_rectangle(x, y, w, h, color.into());
		}

		fn outline_rect(&mut self, x: i32, y: i32, w: i32, h: i32, color: impl Into<Color>) {
			self.draw_rectangle_lines(x, y, w, h, color.into());
		}
		
		fn fill_text(&mut self, x: i32, y: i32, text: &str, size: i32, color: impl Into<Color>) {
			self.draw_text(text, x, y, size, color.into());
		}
		
		fn clip_region(&mut self, x: i32, y: i32, w: i32, h: i32) -> Option<impl RenderContext> {
			None::<RaylibDrawHandle<'_>>
		}
	}

	pub struct RayViewContext {
		pub handle: RaylibHandle,
		pub thread: RaylibThread,
	}

	impl RayViewContext {
		pub fn new(title: &str, width: i32, height: i32) -> Self {
			let (mut rl, thread) = raylib::init()
				.size(width, height)
				.title(title)
				.resizable()
				.vsync()
				.build();

			rl.set_target_fps(60);

			Self {
				handle: rl,
				thread: thread,
			}
		}
	}
}