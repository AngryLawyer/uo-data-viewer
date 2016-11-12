use sdl2::render::{Renderer};
use sdl2::event::Event;
pub type BoxedScene<T> = Box<Scene<T> + 'static>;

pub trait Scene<T> {
    fn render(&self, renderer: &mut Renderer);
    fn handle_event(&self, event: &Event) -> Option<SceneChangeEvent<T>>;
}

pub enum SceneName {
    TitleScene,
    SkillsScene, 
}

pub enum SceneChangeEvent<T> {
    PushScene(T),
    SwapScene(T),
    PopScene,
}

pub struct SceneStack<T> {
    scenes: Vec<BoxedScene<T>>
}

impl<T> SceneStack<T> {

    pub fn new() -> SceneStack<T> {
        SceneStack {
            scenes: vec![]
        }
    }

    pub fn is_empty(&self) -> bool {
        self.scenes.len() == 0
    }

    pub fn push(&mut self, scene: BoxedScene<T>) {
        self.scenes.push(scene)
    }

    pub fn swap(&mut self, scene: BoxedScene<T>) -> Option<BoxedScene<T>> {
        let old_scene = self.scenes.pop();
        self.scenes.push(scene);
        old_scene
    }

    pub fn pop(&mut self) -> Option<BoxedScene<T>> {
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

    pub fn handle_event(&mut self, event: &Event) -> Option<SceneChangeEvent<T>> {
        let maybe_last_scene = self.scenes.pop();
        match maybe_last_scene {
            Some(scene) => {
                let event = scene.handle_event(event);
                self.scenes.push(scene);
                event
            },
            None => None
        }
    }
}
