use scene::{Scene, BoxedScene, SceneChangeEvent, SceneName};
use engine::EngineData;
use text_renderer::TextRenderer;
use sdl2::pixels::Color;
use sdl2::render::{WindowCanvas, Texture, TextureQuery};
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::ttf::Font;

pub struct TitleScene {
}

impl TitleScene {
    pub fn new<'a>(renderer: &mut WindowCanvas, engine_data: &mut EngineData<'a>) -> BoxedScene<SceneName, EngineData<'a>> {
        Box::new(TitleScene {
        })
    }
}

impl<'a> Scene<SceneName, EngineData<'a>> for TitleScene {

    fn render(&self, renderer: &mut WindowCanvas, engine_data: &mut EngineData) {
        //let TextureQuery {width, height, .. } = self.text.query();
        //let target = Rect::new(0, 0, width, height);
        renderer.clear();
        //renderer.copy(&self.text, None, Some(target)).unwrap();
        renderer.present();
    }

    fn handle_event(&mut self, event: &Event, renderer: &mut WindowCanvas, engine_data: &mut EngineData) -> Option<SceneChangeEvent<SceneName>> {
        match *event {
            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                Some(SceneChangeEvent::PopScene)
            },
            /*Event::KeyDown { keycode: Some(Keycode::Num1), .. } => {
                Some(SceneChangeEvent::PushScene(SceneName::SkillsScene))
            },
            Event::KeyDown { keycode: Some(Keycode::Num2), .. } => {
                Some(SceneChangeEvent::PushScene(SceneName::TileScene))
            },
            Event::KeyDown { keycode: Some(Keycode::Num3), .. } => {
                Some(SceneChangeEvent::PushScene(SceneName::StaticsScene))
            },
            Event::KeyDown { keycode: Some(Keycode::Num4), .. } => {
                Some(SceneChangeEvent::PushScene(SceneName::HuesScene))
            },
            Event::KeyDown { keycode: Some(Keycode::Num5), .. } => {
                Some(SceneChangeEvent::PushScene(SceneName::MapScene))
            },
            Event::KeyDown { keycode: Some(Keycode::Num6), .. } => {
                Some(SceneChangeEvent::PushScene(SceneName::GumpScene))
            },
            Event::KeyDown { keycode: Some(Keycode::Num7), .. } => {
                Some(SceneChangeEvent::PushScene(SceneName::AnimScene))
            },*/
             _ => None
        }
    }
}
