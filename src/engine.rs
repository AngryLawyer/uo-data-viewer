use ggez::event::{EventHandler, MouseButton};
use ggez::input::keyboard::KeyInput;
use ggez::{Context, GameError, GameResult, timer};

use crate::gump_scene;
use crate::hues_scene;
use crate::map_scene;
use crate::scene::{BoxedScene, SceneChangeEvent, SceneName, SceneStack};
use crate::skills_scene;
use crate::statics_scene;
use crate::tile_scene;
use crate::title_scene;
/*
use anim_scene;
use font_scene;
use map_diff_scene;
use texmaps_scene;
use world_scene;*/

pub struct Engine<'a> {
    scene_stack: Option<SceneStack<'a, SceneName, ()>>,
}

impl<'a> Engine<'a> {
    pub fn new(_ctx: &mut Context) -> Engine<'a> {
        let mut scene_stack = SceneStack::new();
        scene_stack.push(title_scene::TitleScene::new());
        Engine {
            scene_stack: Some(scene_stack),
        }
    }

    pub fn scene_builder(
        &mut self,
        ctx: &mut Context,
        scene_name: SceneName,
    ) -> BoxedScene<'a, SceneName, ()> {
        match scene_name {
            SceneName::TitleScene => title_scene::TitleScene::new(),
            SceneName::SkillsScene => skills_scene::SkillsScene::new(),
            SceneName::TileScene => tile_scene::TileScene::new(ctx),
            SceneName::StaticsScene => statics_scene::StaticsScene::new(ctx),
            SceneName::HuesScene => hues_scene::HuesScene::new(ctx),
            SceneName::MapScene => map_scene::MapScene::new(ctx),
            SceneName::GumpScene => gump_scene::GumpScene::new(ctx),
            _ => panic!("OOP"), /*
                                SceneName::TexMapsScene => texmaps_scene::TexMapsScene::new(ctx),
                                SceneName::AnimScene => anim_scene::AnimScene::new(ctx),
                                SceneName::WorldScene => world_scene::WorldScene::new(),
                                SceneName::FontScene => font_scene::FontScene::new(ctx),
                                SceneName::MapDiffScene => map_diff_scene::MapDiffScene::new(ctx),*/
        }
    }
}

impl<'a> EventHandler for Engine<'a> {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        let mut scene_stack = self
            .scene_stack
            .take()
            .ok_or_else(|| GameError::EventLoopError("Empty scene stack".to_owned()))?;
        // Update code here...
        if scene_stack.is_empty() {
            ctx.request_quit();
        } else {
            let scene_event = scene_stack.update(ctx, &mut ())?;
            match scene_event {
                Some(SceneChangeEvent::PopScene) => {
                    scene_stack.pop();
                }
                Some(SceneChangeEvent::PushScene(scene)) => {
                    scene_stack.push(self.scene_builder(ctx, scene))
                }
                Some(SceneChangeEvent::SwapScene(scene)) => {
                    scene_stack.swap(self.scene_builder(ctx, scene));
                }
                _ => (),
            }
        }
        self.scene_stack = Some(scene_stack);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut scene_stack = self
            .scene_stack
            .take()
            .ok_or_else(|| GameError::EventLoopError("Empty scene stack".to_owned()))?;
        scene_stack.draw(ctx, &mut ())?;
        self.scene_stack = Some(scene_stack);
        timer::yield_now();
        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keyinput: KeyInput,
        repeat: bool,
    ) -> GameResult {
        let mut scene_stack = self.scene_stack.take().expect("Empty scene stack");
        scene_stack.key_down_event(ctx, keyinput, repeat, &mut ());
        self.scene_stack = Some(scene_stack);
        Ok(())
    }

    fn mouse_button_down_event(
        &mut self,
        ctx: &mut Context,
        button: MouseButton,
        x: f32,
        y: f32,
    ) -> GameResult {
        let mut scene_stack = self.scene_stack.take().expect("Empty scene stack");
        scene_stack.mouse_button_down_event(ctx, button, x, y, &mut ());
        self.scene_stack = Some(scene_stack);
        Ok(())
    }
}
