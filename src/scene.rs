use sdl2::render::{WindowCanvas};
use sdl2::event::Event;
pub type BoxedScene<'a, SceneChangeParamsT, EngineDataT> = Box<Scene<SceneChangeParamsT, EngineDataT> + 'a>;

pub trait Scene<SceneChangeParamsT, EngineDataT> {
    fn render(&self, renderer: &mut WindowCanvas, engine_data: &mut EngineDataT);
    fn handle_event(&mut self, event: &Event, renderer: &mut WindowCanvas, engine_data: &mut EngineDataT) -> Option<SceneChangeEvent<SceneChangeParamsT>>;
}

// FIXME: This should live elsewhere
pub enum SceneName {
    TitleScene,
    SkillsScene,
    TileScene,
/*    StaticsScene,
    HuesScene,
    MapScene,
    GumpScene,
    AnimScene*/
}

pub enum SceneChangeEvent<T> {
    PushScene(T),
    SwapScene(T),
    PopScene,
}

pub struct SceneStack<'a, SceneChangeParamsT, EngineDataT> {
    scenes: Vec<BoxedScene<'a, SceneChangeParamsT, EngineDataT>>
}

impl<'a, SceneChangeParamsT, EngineDataT> SceneStack<'a, SceneChangeParamsT, EngineDataT> {

    pub fn new() -> SceneStack<'a, SceneChangeParamsT, EngineDataT> {
        SceneStack {
            scenes: vec![]
        }
    }

    pub fn is_empty(&self) -> bool {
        self.scenes.len() == 0
    }

    pub fn push(&mut self, scene: BoxedScene<'a, SceneChangeParamsT, EngineDataT>) {
        self.scenes.push(scene)
    }

    pub fn swap(&mut self, scene: BoxedScene<'a, SceneChangeParamsT, EngineDataT>) -> Option<BoxedScene<'a, SceneChangeParamsT, EngineDataT>> {
        let old_scene = self.scenes.pop();
        self.scenes.push(scene);
        old_scene
    }

    pub fn pop(&mut self) -> Option<BoxedScene<'a, SceneChangeParamsT, EngineDataT>> {
        self.scenes.pop()
    }

    pub fn render(&mut self, renderer: &mut WindowCanvas, engine_data: &mut EngineDataT) {
        let maybe_last_scene = self.scenes.pop();
        match maybe_last_scene {
            Some(scene) => {
                scene.render(renderer, engine_data);
                self.scenes.push(scene);
            },
            None => ()
        }
    }

    pub fn handle_event(&mut self, event: &Event, renderer: &mut WindowCanvas, engine_data: &mut EngineDataT) -> Option<SceneChangeEvent<SceneChangeParamsT>> {
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
