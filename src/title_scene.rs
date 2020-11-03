use scene::{SceneName, SceneChangeEvent, BoxedScene, Scene};
use cgmath::Point2;
use ggez::Context;
use ggez::graphics::{self, Text};
use ggez::event::{KeyCode, KeyMods};

pub struct TitleScene {
    text: Text,
    last_event: Option<SceneChangeEvent<SceneName>>,
}

impl<'a> TitleScene {
    pub fn new() -> BoxedScene<'a, SceneName, ()> {
        Box::new(TitleScene {
            text: Text::new("1. Skills Scene\n2. Tile Scene\n3. Statics Scene\n4. Hues Scene\n5. Map Scene\n6. Gump Scene\n7. Anim Scene\n8. TexMaps Scene\n9. World Scene"),
            last_event: None
        })
    }
}

impl Scene<SceneName, ()> for TitleScene {
    fn draw(&mut self, ctx: &mut Context, engine_data: &mut ()) {
        graphics::clear(ctx, graphics::BLACK);
        graphics::draw(ctx, &self.text, (Point2::new(0.0, 0.0), graphics::WHITE));
    }

    fn update(&mut self, ctx: &mut Context, engine_data: &mut ()) -> Option<SceneChangeEvent<SceneName>> {
        self.last_event.take()
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        keymods: KeyMods,
        repeat: bool,
        engine_data: &mut ()
    ) {
        self.last_event = match keycode {
            KeyCode::Escape => Some(SceneChangeEvent::PopScene),
            KeyCode::Key1 => Some(SceneChangeEvent::PushScene(SceneName::SkillsScene)),
            KeyCode::Key2 => Some(SceneChangeEvent::PushScene(SceneName::TileScene)),
            KeyCode::Key3 => Some(SceneChangeEvent::PushScene(SceneName::StaticsScene)),
            KeyCode::Key4 => Some(SceneChangeEvent::PushScene(SceneName::HuesScene)),
            KeyCode::Key5 => Some(SceneChangeEvent::PushScene(SceneName::MapScene)),
            KeyCode::Key6 => Some(SceneChangeEvent::PushScene(SceneName::GumpScene)),
            KeyCode::Key7 => Some(SceneChangeEvent::PushScene(SceneName::AnimScene)),
            KeyCode::Key8 => Some(SceneChangeEvent::PushScene(SceneName::TexMapsScene)),
            KeyCode::Key9 => Some(SceneChangeEvent::PushScene(SceneName::WorldScene)),
            _ => None
        }
    }
}
