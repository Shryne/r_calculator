extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };
use std::path::Path;

pub mod gui {
    use graphics::{Graphics, Context};
    use opengl_graphics::{GlGraphics, GlyphCache, TextureSettings};
    use piston::input::RenderArgs;
    use graphics::types::Rectangle;
    use graphics::text::Text;
    use graphics::{DrawState, Transformed};
    use graphics::character::CharacterCache;

    pub struct Pos {
        x: i32,
        y: i32
    }

    impl Pos {
        pub fn new(x: i32, y: i32) -> Pos {
            Pos { x, y }
        }
    }

    pub struct Size {
        w: i32,
        h: i32
    }

    impl Size {
        pub fn new(w: i32, h: i32) -> Size {
            Size { w, h }
        }
    }

    pub struct Area {
        pos: Pos,
        size: Size
    }

    impl Area {
        pub fn new(pos: Pos, size: Size) -> Area {
            Area {
                pos,
                size
            }
        }

        pub fn xywh(x: i32, y: i32, w: i32, h: i32) -> Area {
            Area::new(
                Pos::new(x, y),
                Size::new(w, h)
            )
        }

        fn x(&self) -> i32 {
            self.pos.x
        }

        fn y(&self) -> i32 {
            self.pos.y
        }

        fn w(&self) -> i32 {
            self.size.w
        }

        fn h(&self) -> i32 {
            self.size.h
        }
    }

    pub struct Button<'a> {
        area: Area,
        color: [f32;4],
        glyph: GlyphCache<'a>,
        text: String
    }

    impl<'a> Button<'a> {
        pub fn new(text: String, area: Area) -> Button<'a> {
            Button {
                area,
                color: [0.1, 0.1, 0.1, 1.0],
                glyph: GlyphCache::new(
                    "C:/windows/fonts/calibri.ttf",
                    (),
                    TextureSettings::new()
                ).expect("Unable to load font"),
                text
            }
        }

        pub fn draw(&mut self, gl: &mut GlGraphics, args: &RenderArgs) {
            gl.draw(args.viewport(), |c, gl| {
                graphics::rectangle(
                    self.color,
                    [
                        self.area.x() as f64,
                        self.area.y() as f64,
                        self.area.w() as f64,
                        self.area.h() as f64
                    ],
                    c.transform,
                    gl
                );
                let size: u32 = (self.area.h() as u32) / 3;
                let width = self.glyph.width(size, &self.text).unwrap();
                Text::new_color([1.0, 1.0, 1.0, 1.0], size)
                    .draw(
                        &self.text,
                        &mut self.glyph,
                        &DrawState::default(),
                        c.transform.trans(
                            self.area.x() as f64 + (self.area.w() as f64 - width) / 2.0,
                            self.area.y() as f64 + size as f64 + (self.area.h() as f64 - size as f64) / 2.0
                        ),
                        gl
                    ).unwrap();
            });
        }
    }
}

fn main() {
    let opengl = OpenGL::V4_5;
    let texts = [
        "1", "2", "3", "C", "=", "4", "5", "6", "+", "-", "7", "8", "9", "*", "/"
    ];
    const VARS_IN_ROW: i32 = 5;
    let rows: i32 = (texts.len() as i32 / VARS_IN_ROW);
    const BUTTON_SIZE: i32 = 100;
    const MARGIN: i32 = 10;

    let window_size: (u32, u32) = (
        (BUTTON_SIZE * VARS_IN_ROW + MARGIN * (VARS_IN_ROW + 1)) as u32,
        (BUTTON_SIZE * (rows + 1) + MARGIN * (rows + 2)) as u32
    );

    let mut window: Window = WindowSettings::new("Rust Calculator", window_size)
        .graphics_api(opengl)
        .exit_on_esc(true)
        .resizable(false)
        .build()
        .unwrap();

    let mut buttons: Vec<gui::Button> = Vec::new();

    for y in 0..rows {
        for x in 0..VARS_IN_ROW {
            buttons.push(
                gui::Button::new(
                    texts[(y * VARS_IN_ROW + x) as usize].to_string(),
                    gui::Area::xywh(
                        MARGIN * (x + 1) + BUTTON_SIZE * x,
                        MARGIN * (y + 2) + BUTTON_SIZE * (y + 1),
                        BUTTON_SIZE,
                        BUTTON_SIZE
                    )
                )
            );
        }
    };

    let mut gl = GlGraphics::new(opengl);
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            graphics::clear([0.0, 0.0, 0.0, 1.0], &mut gl);
            for i in 0..buttons.len() {
                buttons[i].draw(&mut gl, &r);
            };
        }
    }
}