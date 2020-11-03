use ggez::timer;
use scene::{SceneName, SceneStack, SceneChangeEvent, BoxedScene};
use title_scene;
use skills_scene;
/*use sdl2_engine_helpers::scene::{BoxedScene, SceneStack, SceneChangeEvent};
use sdl2::event::Event;
use sdl2::EventPump;
use sdl2::render::WindowCanvas;
use sdl2_engine_helpers::game_loop::GameLoop;

use text_renderer::TextRenderer;
// TODO: Move EngineData into its own hoojama
pub struct EngineData<'a> {
    pub text_renderer: TextRenderer<'a>,
}

impl<'a> EngineData<'a> {
    pub fn new(text_renderer: TextRenderer<'a>) -> EngineData<'a> {
        EngineData {
            text_renderer,
        }
    }
}

pub struct Engine<'a, EventT, SceneChangeParamsT, EngineDataT> {
    game_loop: GameLoop,
    event_pump: Option<EventPump>,
    scene_stack: Option<SceneStack<'a, EventT, SceneChangeParamsT, EngineDataT>>,
}

impl<'a, SceneChangeParamsT, EngineDataT> Engine<'a, Event, SceneChangeParamsT, EngineDataT> {
    pub fn new(fps: u32, event_pump: EventPump, initial_scene: BoxedScene<Event, SceneChangeParamsT, EngineDataT>) -> Engine<Event, SceneChangeParamsT, EngineDataT> {
        let mut scene_stack = SceneStack::new();
        scene_stack.push(initial_scene);
        Engine {
            game_loop: GameLoop::new(fps),
            scene_stack: Some(scene_stack),
            event_pump: Some(event_pump)
        }
    }

    pub fn run<F>(&mut self, mut scenebuilder: F, renderer: &mut WindowCanvas, engine_data: &mut EngineDataT)
        where F: FnMut(SceneChangeParamsT, &mut EngineDataT) -> BoxedScene<'a, Event, SceneChangeParamsT, EngineDataT> {
        let mut event_pump = self.event_pump.take().unwrap();
        let mut scene_stack = self.scene_stack.take().unwrap();
        self.game_loop.run(|frame| {
            let mut ended = false;
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..}  => {
                        ended = true
                    },
                    _ => {
                        scene_stack.handle_event(&event, engine_data, frame);
                    }
                }
            }
            let scene_event = scene_stack.think(engine_data, frame);
            match scene_event {
                Some(SceneChangeEvent::PopScene) => {
                    scene_stack.pop();
                },
                Some(SceneChangeEvent::PushScene(scene)) => {
                    scene_stack.push(scenebuilder(scene, engine_data))
                },
                Some(SceneChangeEvent::SwapScene(scene)) => {
                    scene_stack.swap(scenebuilder(scene, engine_data));
                },
                _ => ()
            }
            scene_stack.render(renderer, engine_data, frame);
            ended || scene_stack.is_empty()
        });
        self.event_pump = Some(event_pump);
        self.scene_stack = Some(scene_stack);
    }
}*/
use ggez::{graphics, Context, ContextBuilder, GameResult};
use ggez::event::{self, EventHandler, quit, KeyCode, KeyMods};
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

    pub fn scenebuilder(&mut self, sceneName: SceneName) -> BoxedScene<'a,  SceneName, ()> {
        match sceneName {
            SceneName::TitleScene => {
                title_scene::TitleScene::new()
            },
            SceneName::SkillsScene => {
                skills_scene::SkillsScene::new()
            },
            _ => {
                title_scene::TitleScene::new()
            }
            /*
            scene::SceneName::TileScene => {
                tile_scene::TileScene::new(engine_data, texture_creator)
            },
            scene::SceneName::StaticsScene => {
                statics_scene::StaticsScene::new(engine_data, texture_creator)
            },
            scene::SceneName::HuesScene => {
                hues_scene::HuesScene::new(engine_data, texture_creator)
            },
            scene::SceneName::MapScene => {
                map_scene::MapScene::new(engine_data, texture_creator)
            },
            scene::SceneName::GumpScene => {
                gump_scene::GumpScene::new(engine_data, texture_creator)
            },
            scene::SceneName::AnimScene => {
                anim_scene::AnimScene::new(engine_data, texture_creator)
            },
            scene::SceneName::TexMapsScene => {
                texmaps_scene::TexMapsScene::new(engine_data, texture_creator)
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
                    scene_stack.push(self.scenebuilder(scene))
                },
                Some(SceneChangeEvent::SwapScene(scene)) => {
                    scene_stack.swap(self.scenebuilder(scene));
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
}
