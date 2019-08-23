use opengl_graphics::{GlGraphics, GlyphCache, TextureSettings};
use piston::input::RenderArgs;
use graphics::text::Text;
use graphics::{DrawState, Transformed};
use graphics::character::CharacterCache;

use super::unit::*;

/// A visual object
pub trait Shape {
    /// Draws the element
    fn draw(&mut self, gl: &mut GlGraphics, args: &RenderArgs);
}

pub struct Button<'a> {
    area: Area,
    color: [f32;4],
    glyph: GlyphCache<'a>,
    text: String
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
            text: text.to_string()
        }
    }
}

impl<'a> Shape for Button<'a> {
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
        });
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
}