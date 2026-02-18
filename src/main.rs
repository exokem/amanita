use raylib::{color::Color, prelude::{RaylibDraw}};
use crate::{ray::context, vld::RenderContext};

mod ray;
mod vld;

#[derive(Debug)]
enum Tag {
    // Structural frame with no semantic meaning
    Frame,

    // Button control
    Button,

    Label {text: String},

}

const PADDING: i32 = 6;
const GAP: i32 = 6;

#[derive(Debug)]
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

#[derive(Debug)]
struct Border {
    color: Color,
    width: SidedSize,
}

impl Border {
    pub fn render(&self, b: &RenderBox, context: &mut impl RenderContext) -> RenderBox {

        // Left
        context.fill_rect(b.x, b.y, self.width.l as i32, b.h, self.color);
        // Top
        context.fill_rect(b.x, b.y, b.w, self.width.t as i32, self.color);
        // Right
        context.fill_rect(b.x + b.w - self.width.r as i32, b.y, self.width.r as i32, b.h, self.color);
        // Bottom
        context.fill_rect(b.x, b.y + b.h - self.width.b as i32, b.w, self.width.b as i32, self.color);

        b.shrink(self.width.t as i32, self.width.r as i32, self.width.b as i32, self.width.l as i32)
    }
}

type Padding = SidedSize;

impl Padding {
    pub fn render(&self, b: &RenderBox, _: &mut impl RenderContext) -> RenderBox {
        b.shrink(self.t as i32, self.r as i32, self.b as i32, self.l as i32)
    }
}

#[derive(Debug)]
struct Style {
    padding: Option<Padding>,
    border: Option<Border>,
}

impl Tag {
    pub fn render(&self, b: &RenderBox, context: &mut impl RenderContext, element: &Element) {

        let mut area = *b;

        if let Some(style) = &element.style {
            if let Some(border) = &style.border {
                area = border.render(&area, context)
            }

            if let Some(padding) = &style.padding {
                area = padding.render(&area, context)
            }
        }        

        // context.outline_rect(b.x, b.y, b.w, b.h, Color::RED);

        if element.content.len() != 0 {
            let item_areas = area.split_x(element.content.len(), GAP);

            for i in 0..element.content.len() {
                let area = &item_areas[i];
                let el = &element.content[i];

                // context.outline_rect(area.x, area.y, area.w, area.h, Color::BLACK);

                el.tag.render(area, context, &el);
            }
        }
        

        match self {
            Self::Frame => {
                
            },
            Self::Button => {

            },
            Self::Label { text } => {

            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct RenderBox {
    x: i32,
    y: i32,
    w: i32,
    h: i32,
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
}

#[derive(Debug)]
struct Element {
    tag: Tag,
    content: Vec<Element>,
    style: Option<Style>,
}

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
        content: vec![
            Element {
                tag: Tag::Label {
                    text: String::from("test label")
                },
                content: vec![],
                style: Some(Style {
                    padding: Some(Padding::new_uniform(3)),
                    border: Some(Border { color: Color::ORANGE, width: SidedSize::new_uniform(3) })
                }),
            },
            Element {
                tag: Tag::Button,
                content: vec![

                ],
                style: Some(Style {
                    padding: Some(Padding::new_uniform(3)),
                    border: Some(Border { color: Color::RED, width: SidedSize::new_uniform(1) })
                }),
            }
        ],
        style: Some(Style {
            padding: Some(Padding::new_uniform(3)),
            border: Some(Border { color: Color::GREEN, width: SidedSize::new_uniform(3) })
        }),
    };

    while !rl.window_should_close() {
        let w = rl.get_screen_width();
        let h = rl.get_screen_height();
        let mut d = rl.begin_drawing(&thread);
        {
            d.clear_background(Color::RAYWHITE);
            let rb = RenderBox {x: 10, y: 10, w: w - 20, h: h - 20};

            ui.tag.render(&rb, &mut d, &ui);
        }
    }
}
