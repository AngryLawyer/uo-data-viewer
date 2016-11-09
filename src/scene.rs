use sdl2::render::{Renderer};
use sdl2::event::Event;
pub type BoxedScene = Box<Scene + 'static>;

pub trait Scene {
    fn render(&self, renderer: &mut Renderer);
    fn handle_event(&self, event: &Event);
}

pub struct SceneStack {
    scenes: Vec<BoxedScene>
}

impl SceneStack {

    pub fn new() -> SceneStack {
        SceneStack {
            scenes: vec![]
        }
    }

    pub fn push_scene(&mut self, scene: BoxedScene) {
        self.scenes.push(scene)
    }

    pub fn swap_scene(&mut self, scene: BoxedScene) -> Option<BoxedScene> {
        let old_scene = self.scenes.pop();
        self.scenes.push(scene);
        old_scene
    }

    pub fn pop_scene(&mut self) -> Option<BoxedScene> {
        self.scenes.pop()
    }

    pub fn render(&self, renderer: &mut Renderer) {
    }

    pub fn handle_event(&self, event: &Event) {
    }
}
