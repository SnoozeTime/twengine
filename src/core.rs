use sdl2::render::{Canvas, Texture, RenderTarget, TextureQuery, TextureCreator};
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use std::collections::HashMap;
use super::math::Vector2d;

use serde_derive::{Serialize, Deserialize};

use super::physics::{BodyType, RigidBody};

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
        for (_, c) in self.content.char_indices() {
            let texture: &mut Texture = fonts.get_mut(&c).expect(&format!("no '{}' in map", c));
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
    pub body: RigidBody,

    r: u8,
    g: u8,
    b: u8,
}

impl GameObject {

    pub fn new(x: f32, y: f32, w: u32, h: u32, r: u8, g: u8, b: u8, body_type: BodyType) -> GameObject {
        let body = RigidBody::new(x, y, w, h, body_type);
        GameObject {
            body,
            r,
            g,
            b,
        }
    }

    pub fn render<T: RenderTarget>(&self, canvas: &mut Canvas<T>, camera: &Camera) {
        canvas.set_draw_color(Color::RGB(self.r, self.g, self.b));

        let global_pos = Vector2d::new(self.body.position.x as i32, self.body.position.y as i32);
        let local_pos = camera.toLocal(global_pos);
        canvas.fill_rect(rect!(local_pos.x, local_pos.y,
                               self.body.shape.w, self.body.shape.h)).expect("GameObject render failed");
    }

    pub fn contains_point(&self, x: i32, y: i32) -> bool {
        rect!(self.body.position.x, self.body.position.y,
              self.body.shape.w, self.body.shape.h).contains_point(Point::new(x, y))
    }    

    pub fn x(&self) -> f32 {
        self.body.position.x
    }

    pub fn y(&self) -> f32 {
        self.body.position.y
    }

    pub fn w(&self) -> u32 {
        self.body.shape.w
    }

    pub fn h(&self) -> u32 {
        self.body.shape.h
    }
}

#[derive(Serialize, Deserialize)]
pub struct Scene {
    pub gameobjects: Vec<GameObject>,
    pub camera: Camera,
}

impl Scene {
    pub fn new() -> Scene {
        Scene { gameobjects: Vec::new(), camera: Camera::new() }
    }

    pub fn add_rect(&mut self, x: f32, y: f32, w: u32, h: u32, color: Color) {
        self.gameobjects.push(GameObject::new(x, y, w, h, color.r, color.g, color.b, BodyType::Dynamic));
    }

    pub fn add_static(&mut self, x: f32, y: f32, w: u32, h: u32, color: Color) {
        self.gameobjects.push(GameObject::new(x, y, w, h, color.r, color.g, color.b, BodyType::Static));
    }


    pub fn render<T: RenderTarget>(&self, canvas: &mut Canvas<T>) {
        for go in self.gameobjects.iter() {
            go.render(canvas, &self.camera);
        }
    }

}

#[derive(Debug, Serialize, Deserialize)]
pub struct Camera {
    pub position: Vector2d<i32>,
}

impl Camera {

    pub fn new() -> Camera {
        Camera { position: Vector2d::new(0, 0)}
    }

    pub fn toLocal(&self, global_pos: Vector2d<i32>) -> Vector2d<i32> {
        global_pos - self.position
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


