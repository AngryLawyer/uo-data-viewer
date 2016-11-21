use sdl2::render::{Renderer};
use sdl2::event::Event;
pub type BoxedScene<SceneChangeParamsT> = Box<Scene<SceneChangeParamsT> + 'static>;

pub trait Scene<SceneChangeParamsT> {
    fn render(&self, renderer: &mut Renderer);
    fn handle_event(&mut self, event: &Event) -> Option<SceneChangeEvent<SceneChangeParamsT>>;
}

pub enum SceneName {
    TitleScene,
    SkillsScene,
    TileScene,
}

pub enum SceneChangeEvent<T> {
    PushScene(T),
    SwapScene(T),
    PopScene,
}

pub struct SceneStack<SceneChangeParamsT> {
    scenes: Vec<BoxedScene<SceneChangeParamsT>>
}

impl<SceneChangeParamsT> SceneStack<SceneChangeParamsT> {

    pub fn new() -> SceneStack<SceneChangeParamsT> {
        SceneStack {
            scenes: vec![]
        }
    }

    pub fn is_empty(&self) -> bool {
        self.scenes.len() == 0
    }

    pub fn push(&mut self, scene: BoxedScene<SceneChangeParamsT>) {
        self.scenes.push(scene)
    }

    pub fn swap(&mut self, scene: BoxedScene<SceneChangeParamsT>) -> Option<BoxedScene<SceneChangeParamsT>> {
        let old_scene = self.scenes.pop();
        self.scenes.push(scene);
        old_scene
    }

    pub fn pop(&mut self) -> Option<BoxedScene<SceneChangeParamsT>> {
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

    pub fn handle_event(&mut self, event: &Event) -> Option<SceneChangeEvent<SceneChangeParamsT>> {
        let maybe_last_scene = self.scenes.pop();
        match maybe_last_scene {
            Some(mut scene) => {
                let event = scene.handle_event(event);
                self.scenes.push(scene);
                event
            },
            None => None
        }
    }
}
