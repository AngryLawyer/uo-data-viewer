use scene::{BoxedScene, Scene, SceneChangeEvent, SceneName};
use engine::EngineData;
use text_renderer::TextRenderer;
use sdl2::pixels::Color;
use std::io::Result;
use std::path::Path;

use sdl2::render::{Renderer, Texture, TextureQuery};
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub struct AnimScene {
    page: Option<Texture>,
}

impl AnimScene {
    pub fn new<'a>(renderer: &mut Renderer, engine_data: &mut EngineData<'a>) -> BoxedScene<SceneName, EngineData<'a>> {
        Box::new(AnimScene {
            page: None,
        })
    }
}

impl<SceneName, EngineData> Scene<SceneName, EngineData> for AnimScene {
    fn render(&self, renderer: &mut Renderer, engine_data: &mut EngineData) {
        renderer.clear();
        renderer.present();
    }

    fn handle_event(&mut self, event: &Event, renderer: &mut Renderer, engine_data: &mut EngineData) -> Option<SceneChangeEvent<SceneName>> {
        match *event {
            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                Some(SceneChangeEvent::PopScene)
            },
             _ => None
        }
    }
}
