extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

mod gui;
use gui::shape::Shape;
use gui::unit::*;

fn main() {
    let opengl = OpenGL::V4_5;
    let texts = [
        "1", "2", "3", "=", "C", "4", "5", "6", "+", "-", "7", "8", "9", "*", "/"
    ];
    const VARS_IN_ROW: i32 = 5;
    let rows: i32 = texts.len() as i32 / VARS_IN_ROW;
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

    let mut buttons: Vec<gui::shape::Button> = Vec::new();

    for y in 0..rows {
        for x in 0..VARS_IN_ROW {
            buttons.push(
                gui::shape::Button::new(
                    texts[(y * VARS_IN_ROW + x) as usize].to_string(),
                    Area::xywh(
                        MARGIN * (x + 1) + BUTTON_SIZE * x,
                        MARGIN * (y + 2) + BUTTON_SIZE * (y + 1),
                        BUTTON_SIZE,
                        BUTTON_SIZE
                    )
                )
            );
        }
    };

    let mut text_field = gui::shape::TextField::new(
        Area::xywh(
            MARGIN,
            MARGIN,
            MARGIN * (VARS_IN_ROW - 1) + BUTTON_SIZE * VARS_IN_ROW,
            BUTTON_SIZE
        )
    );
    let mut gl = GlGraphics::new(opengl);
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            graphics::clear([0.0, 0.0, 0.0, 1.0], &mut gl);
            text_field.draw(&mut gl, &r);
            for i in 0..buttons.len() {
                buttons[i].draw(&mut gl, &r);
            };
        }
    }
}