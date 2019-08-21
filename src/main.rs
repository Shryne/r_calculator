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

const WINDOW_SIZE: (u32, u32) = (500, 500);

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
        glyph: GlyphCache<'a>
    }

    impl<'a> Button<'a> {
        pub fn new(area: Area) -> Button<'a> {
            Button {
                area,
                color: [0.1, 0.1, 0.1, 1.0],
                glyph: GlyphCache::new(
                    "C:/windows/fonts/calibri.ttf",
                    (),
                    TextureSettings::new()
                ).expect("Unable to load font")
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
                let size = 20;
                let width = self.glyph.width(size, "Hello").unwrap();
                Text::new_color([1.0, 1.0, 1.0, 1.0], size)
                    .draw(
                        "Hallo",
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

    let mut window: Window = WindowSettings::new("Rust Calculator", WINDOW_SIZE)
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut button = gui::Button::new(
        gui::Area::xywh(10, 10, 100, 100)
    );

    let mut gl = GlGraphics::new(opengl);
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            graphics::clear([0.0, 0.0, 0.0, 1.0], &mut gl);
            button.draw(&mut gl, &r);
        }
    }
}