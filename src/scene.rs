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

    pub fn push(&mut self, scene: BoxedScene) {
        self.scenes.push(scene)
    }

    pub fn swap(&mut self, scene: BoxedScene) -> Option<BoxedScene> {
        let old_scene = self.scenes.pop();
        self.scenes.push(scene);
        old_scene
    }

    pub fn pop(&mut self) -> Option<BoxedScene> {
        self.scenes.pop()
    }

    pub fn render(&mut self, renderer: &mut Renderer) {
        let maybe_last_scene = self.scenes.pop();
        match maybe_last_scene {
            Some(scene) => {
                scene.render(renderer);
                self.scenes.push(scene);
            },
            None => ()
        }
    }

    pub fn handle_event(&mut self, event: &Event) {
        let maybe_last_scene = self.scenes.pop();
        match maybe_last_scene {
            Some(scene) => {
                scene.handle_event(event);
                self.scenes.push(scene);
            },
            None => ()
        }
    }
}
