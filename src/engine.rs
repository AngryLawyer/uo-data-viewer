use sdl2::EventPump;
use scene::SceneStack;
use sdl2_engine_helpers::game_loop::GameLoop;


pub struct Engine<T> {
    game_loop: GameLoop,
    event_pump: EventPump,
    scene_stack: SceneStack<T>,
}

impl<T> Engine<T> {
    pub fn new(fps: u32, event_pump: EventPump) -> Engine<T> {
        Engine {
            game_loop: GameLoop::new(fps),
            scene_stack: SceneStack::new(),
            event_pump: event_pump
        }
    }
}
