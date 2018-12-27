use std::path::Path;
use sdl2::rect::Point;
use std::time::Duration;

mod context;
mod core;
mod editor;
mod game;
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


fn run(font_path: &Path) {

    let mut loop_mode = LoopMode::EDITOR;
    let mut context = Context::new().unwrap();
    let texture_creator = context.canvas.texture_creator();

    let mut font = context.font_context.load_font(font_path, 24).unwrap();
    let mut textures = TextureCache::new(&texture_creator, font);
    let mut events = context.sdl_context.event_pump().unwrap();


    let mut editor = Editor::new();
    let mut game = Game::new();
    'running: loop {

        if loop_mode == LoopMode::EDITOR {
            match editor.update(&mut events) {
                Some(EditorAction::QUIT) => break 'running,
                Some(EditorAction::RUN) => {
                    loop_mode = LoopMode::GAME; 
                    game.set_scene(&editor.current_scene);    
                    continue
                },
                _ => {}
            }

            editor.render(&mut context, &mut textures);
        } else {

            match game.update(&mut events) {
                Some(GameAction::QUIT) => loop_mode = LoopMode::EDITOR,
                _ => {},
            }

            game.render(&mut context, &mut textures);
        }
        
        std::thread::sleep(Duration::from_millis(10));
    }

}

fn main() {
    let args: Vec<_> = env::args().collect();

    println!("linked sdl2_ttf: {}", sdl2::ttf::get_linked_version());

    if args.len() < 2 {
        println!("Usage: ./demo font.[ttf|ttc|fon]")
    } else {
        let path: &Path = Path::new(&args[1]);
        run(path);
    }

}

