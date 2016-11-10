use scene::{Scene, BoxedScene, SceneChangeEvent};
use sdl2::pixels::Color;
use sdl2::render::{Renderer, Texture, TextureQuery};
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2_ttf::Font;

pub struct TitleScene {
    pub text: Texture
}

impl TitleScene {
    pub fn new<T>(font: &Font, renderer: &mut Renderer) -> BoxedScene<T> {
        let surface = font.render("1. ").blended(Color::RGBA(255, 255, 255, 255)).unwrap();
        let texture = renderer.create_texture_from_surface(&surface).unwrap();

        Box::new(TitleScene {
           text: texture
        })
    }
}

impl<T> Scene<T> for TitleScene {

    fn render(&self, renderer: &mut Renderer) {
        let TextureQuery {width, height, .. } = self.text.query();
        let target = Rect::new(0, 0, width, height);
        renderer.clear();
        renderer.copy(&self.text, None, Some(target)).unwrap();
        renderer.present();
    }

    fn handle_event(&self, event: &Event) -> Option<SceneChangeEvent<T>> {
        match *event {
            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                Some(SceneChangeEvent::PopScene)
            },
            Event::KeyDown { keycode: Some(Keycode::Num1), .. } => {
                println!("1");
                None
            },
             _ => None
        }
    }
}
