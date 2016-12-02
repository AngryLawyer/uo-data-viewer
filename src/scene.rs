use sdl2::render::{Renderer};
use sdl2::event::Event;
pub type BoxedScene<SceneChangeParamsT, EngineDataT> = Box<Scene<SceneChangeParamsT, EngineDataT> + 'static>;

pub trait Scene<SceneChangeParamsT, EngineDataT> {
    fn render(&self, renderer: &mut Renderer, engine_data: &mut EngineDataT);
    fn handle_event(&mut self, event: &Event, renderer: &mut Renderer, engine_data: &mut EngineDataT) -> Option<SceneChangeEvent<SceneChangeParamsT>>;
}

// FIXME: This should live elsewhere
pub enum SceneName {
    TitleScene,
    SkillsScene,
    TileScene,
    StaticsScene,
    HuesScene,
}

pub enum SceneChangeEvent<T> {
    PushScene(T),
    SwapScene(T),
    PopScene,
}

pub struct SceneStack<SceneChangeParamsT, EngineDataT> {
    scenes: Vec<BoxedScene<SceneChangeParamsT, EngineDataT>>
}

impl<SceneChangeParamsT, EngineDataT> SceneStack<SceneChangeParamsT, EngineDataT> {

    pub fn new() -> SceneStack<SceneChangeParamsT, EngineDataT> {
        SceneStack {
            scenes: vec![]
        }
    }

    pub fn is_empty(&self) -> bool {
        self.scenes.len() == 0
    }

    pub fn push(&mut self, scene: BoxedScene<SceneChangeParamsT, EngineDataT>) {
        self.scenes.push(scene)
    }

    pub fn swap(&mut self, scene: BoxedScene<SceneChangeParamsT, EngineDataT>) -> Option<BoxedScene<SceneChangeParamsT, EngineDataT>> {
        let old_scene = self.scenes.pop();
        self.scenes.push(scene);
        old_scene
    }

    pub fn pop(&mut self) -> Option<BoxedScene<SceneChangeParamsT, EngineDataT>> {
        self.scenes.pop()
    }

    pub fn render(&mut self, renderer: &mut Renderer, engine_data: &mut EngineDataT) {
        let maybe_last_scene = self.scenes.pop();
        match maybe_last_scene {
            Some(scene) => {
                scene.render(renderer, engine_data);
                self.scenes.push(scene);
            },
            None => ()
        }
    }

    pub fn handle_event(&mut self, event: &Event, renderer: &mut Renderer, engine_data: &mut EngineDataT) -> Option<SceneChangeEvent<SceneChangeParamsT>> {
        let maybe_last_scene = self.scenes.pop();
        match maybe_last_scene {
            Some(mut scene) => {
                let event = scene.handle_event(event, renderer, engine_data);
                self.scenes.push(scene);
                event
            },
            None => None
        }
    }
}
