
use raylib::{color::Color, ffi::{BeginTextureMode, Rectangle, RenderTexture}, math::Vector2, prelude::{RaylibDraw, RaylibScissorModeExt, RaylibTextureModeExt}, texture::{RaylibRenderTexture2D, RenderTexture2D}};

mod ray;

#[allow(unused)]
mod vld;

mod vec2;

#[allow(unused)]
mod style;

#[allow(unused)]
mod tag;

#[allow(unused)]
mod layout;

#[allow(unused)]
mod element;

use crate::{element::Element, layout::{Alignment, Flex, Layout, Spacing}, ray::context::{RayInputContext, RayViewContext}, style::{Border, Font, Padding, SidedSize, Style}, tag::Tag, vec2::Vec2i, vld::{InputContext, RenderContext}};
use crate::element::{ElementState};

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

#[allow(unused)]
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


// pub fn measure_text(text: &str, font_size: i32) -> Vector2 {
// 	let c_text = CString::new(text).unwrap();
// 	unsafe { ffi::MeasureTextEx(ffi::GetFontDefault(), c_text.as_ptr(), font_size as f32, 0.0) }
// }

struct Props {
	x: i32,
	y: i32,
}

enum Sample {
	A { props: Props },
	B { v: bool }
}

impl Sample {
	pub fn update(&mut self) {
		match self {
			Sample::A { props } => {
				props.x = 2;
			}
			Sample::B { v } => {
				*v = true;
			}
		}
	}

	pub fn print(&self) {
		match self {
			Sample::A { props } => {
				println!("{}", props.x)
			}
			Sample::B { v } => {
				println!("{}", v)
			}
		}
	}
}

