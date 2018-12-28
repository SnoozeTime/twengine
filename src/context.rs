use sdl2::{Sdl, VideoSubsystem};
use sdl2::render::WindowCanvas;
use sdl2::ttf::Sdl2TtfContext;

pub struct Context {
    pub sdl_context: Sdl,
    video_subsystem: VideoSubsystem,
    pub canvas: WindowCanvas,
    pub font_context: Sdl2TtfContext,
}

impl Context {

    pub fn new() -> Result<Context, Box<std::error::Error>> {
        let sdl_context = sdl2::init()?; 
        let video_subsystem = sdl_context.video()?;
        let font_context = sdl2::ttf::init()?;


        let window = video_subsystem.window("Mouse", 800, 600)
            .position_centered()
            .build()?;

        let canvas = window.into_canvas().build()?;
       
        Ok(Context { sdl_context, video_subsystem, canvas, font_context,})
    }
}


