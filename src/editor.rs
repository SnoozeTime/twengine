
use super::core::{Text, Scene, TextureCache, GameObject};
use super::context::Context;
use std::collections::HashSet;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};


pub enum EditorAction {
    QUIT,
    RUN,
    SWITCH_TO_ADD,
    SWITCH_TO_SELECT,
}

pub struct Button {
    // content
    label: Text,
    shape: Rect,
    shape_color: Color,

    // action
    action: EditorAction,
}


impl Button {

    pub fn new( content: String, shape: Rect, shape_color: Color, text_color: Color, action: EditorAction) -> Button {
        Button {label: Text { content, color: text_color }, shape, shape_color, action}
    }

    pub fn render<T: sdl2::render::RenderTarget>(&self, canvas: &mut sdl2::render::Canvas<T>, textures: &mut TextureCache) {
        canvas.set_draw_color(self.shape_color);
        canvas.fill_rect(self.shape);
        self.label.render(canvas, &mut textures.char_textures, self.shape.x, self.shape.y);
    }


    pub fn is_pressed(&self, x: i32, y: i32) -> bool {
        self.shape.contains_point(Point::new(x, y))
    }
}

#[derive(PartialEq, Debug)]
enum EditorState {
    ADD_RECT,
    SELECT,
}


pub struct Editor {
    debug_text: Text,
    buttons: Vec<Button>,
    pub current_scene: Scene,
    prev_buttons: HashSet<sdl2::mouse::MouseButton>,
    state: EditorState,
    
    selection: Option<usize>,
}

impl Editor {

    pub fn new() -> Editor {
        let mut buttons = Vec::new();
        buttons.push(Button::new(
                "A".to_string(),
                Rect::new(10, 0, 20, 20), 
                Color::RGB(255, 255, 255),
                Color::RGB(255, 255, 255),
                EditorAction::SWITCH_TO_ADD));
        buttons.push(Button::new(
                "S".to_string(),
                Rect::new(40, 0, 20, 20), 
                Color::RGB(255, 255, 255),
                Color::RGB(255, 255, 255),
                EditorAction::SWITCH_TO_SELECT));
        Editor {
            current_scene: Scene::new(),
            debug_text: Text { content: String::new(), color: Color::RGB(255, 0, 0) },
            prev_buttons: HashSet::new(),
            buttons,
            state: EditorState::ADD_RECT,
            selection: None,
        }
    }

    pub fn update(&mut self, events: &mut sdl2::EventPump) -> Option<EditorAction> {

        for event in events.poll_iter() {
            match event {
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } |
                    Event::Quit { .. } => return Some(EditorAction::QUIT),
                    Event::KeyDown { keycode: Some(Keycode::Space), .. } => return Some(EditorAction::RUN),
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

        let current_color = Color::RGB(0, 0, 255);
        if !new_buttons.is_empty() {
            let mut button_clicked = false;
            // detect if click on  editor button.
            for button in &self.buttons {
                if button.is_pressed(state.x(), state.y()) {
                    button_clicked = true;
                    match button.action {
                    EditorAction::QUIT => return Some(EditorAction::QUIT),
                    EditorAction::RUN => return Some(EditorAction::RUN),
                    EditorAction::SWITCH_TO_SELECT => self.state = EditorState::SELECT,
                    EditorAction::SWITCH_TO_ADD => self.state = EditorState::ADD_RECT,
                    }
                }
            }

            if !button_clicked {
                
                if self.state == EditorState::ADD_RECT {
                    // If not click on a button, execute whatever action 
                    self.current_scene.add_rect(state.x(), state.y(), 20, 20, current_color.clone());
                } else if self.state == EditorState::SELECT {
                    let mut found_selection = false;
                    for (idx, go) in self.current_scene.gameobjects.iter().enumerate() {
                        if go.contains_point(state.x(), state.y()) {
                            self.selection = Some(idx);
                            found_selection = true;
                            break;
                        }
                    }

                    if !found_selection {
                        self.selection = None;
                    }
                }
            }

            
        }

        self.prev_buttons = buttons;
        self.debug_text.content = format!("x:{} y:{} state: {:?}", state.x(), state.y(), self.state);

        None
    }

    pub fn render(&self, context: &mut Context, mut textures: &mut TextureCache) {
        let mut canvas = &mut context.canvas;
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        for button in self.buttons.iter() {
            button.render(canvas, &mut textures);
        }

        self.debug_text.render(canvas, &mut textures.char_textures, 70, 0);
        self.current_scene.render(canvas);

        if let Some(idx) = self.selection {
            let go = &self.current_scene.gameobjects[idx];
            let rect = Rect::new(go.x, go.y, go.w, go.h);
            canvas.set_draw_color(Color::RGB(255, 255, 255));
            canvas.draw_rect(rect);
        }

        canvas.present();
    }
}


