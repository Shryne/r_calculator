use opengl_graphics::{GlGraphics, GlyphCache, TextureSettings};
use piston::input::RenderArgs;
use graphics::text::Text;
use graphics::{DrawState, Transformed};
use graphics::character::CharacterCache;
use piston::input;
use input::Key;

use super::unit::*;

/// A visual object
pub trait Shape {
    /// Draws the element
    fn draw(&mut self, gl: &mut GlGraphics, args: &RenderArgs);

    fn on_event(&mut self, event: (&input::Button, i32, i32), is_press: bool);
}

pub struct Button<'a> {
    area: Area,
    color: [f32;4],
    glyph: GlyphCache<'a>,
    text: String,
    pressed: bool
}

impl<'a> Button<'a> {
    pub fn new<S, A>(text: S, area: A) -> Button<'a>
        where S: ToString,
              A: Into<Area> {
        Button {
            area: area.into(),
            color: [0.1, 0.1, 0.1, 1.0],
            glyph: GlyphCache::new(
                "C:/windows/fonts/calibri.ttf",
                (),
                TextureSettings::new()
            ).expect("Unable to load font"),
            text: text.to_string(),
            pressed: false
        }
    }
}

impl<'a> Shape for Button<'a> {
    fn draw(&mut self, gl: &mut GlGraphics, args: &RenderArgs) {
        gl.draw(args.viewport(), |c, gl| {
            let draw_color;
            if self.pressed {
                draw_color = [0.2, 0.2, 0.2, 1.0]
            } else {
                draw_color = self.color
            }
            graphics::rectangle(
                draw_color,
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

    fn on_event(&mut self, event: (&input::Button, i32, i32), is_press: bool) {
        let (button, x, y) = event;
        if let input::Button::Mouse(key) = *button {
            match key {
                input::MouseButton::Left => {
                    if self.area.x() <= x
                        && x <= self.area.x() + self.area.w()
                        && self.area.y() <= y
                        && y <= self.area.y() + self.area.h() {

                        self.pressed = is_press;
                    }
                },
                _ => {}
            }
        }
    }
}

pub struct TextField<'a> {
    area: Area,
    color: [f32;4],
    glyph: GlyphCache<'a>,
    text: String
}

impl<'a> TextField<'a> {
    pub fn new(area: impl Into<Area>) -> TextField<'a> {
        TextField {
            area: area.into(),
            color: [0.1, 0.1, 0.1, 1.0],
            glyph: GlyphCache::new(
                "C:/windows/fonts/calibri.ttf",
                (),
                TextureSettings::new()
            ).expect("Unable to load font"),
            text: "0".to_string()
        }
    }
}

impl<'a> Shape for TextField<'a> {
    fn draw(&mut self, gl: &mut GlGraphics, args: &RenderArgs) {
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

            graphics::line_from_to(
                [0.1, 0.1, 0.1, 1.0],
                1.0,
                [335.0, 120.0],
                [335.0, 440.0],
                c.transform,
                gl
            )
        });
    }

    fn on_event(&mut self, event: (&input::Button, i32, i32), is_press: bool) {

    }
}