use ggez::timer;
use scene::{SceneName, SceneStack, SceneChangeEvent, BoxedScene};
use title_scene;
use skills_scene;
use tile_scene;
use statics_scene;
use hues_scene;
use texmaps_scene;
use gump_scene;
use anim_scene;
use ggez::{graphics, Context, ContextBuilder, GameResult};
use ggez::event::{self, EventHandler, quit, KeyCode, KeyMods, MouseButton};
pub struct Engine<'a> {
    scene_stack: Option<SceneStack<'a, SceneName, ()>>,
}

impl<'a> Engine<'a> {
    pub fn new(_ctx: &mut Context) -> Engine<'a> {
        let mut scene_stack = SceneStack::new();
        scene_stack.push(title_scene::TitleScene::new());
        Engine {
            scene_stack: Some(scene_stack)
        }
    }

    pub fn scenebuilder(&mut self, ctx: &mut Context, sceneName: SceneName) -> BoxedScene<'a,  SceneName, ()> {
        match sceneName {
            SceneName::TitleScene => {
                title_scene::TitleScene::new()
            },
            SceneName::SkillsScene => {
                skills_scene::SkillsScene::new()
            },
            SceneName::TileScene => {
                tile_scene::TileScene::new(ctx)
            },
            SceneName::StaticsScene => {
                statics_scene::StaticsScene::new(ctx)
            },
            SceneName::HuesScene => {
                hues_scene::HuesScene::new(ctx)
            },
            SceneName::TexMapsScene => {
                texmaps_scene::TexMapsScene::new(ctx)
            },
            SceneName::GumpScene => {
                gump_scene::GumpScene::new(ctx)
            },
            SceneName::AnimScene => {
                anim_scene::AnimScene::new(ctx)
            },
            _ => {
                title_scene::TitleScene::new()
            }
            /*
            scene::SceneName::MapScene => {
                map_scene::MapScene::new(engine_data, texture_creator)
            },
            scene::SceneName::GumpScene => {
                gump_scene::GumpScene::new(engine_data, texture_creator)
            },
            scene::SceneName::WorldScene => {
                world_scene::WorldScene::new(engine_data, texture_creator)
            },*/
        
        }
    }
}

impl<'a> EventHandler for Engine<'a> {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        let mut scene_stack = self.scene_stack.take().unwrap();
        // Update code here...
        if (scene_stack.is_empty()) {
            quit(ctx);
        } else {
            let scene_event = scene_stack.update(ctx, &mut ());
            match scene_event {
                Some(SceneChangeEvent::PopScene) => {
                    scene_stack.pop();
                },
                Some(SceneChangeEvent::PushScene(scene)) => {
                    scene_stack.push(self.scenebuilder(ctx, scene))
                },
                Some(SceneChangeEvent::SwapScene(scene)) => {
                    scene_stack.swap(self.scenebuilder(ctx, scene));
                },
                _ => ()
            }
        }
        self.scene_stack = Some(scene_stack);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let mut scene_stack = self.scene_stack.take().unwrap();
        scene_stack.draw(ctx, &mut ());
        self.scene_stack = Some(scene_stack);
        graphics::present(ctx)?;
        timer::yield_now();
        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        keymods: KeyMods,
        repeat: bool
    ) {
        let mut scene_stack = self.scene_stack.take().unwrap();
        scene_stack.key_down_event(ctx, keycode, keymods, repeat, &mut ());
        self.scene_stack = Some(scene_stack);
    }

    fn mouse_button_down_event(
        &mut self,
        ctx: &mut Context,
        button: MouseButton,
        x: f32,
        y: f32
    ) {
        let mut scene_stack = self.scene_stack.take().unwrap();
        scene_stack.mouse_button_down_event(ctx, button, x, y, &mut ());
        self.scene_stack = Some(scene_stack);
    }
}
