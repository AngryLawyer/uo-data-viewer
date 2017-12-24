use scene::{BoxedScene, SceneStack, SceneChangeEvent};
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
}
