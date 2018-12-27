use super::core::{Text, Scene, TextureCache};
use super::context::Context;
use std::collections::HashSet;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

use serde_json::{Value, Error};

pub enum GameAction {
    QUIT,
}

pub struct Game {
    current_scene: Scene,
    prev_buttons: HashSet<sdl2::mouse::MouseButton>,
}

impl Game {

    pub fn new() -> Game {
        Game {
            current_scene: Scene::new(),
            prev_buttons: HashSet::new(),
        }
    }

    pub fn set_scene(&mut self, scene: &Scene) {
        let data = serde_json::to_string(scene).unwrap();
        self.current_scene = serde_json::from_str(&data).unwrap();
    }

    pub fn update(&mut self, events: &mut sdl2::EventPump) -> Option<GameAction> {

        for event in events.poll_iter() {
            match event {
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } |
                    Event::Quit { .. } => return Some(GameAction::QUIT),
                    _ => {}
            }
        }

        // get a mouse state
        let state = events.mouse_state();

        // Create a set of pressed Keys.
        let buttons = state.pressed_mouse_buttons().collect();

        // Get the difference between the new and old sets.
        let new_buttons = &buttons - &self.prev_buttons;
        let old_buttons = &self.prev_buttons - &buttons;


        self.prev_buttons = buttons;

        // Update scene elements.
        for go in &mut self.current_scene.gameobjects {
            go.x -= 1;
        }

        None
    }

    pub fn render(&self, context: &mut Context, textures: &TextureCache) {
        context.canvas.set_draw_color(Color::RGB(0, 0, 0));
        context.canvas.clear();
        self.current_scene.render(&mut context.canvas);
        context.canvas.present();
    }
}


