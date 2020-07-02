#![allow(dead_code)]

mod application;
mod colors;
mod noise_map;

use glutin_window::GlutinWindow as Window;
use piston::{
    ButtonEvent,
    RenderEvent,
    UpdateEvent,
    WindowSettings,
};
use piston::event_loop::{
    EventSettings,
    Events,
};
use opengl_graphics::{
    GlGraphics,
    OpenGL,
};

use application::Application;

const WINDOW_WIDTH: u32 = 1024;
const WINDOW_HEIGHT: u32 = 512;

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("LilFortress", [WINDOW_WIDTH, WINDOW_HEIGHT])
    .graphics_api(opengl)
    .exit_on_esc(true)
    .build()
    .expect("Could not create window");

    let mut application = Application::new(
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
        GlGraphics::new(opengl),
    );

    let mut events = Events::new(EventSettings::new());

    while let Some(event) = events.next(&mut window) {
        if let Some(args) = event.render_args() {
            application.render(&args);
        }

        if let Some(args) = event.update_args() {
            application.update(&args);
        }

        if let Some(args) = event.button_args() {
            application.handle(&args);
        }
    }
}
