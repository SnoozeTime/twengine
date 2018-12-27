use std::env;
use sdl2::{Sdl, VideoSubsystem};
use sdl2::rect::Rect;
use std::path::Path;
use std::collections::HashMap;

use sdl2::render::{Canvas, RenderTarget, Texture, TextureQuery, WindowCanvas, TextureCreator};
use sdl2::video::WindowContext;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::collections::HashSet;
use std::time::Duration;
use sdl2::pixels::Color;

macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);

struct Text {
    content: String,
}

impl Text {
    fn render<T: RenderTarget>(&self, canvas: &mut Canvas<T>, fonts: &HashMap<char, Texture>, x: i32, y: i32) {

        let mut tx = x;
        for (i, c) in self.content.char_indices() {
            let texture = fonts.get(&c).expect(&format!("no '{}' in map", c));
            let TextureQuery { width, height, .. } = texture.query();
            let target = rect!(tx, y, width, height);

            tx += 2+(width as i32);
            canvas.copy(&texture, None, Some(target)).unwrap();
        }
    }
}

struct GameObject {
    // position
    x: i32,
    y: i32,
    w: i32,
    h: i32,

    color: Color,
}

impl GameObject {
    pub fn new(x: i32, y: i32, w: i32, h: i32, color: Color) -> GameObject {
        GameObject {
            x,
            y,
            w,
            h,
            color,
        }
    }

    pub fn render<T: RenderTarget>(&self, canvas: &mut Canvas<T>) {
        canvas.set_draw_color(self.color);
        canvas.fill_rect(rect!(self.x, self.y, self.w, self.h));
    }
}

struct Scene {
    gameobjects: Vec<GameObject>,
}

impl Scene {
    pub fn new() -> Scene {
        Scene { gameobjects: Vec::new() }
    }

    pub fn add_rect(&mut self, x: i32, y: i32, w: i32, h: i32, color: Color) {
        self.gameobjects.push(GameObject::new(x, y, w, h, color));
    }

    pub fn render<T: RenderTarget>(&self, canvas: &mut Canvas<T>) {
        for go in self.gameobjects.iter() {
            go.render(canvas);
        }
    }
}

#[derive(PartialEq)]
enum LoopMode {
    EDITOR,
    GAME,
}

enum EditorAction {
    QUIT,
    RUN,
}

struct Editor {
    debug_text: Text,
    current_scene: Scene,
    prev_buttons: HashSet<sdl2::mouse::MouseButton>,
    running: bool,
}

impl Editor {

    pub fn new() -> Editor {
        Editor {
            current_scene: Scene::new(),
            debug_text: Text { content: String::new() },
            prev_buttons: HashSet::new(),
            running: true,
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
            self.current_scene.add_rect(state.x(), state.y(), 20, 20, current_color.clone());
        }

        self.prev_buttons = buttons;

        self.debug_text.content = format!("x:{} y:{}", state.x(), state.y());

        None
    }

    pub fn render(&self, context: &mut Context, textures: &TextureCache) {
        context.canvas.set_draw_color(Color::RGB(0, 0, 0));
        context.canvas.clear();
        self.debug_text.render(&mut context.canvas, &textures.char_textures, 0, 0);
        self.current_scene.render(&mut context.canvas);
        context.canvas.present();
    }
}

struct Context {
    sdl_context: Sdl,
    video_subsystem: VideoSubsystem,
    canvas: WindowCanvas,
    font_context: sdl2::ttf::Sdl2TtfContext,
}

impl Context {

    fn new() -> Result<Context, Box<std::error::Error>> {
        let sdl_context = sdl2::init()?; 
        let video_subsystem = sdl_context.video()?;
        let font_context = sdl2::ttf::init()?;


        let window = video_subsystem.window("Mouse", 800, 600)
            .position_centered()
            .build()?;

        let canvas = window.into_canvas().build()?;
        let texture_creator = canvas.texture_creator();
        
       
        Ok(Context { sdl_context, video_subsystem, canvas, font_context,})
    }
}

struct TextureCache<'a> {
    char_textures: HashMap<char, Texture<'a>>,
}

impl<'a> TextureCache<'a> {

    pub fn new<T>(texture_creator: &'a TextureCreator<T>, font: sdl2::ttf::Font) -> TextureCache<'a> {
        let mut char_textures = HashMap::new();
        // instead of creating textures every frame for rendering text, I am
        // going to store the characters texture in a big map and render the
        // text from these textures.
        let chars = ['0','1', '2', '3', '4', '5', '6', '7', '8', '9', 'x', 'y', ':', ' '];
        for c in chars.iter() {
            let surface = font.render(&c.to_string()).blended(Color::RGBA(255, 0, 0, 255)).unwrap();
            let texture = texture_creator.create_texture_from_surface(&surface).unwrap();
            char_textures.insert(*c, texture);
        }

        TextureCache { char_textures }
    }
}

fn run(font_path: &Path) {

    let mut loop_mode = LoopMode::EDITOR;
    let mut context = Context::new().unwrap();
    let texture_creator = context.canvas.texture_creator();

    let mut font = context.font_context.load_font(font_path, 24).unwrap();

    let mut textures = TextureCache::new(&texture_creator, font);
    
    let mut events = context.sdl_context.event_pump().unwrap();


    let mut scene = Scene::new();
    let mut current_color = Color::RGB(0, 0, 255);

    let mut editor = Editor::new();
    'running: loop {

        if loop_mode == LoopMode::EDITOR {
            match editor.update(&mut events) {
                Some(EditorAction::QUIT) => break 'running,
                Some(EditorAction::RUN) => {loop_mode = LoopMode::GAME; continue },
                None => {}
            }

            editor.render(&mut context, &textures);
        } else {

            println!("IN THE GAME");
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
