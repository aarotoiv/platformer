extern crate piston_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate find_folder;

mod player;
mod renderer;
mod world;

use player::Player;
use renderer::Renderer;
use world::World;

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

    let mut player = Player::new(&window_width, &window_height);
    let mut world = World::new();
    world.initialize();

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(u) = e.update_args() {
            player.update(&u);
            player.handle_collisions(&world);
        }
        if let Some(r) = e.render_args() {
            render.render(&r, &player, &world);
        }
        if let Some(b) = e.press_args() {
            handle_press(&b, &mut player);
        }
        if let Some(b) = e.release_args() {
            handle_release(&b, &mut player);
        }
    }
}
fn handle_press(args: &Button, player: &mut Player) {
    if let &Button::Keyboard(key) = args {
        match key {
            Key::A => {
                player.handle_press(Key::A);
            }
            Key::D => {
                player.handle_press(Key::D);
            }
            _ => {}
        }
    }
}

fn handle_release(args: &Button, player: &mut Player) {
    if let &Button::Keyboard(key) = args {
        match key {
            Key::A => {
                player.handle_release(Key::A);
            }
            Key::D => {
                player.handle_release(Key::D);
            }
            _ => {}
        }
    }
}
