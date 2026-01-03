use ggez::input::keyboard::{KeyCode, KeyInput};
use ggez::graphics::{Canvas, Color, DrawParam, Text};
use ggez::{Context, GameResult};
use crate::scene::{BoxedScene, Scene, SceneChangeEvent, SceneName};

pub struct TitleScene {
    text: Text,
    last_event: Option<SceneChangeEvent<SceneName>>,
}

impl<'a> TitleScene {
    pub fn new() -> BoxedScene<'a, SceneName, ()> {
        Box::new(TitleScene {
            text: Text::new("1. Skills Scene\n2. Tile Scene\n3. Statics Scene\n4. Hues Scene\n5. Map Scene\n6. Gump Scene\n7. Anim Scene\n8. TexMaps Scene\n9. World Scene\n0. Font Scene\nA. Map Diff Scene"),
            last_event: None
        })
    }
}

impl Scene<SceneName, ()> for TitleScene {
    fn draw(&mut self, ctx: &mut Context, _engine_data: &mut ()) -> GameResult {
        let mut canvas = Canvas::from_frame(ctx, Color::BLACK);
        canvas.draw(&self.text, DrawParam::default().color(Color::WHITE));
        canvas.finish(ctx)
    }

    fn update(
        &mut self,
        _ctx: &mut Context,
        _engine_data: &mut (),
    ) -> GameResult<Option<SceneChangeEvent<SceneName>>> {
        Ok(self.last_event.take())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keyinput: KeyInput,
        _repeat: bool,
        _engine_data: &mut (),
    ) {
        self.last_event = match keyinput.keycode {
            Some(KeyCode::Escape) => Some(SceneChangeEvent::PopScene),
            Some(KeyCode::Key1) => Some(SceneChangeEvent::PushScene(SceneName::SkillsScene)),
            Some(KeyCode::Key2) => Some(SceneChangeEvent::PushScene(SceneName::TileScene)),
            Some(KeyCode::Key3) => Some(SceneChangeEvent::PushScene(SceneName::StaticsScene)),
            Some(KeyCode::Key4) => Some(SceneChangeEvent::PushScene(SceneName::HuesScene)),
            Some(KeyCode::Key5) => Some(SceneChangeEvent::PushScene(SceneName::MapScene)),
            Some(KeyCode::Key6) => Some(SceneChangeEvent::PushScene(SceneName::GumpScene)),
            Some(KeyCode::Key7) => Some(SceneChangeEvent::PushScene(SceneName::AnimScene)),
            Some(KeyCode::Key8) => Some(SceneChangeEvent::PushScene(SceneName::TexMapsScene)),
            Some(KeyCode::Key9) => Some(SceneChangeEvent::PushScene(SceneName::WorldScene)),
            Some(KeyCode::Key0) => Some(SceneChangeEvent::PushScene(SceneName::FontScene)),
            Some(KeyCode::A) => Some(SceneChangeEvent::PushScene(SceneName::MapDiffScene)),
            _ => None,
        }
    }
}
