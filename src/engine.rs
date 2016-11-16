use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::render::Renderer;
use scene::{BoxedScene, SceneStack, SceneChangeEvent};
use sdl2_engine_helpers::game_loop::GameLoop;


pub struct Engine<T> {
    game_loop: GameLoop,
    event_pump: Option<EventPump>,
    scene_stack: Option<SceneStack<T>>,
}

impl<T> Engine<T> {
    pub fn new(fps: u32, event_pump: EventPump, initial_scene: BoxedScene<T>) -> Engine<T> {
        let mut scene_stack = SceneStack::new();
        scene_stack.push(initial_scene);
        Engine {
            game_loop: GameLoop::new(fps),
            scene_stack: Some(scene_stack),
            event_pump: Some(event_pump)
        }
    }

    pub fn run<F>(&mut self, mut scenebuilder: F, renderer: &mut Renderer)
        where F: FnMut(T, &mut Renderer) -> BoxedScene<T> {
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
                        let scene_event = scene_stack.handle_event(&event);
                        match scene_event {
                            Some(SceneChangeEvent::PopScene) => {
                                scene_stack.pop();
                            },
                            Some(SceneChangeEvent::PushScene(scene)) => {
                                scene_stack.push(scenebuilder(scene, renderer))
                            },
                            Some(SceneChangeEvent::SwapScene(scene)) => {
                                scene_stack.pop();
                                scene_stack.push(scenebuilder(scene, renderer))
                            },
                            _ => ()
                        }
                    }
                }
            }
            scene_stack.render(renderer);
            ended || scene_stack.is_empty()
        });
        self.event_pump = Some(event_pump);
        self.scene_stack = Some(scene_stack);
    }
}
