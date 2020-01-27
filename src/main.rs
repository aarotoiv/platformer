extern crate piston_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate find_folder;

mod player;
mod renderer;

use player::Player;
use renderer::Renderer;

use piston_window::*;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{
    Button, Key, PressEvent, ReleaseEvent, RenderEvent, UpdateEvent,
};
use piston::window::WindowSettings;

fn main() {
    let opengl = OpenGL::V3_2;

    let window_width = 1600;
    let window_height = 900;


    let mut window: PistonWindow = WindowSettings::new("meme", [window_width, window_height])
        .samples(16)
        .graphics_api(opengl)
        .resizable(false)
        .exit_on_esc(true)
        .vsync(true)
        .build()
        .unwrap();

    let mut render = Renderer {
        gl: GlGraphics::new(opengl),
    };

    let player = Player::new(&window_width, &window_height);


    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(u) = e.update_args() {
            println!("UPDATE");
        }
        if let Some(r) = e.render_args() {
            render.render(&r, &player);
        }
        if let Some(b) = e.press_args() {
            handle_press(&b);
        }
        if let Some(b) = e.release_args() {
            handle_release(&b);
        }
    }
}
fn handle_press(args: &Button) {
    if let &Button::Keyboard(key) = args {
        match key {
            Key::A => {
                println!("A DOWN");
            }
            Key::D => {
                println!("D DOWN");
            }
            Key::W => {
                println!("W DOWN");
            }
            Key::S => {
                println!("S DOWN");
            }
            _ => {}
        }
    }
}

fn handle_release(args: &Button) {
    if let &Button::Keyboard(key) = args {
        match key {
            Key::A => {
                println!("A RELEASED");
            }
            Key::D => {
                println!("D RELEASED");
            }
            Key::W => {
                println!("W RELEASED");
            }
            Key::S => {
                println!("S RELEASED");
            }
            _ => {}
        }
    }
}