fn main() {
	let screen_w = 800;
	let screen_h = 450;

	let mut ray = RayViewContext::new("amanita", screen_w, screen_h);
	let input = RayInputContext {};

	let mut s = Sample::A { props: Props{x: 0, y: 0} };
	let mut s2 = Sample::B { v: false };
	s2.update();
	s2.print();



    // let (mut rl, thread) = raylib::init()
    //     .size(800, 450)
    //     .title("cascading-oxide")
    //     .resizable()
    //     .vsync()
    //     .build();

    // rl.set_target_fps(60);

    let mut ui = Element {
        tag: Tag::Frame,
		state: ElementState::None,
		// layout: Layout::Sequential,
		layout: Flex::row()
			.item_gap(6)
			.axis_align(Alignment::Center)
			.spacing(Spacing::Between)
			.build(),
        elements: vec![
            Element {
				state: ElementState::None,
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
					max_width: None,
					max_height: None,
					background: None,
                },
            },
            
			
            // },
			Element {
				state: ElementState::Scroll { offset_y: 0.0 },
				tag: Tag::Frame,
				layout: Layout::Scroll,
				// layout: Layout::Flex { gap: 6, direction: FlexDirection::Row, is_reverse: false, axis_alignment: Alignment::Start, cross_alignment: Alignment::End, spacing: Spacing::None },
				elements: vec![
					Element {
						state: ElementState::None,
						tag: Tag::Frame,
						layout: Flex::row().build(),
						// state: State::new(),
						elements: vec![
							Element {
								state: ElementState::None,
								tag: Tag::Label {
									text: String::from("two thousand"),
									font: Font { size: 48 },
								},
									layout: Layout::Sequential,
								    elements: vec![],
								    style: Style {
								        padding: Some(Padding::new_uniform(3)),
								        border: Some(Border { color: Color::ORANGE, width: SidedSize::new_uniform(3) }),
										min_width: 0,
										min_height: 0,
										max_width: None,
										max_height: None,
										
										background: None,
								    },
							}
						],
						style: Style {
							padding: Some(Padding::new_uniform(3)),
							border: Some(Border { color: Color::ORANGE, width: SidedSize::new_uniform(3) }),
							min_width: 0,
							min_height: 0,
							max_width: None,
							max_height: None,
							
							background: Some(Color::GREEN),
						},
					},
				],
				style: Style {
					padding: Some(Padding::new_uniform(3)),
					border: Some(Border { color: Color::GREEN, width: SidedSize::new_uniform(3) }),
					min_width: 0,
					min_height: 0,
					max_width: Some(100),
					max_height: Some(400),
					background: None,
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
			max_width: None,
			max_height: None,
			background: None,
        },
    };

    while !ray.handle.window_should_close() {
        let w = ray.handle.get_screen_width();
        let h = ray.handle.get_screen_height();
        let mut d = ray.handle.begin_drawing(&ray.thread);
        {
            d.clear_background(Color::RAYWHITE);
            let _ = RenderBox {x: 10, y: 10, w: w - 20, h: h - 20};

			Element::render(&ui, None, &input, &d.get_font_default(), &mut d, );
			Element::update(&mut ui, &input, &d.get_font_default());
        }
    }

	// let virt_w = 400;
	// let virt_h = 400;
	// // let mut target = ray.new_clip_region(virt_w, virt_h).unwrap();

	// let mut ball_position = Vector2 { x: virt_w as f32/2.0, y: virt_h as f32/2.0 };
	// let mut ball_speed = Vector2 { x: 5.0, y: 4.0 };
	// let ball_radius = 20.0;

	// let mut rotation = 0.0;

	// let virt_w = virt_w as f32;
	// let virt_h = virt_h as f32;

	// let mut scroll_offset_y = 0.0;

	// while !ray.handle.window_should_close() {
    //     // Update
    //     //-----------------------------------------------------
    //     // Ball movement logic
    //     ball_position.x += ball_speed.x;
    //     ball_position.y += ball_speed.y;

    //     // Check walls collision for bouncing
    //     if (ball_position.x >= (virt_w - ball_radius)) || (ball_position.x <= ball_radius) {
	// 		ball_speed.x *= -1.0;
	// 	}

    //     if (ball_position.y >= (virt_h - ball_radius)) || (ball_position.y <= ball_radius) {
	// 		ball_speed.y *= -1.0;
	// 	}

    //     // Render texture rotation
    //     rotation += 0.5;
    //     //-----------------------------------------------------

	// 	let scroll_delta = ray.handle.scroll_delta();

	// 	scroll_offset_y += scroll_delta.y;

    //     // Draw our scene to the render texture
	// 	// {
	// 	// 	ray.handle.draw_texture_mode(&ray.thread, &mut target, |mut d| {
	// 	// 		d.clear_background(Color::GAINSBORO);
	// 	// 		d.draw_text("sample text", 10, scroll_offset_y as i32, 32, Color::BLACK);
	// 	// 		Element::render( &ui, &d.get_font_default(), &mut d, Some(Vec2i::new(0, scroll_offset_y as i32)));
				
	// 	// 	});
	// 	// 	let mut d = ray.handle.begin_texture_mode(&ray.thread, &mut target); 
	// 	// 	{
	// 	// 		// d.draw_rectangle(0, 0, 20, 20, Color::PURPLE);
	// 	// 		// d.draw_circle_v(ball_position, ball_radius, Color::REBECCAPURPLE);
	// 	// 	}
	// 	// }

	// 	{
	// 		// Draw render texture to main framebuffer
	// 		let mut d = ray.handle.begin_drawing(&ray.thread);
	// 		{
	// 			d.clear_background(Color::RAYWHITE);

	// 			d.fill_rect(20, 20, 200, 200, Color::GAINSBORO);
	// 			{
	// 				let mut d = d.begin_scissor_mode(20, 20, 200, 200);
	// 				d.draw_circle(100, 100 + scroll_offset_y as i32, 100.0, Color::REBECCAPURPLE);
	// 			}
	// 			// let tex = target.texture();
	// 			// d.draw_texture_pro(
	// 			// 	tex,
	// 			// 	Rectangle {
	// 			// 		x: 0.0,
	// 			// 		y: 0.0,
	// 			// 		width: tex.width as f32,
	// 			// 		height: -(tex.height as f32), // <-- flip vertically
	// 			// 	},
	// 			// 	Rectangle {
	// 			// 		x: 20.0,
	// 			// 		y: 20.0,
	// 			// 		width: tex.width as f32,
	// 			// 		height: tex.height as f32,
	// 			// 	},
	// 			// 	Vector2 { x: 0.0, y: 0.0 },
	// 			// 	0.0,
	// 			// 	Color::WHITE,
	// 			// );
	// 			// d.draw_texture(tex, 20, 20, Color::WHITE);
	// 			// d.draw_texture_pro(
	// 			// 	tex, 
	// 			// 	Rectangle {x: 0.0, y: 0.0, width: tex.width as f32, height: tex.height as f32}, 
	// 			// 	Rectangle {x: screen_w as f32 / 2.0, y: screen_h as f32 / 2.0, width: tex.width as f32, height: tex.height as f32}, 
	// 			// 	Vector2 {x: tex.width as f32 / 2.0, y: tex.height as f32 / 2.0}, 
	// 			// 	0.0, 
	// 			// 	Color::WHITE,
	// 			// );

	// 			d.draw_fps(10, 10);
	// 		}
	// 	}
	// }
}
