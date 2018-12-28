use std::time::{Duration, Instant};
use std::path::Path;

mod context;
mod core;
mod editor;
mod game;
mod math;
mod physics;
use std::env;

use self::context::Context;
use self::core::TextureCache;
use self::editor::{EditorAction, Editor};
use self::game::{GameAction, Game};

#[derive(PartialEq)]
enum LoopMode {
    EDITOR,
    GAME,
}

pub fn run(font_path: &Path) {

    let mut loop_mode = LoopMode::EDITOR;
    let mut context = Context::new().unwrap();
    let texture_creator = context.canvas.texture_creator();

    let font = context.font_context.load_font(font_path, 24).unwrap();
    let mut textures = TextureCache::new(&texture_creator, font);
    let mut events = context.sdl_context.event_pump().unwrap();


    let mut editor = Editor::new();
    let mut game = Game::new();

    let fixed_time_stamp = Duration::new(0, 16666667); 
    let mut previous_clock = Instant::now();
    let mut accumulator = Duration::new(0, 0);

    'running: loop {

        if loop_mode == LoopMode::EDITOR {
            match editor.update(&mut events) {
                Some(EditorAction::QUIT) => break 'running,
                Some(EditorAction::RUN) => {
                    loop_mode = LoopMode::GAME; 
                    game.set_scene(&editor.current_scene);
                    previous_clock = Instant::now();
                    continue
                },
                _ => {}
            }

            editor.render(&mut context, &mut textures);
        std::thread::sleep(Duration::from_millis(10));
        } else {

            accumulator += Instant::now() - previous_clock;
            previous_clock = Instant::now();

            while accumulator > fixed_time_stamp {
                accumulator -= fixed_time_stamp;
                match game.update(&mut events, fixed_time_stamp.subsec_millis()) {
                    Some(GameAction::QUIT) => loop_mode = LoopMode::EDITOR,
                    _ => {},
                }
            }

            game.render(&mut context, &mut textures);
            std::thread::sleep(fixed_time_stamp);
        }
        
    }

}


