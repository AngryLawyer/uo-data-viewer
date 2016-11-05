use scene::{Scene, BoxedScene};
use sdl2::pixels::Color;
use sdl2::render::{Renderer, Texture, TextureQuery};
use sdl2::rect::Rect;
use sdl2_ttf::Font;

pub struct TitleScene {
    pub text: Texture
}

impl TitleScene {
    pub fn new(font: &Font, renderer: &mut Renderer) -> BoxedScene {
        let surface = font.render("1. ").blended(Color::RGBA(255, 255, 255, 255)).unwrap();
        let texture = renderer.create_texture_from_surface(&surface).unwrap();
        
        Box::new(TitleScene {
           text: texture 
        })
    }
}

impl Scene for TitleScene {

    fn render(&self, renderer: &mut Renderer) {
        let TextureQuery {width, height, .. } = self.text.query();
        let target = Rect::new(0, 0, width, height);
        renderer.clear();
        renderer.copy(&self.text, None, Some(target)).unwrap();
        renderer.present();
    }
}
