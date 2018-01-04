use scene::SceneName;
use engine::EngineData;
use sdl2::pixels::Color;
use sdl2::render::{WindowCanvas, Texture, TextureQuery, TextureCreator};
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::video::WindowContext;
use sdl2_engine_helpers::scene::{Scene, BoxedScene, SceneChangeEvent};

pub struct TitleScene<'b> {
    text: Texture<'b>,
    last_event: Option<SceneChangeEvent<SceneName>>,
}

impl<'b> TitleScene<'b> {
    pub fn new<'a>(engine_data: &mut EngineData<'a>, texture_creator: &'b TextureCreator<WindowContext>) -> BoxedScene<'b, Event, SceneName, EngineData<'a>> {
        Box::new(TitleScene {
            text: engine_data.text_renderer.create_text_texture(texture_creator, "1. Skills Scene\n2. Tile Scene\n3. Statics Scene\n4. Hues Scene\n5. Map Scene\n6. Gump Scene\n7. Anim Scene", Color::RGBA(255, 255, 255, 255)),
            last_event: None
        })
    }
}

impl<'a, 'b> Scene<Event, SceneName, EngineData<'a>> for TitleScene<'b> {

    fn render(&self, renderer: &mut WindowCanvas, _engine_data: &EngineData, _tick: u64) {
        let TextureQuery {width, height, .. } = self.text.query();
        let target = Rect::new(0, 0, width, height);
        renderer.clear();
        renderer.copy(&self.text, None, Some(target)).unwrap();
        renderer.present();
    }

    fn handle_event(&mut self, event: &Event, _engine_data: &mut EngineData, _tick: u64) {
        if self.last_event.is_none() {
            self.last_event = match *event {
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    Some(SceneChangeEvent::PopScene)
                },
                Event::KeyDown { keycode: Some(Keycode::Num1), .. } => {
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
                }, /*
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

    fn think(&mut self, _engine_data: &mut EngineData, _tick: u64) -> Option<SceneChangeEvent<SceneName>> {
        self.last_event.take()
    }
}
