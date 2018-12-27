use sdl2::render::{Canvas, Texture, RenderTarget, TextureQuery, TextureCreator};
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use std::collections::HashMap;

use serde_derive::{Serialize, Deserialize};

#[macro_export]
macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);

pub struct Text {
    pub content: String,
    pub color: Color,
}

impl Text {
    pub fn render<T: RenderTarget>(&self, canvas: &mut Canvas<T>, fonts: &mut HashMap<char, Texture>, x: i32, y: i32) {

        let mut tx = x;
        for (i, c) in self.content.char_indices() {
            let mut texture: &mut Texture = fonts.get_mut(&c).expect(&format!("no '{}' in map", c));
            texture.set_color_mod(self.color.r,
                                  self.color.g,
                                  self.color.b);
            let TextureQuery { width, height, .. } = texture.query();
            let target = rect!(tx, y, width, height);

            tx += 2+(width as i32);
            canvas.copy(&texture, None, Some(target)).unwrap();
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct GameObject {
    // position
    pub x: i32,
    pub y: i32,
    pub w: u32,
    pub h: u32,

    r: u8,
    g: u8,
    b: u8,
}

impl GameObject {
    pub fn new(x: i32, y: i32, w: u32, h: u32, r: u8, g: u8, b: u8) -> GameObject {
        GameObject {
            x,
            y,
            w,
            h,
            r,
            g,
            b,
        }
    }

    pub fn render<T: RenderTarget>(&self, canvas: &mut Canvas<T>) {
        canvas.set_draw_color(Color::RGB(self.r, self.g, self.b));
        canvas.fill_rect(rect!(self.x, self.y, self.w, self.h));
    }

    pub fn contains_point(&self, x: i32, y: i32) -> bool {
        rect!(self.x, self.y, self.w, self.h).contains_point(Point::new(x, y))
    }    
}

#[derive(Serialize, Deserialize)]
pub struct Scene {
    pub gameobjects: Vec<GameObject>,
}

impl Scene {
    pub fn new() -> Scene {
        Scene { gameobjects: Vec::new() }
    }

    pub fn add_rect(&mut self, x: i32, y: i32, w: u32, h: u32, color: Color) {
        self.gameobjects.push(GameObject::new(x, y, w, h, color.r, color.g, color.b));
    }

    pub fn render<T: RenderTarget>(&self, canvas: &mut Canvas<T>) {
        for go in self.gameobjects.iter() {
            go.render(canvas);
        }
    }

}


pub struct TextureCache<'a> {
    pub char_textures: HashMap<char, Texture<'a>>,
}

impl<'a> TextureCache<'a> {

    pub fn new<T>(texture_creator: &'a TextureCreator<T>, font: sdl2::ttf::Font) -> TextureCache<'a> {
        let mut char_textures = HashMap::new();
        // instead of creating textures every frame for rendering text, I am
        // going to store the characters texture in a big map and render the
        // text from these textures.
        let chars = ['0','1', '2', '3', '4', '5', '6', '7', '8', '9', 'x', 'y', ':', ' ', '_',
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q',
        'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n',
        'o', 'p', 'q', 'r', 's', 't','u', 'v', 'w', 'x', 'y', 'z'];
        for c in chars.iter() {
            let surface = font.render(&c.to_string()).blended(Color::RGBA(255, 0, 0, 255)).unwrap();
            let texture = texture_creator.create_texture_from_surface(&surface).unwrap();
            char_textures.insert(*c, texture);
        }

        TextureCache { char_textures }
    }
}


