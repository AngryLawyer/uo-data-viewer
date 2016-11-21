use scene::{Scene, BoxedScene, SceneChangeEvent, SceneName};
use text_renderer::TextRenderer;
use sdl2::pixels::Color;
use sdl2::render::{Renderer, Texture, TextureQuery};
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2_ttf::Font;

pub struct TitleScene {
    text: Texture
}

impl TitleScene {
    pub fn new(text_renderer: &TextRenderer, renderer: &mut Renderer) -> BoxedScene<SceneName> {
        Box::new(TitleScene {
           text: text_renderer.create_text_texture(renderer, "1. Skills Scene\n2. Tile Scene", Color::RGBA(255, 255, 255, 255))
        })
    }
}

impl Scene<SceneName> for TitleScene {

    fn render(&self, renderer: &mut Renderer) {
        let TextureQuery {width, height, .. } = self.text.query();
        let target = Rect::new(0, 0, width, height);
        renderer.clear();
        renderer.copy(&self.text, None, Some(target)).unwrap();
        renderer.present();
    }

    fn handle_event(&mut self, event: &Event) -> Option<SceneChangeEvent<SceneName>> {
        match *event {
            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                Some(SceneChangeEvent::PopScene)
            },
            Event::KeyDown { keycode: Some(Keycode::Num1), .. } => {
                Some(SceneChangeEvent::PushScene(SceneName::SkillsScene))
            },
            Event::KeyDown { keycode: Some(Keycode::Num2), .. } => {
                Some(SceneChangeEvent::PushScene(SceneName::TileScene))
            },
             _ => None
        }
    }
}
