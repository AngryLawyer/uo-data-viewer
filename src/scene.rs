use ggez::event::{KeyCode, KeyMods, MouseButton};
use ggez::{graphics, Context, ContextBuilder, GameResult};

#[derive(Debug, Copy, Clone)]
pub enum SceneName {
    TitleScene,
    SkillsScene,
    TileScene,
    StaticsScene,
    HuesScene,
    MapScene,
    GumpScene,
    AnimScene,
    TexMapsScene,
    WorldScene,
}

pub type BoxedScene<'a, SceneChangeParamsT, EngineDataT> =
    Box<Scene<SceneChangeParamsT, EngineDataT> + 'a>;

pub trait Scene<SceneChangeParamsT, EngineDataT> {
    fn draw(&mut self, ctx: &mut Context, engine_data: &mut EngineDataT);
    fn update(
        &mut self,
        ctx: &mut Context,
        engine_data: &mut EngineDataT,
    ) -> Option<SceneChangeEvent<SceneChangeParamsT>>;
    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        keymods: KeyMods,
        repeat: bool,
        engine_data: &mut EngineDataT,
    ) {
    }
    fn mouse_button_down_event(
        &mut self,
        ctx: &mut Context,
        button: MouseButton,
        x: f32,
        y: f32,
        engine_data: &mut EngineDataT,
    ) {
    }
}

#[derive(Debug, Copy, Clone)]
pub enum SceneChangeEvent<T> {
    PushScene(T),
    SwapScene(T),
    PopScene,
}

pub struct SceneStack<'a, SceneChangeParamsT, EngineDataT> {
    scenes: Vec<BoxedScene<'a, SceneChangeParamsT, EngineDataT>>,
}

impl<'a, SceneChangeParamsT, EngineDataT> SceneStack<'a, SceneChangeParamsT, EngineDataT> {
    pub fn new() -> SceneStack<'a, SceneChangeParamsT, EngineDataT> {
        SceneStack { scenes: vec![] }
    }

    pub fn is_empty(&self) -> bool {
        self.scenes.len() == 0
    }

    pub fn push(&mut self, scene: BoxedScene<'a, SceneChangeParamsT, EngineDataT>) {
        self.scenes.push(scene)
    }

    pub fn swap(
        &mut self,
        scene: BoxedScene<'a, SceneChangeParamsT, EngineDataT>,
    ) -> Option<BoxedScene<'a, SceneChangeParamsT, EngineDataT>> {
        let old_scene = self.scenes.pop();
        self.scenes.push(scene);
        old_scene
    }

    pub fn pop(&mut self) -> Option<BoxedScene<'a, SceneChangeParamsT, EngineDataT>> {
        self.scenes.pop()
    }

    pub fn draw(&mut self, ctx: &mut Context, engine_data: &mut EngineDataT) {
        let maybe_last_scene = self.scenes.pop();
        match maybe_last_scene {
            Some(mut scene) => {
                scene.draw(ctx, engine_data);
                self.scenes.push(scene);
            }
            None => (),
        }
    }

    pub fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        keymods: KeyMods,
        repeat: bool,
        engine_data: &mut EngineDataT,
    ) {
        let maybe_last_scene = self.scenes.pop();
        match maybe_last_scene {
            Some(mut scene) => {
                scene.key_down_event(ctx, keycode, keymods, repeat, engine_data);
                self.scenes.push(scene);
            }
            None => (),
        }
    }

    pub fn update(
        &mut self,
        ctx: &mut Context,
        engine_data: &mut EngineDataT,
    ) -> Option<SceneChangeEvent<SceneChangeParamsT>> {
        let maybe_last_scene = self.scenes.pop();
        match maybe_last_scene {
            Some(mut scene) => {
                let event = scene.update(ctx, engine_data);
                self.scenes.push(scene);
                event
            }
            None => None,
        }
    }

    pub fn mouse_button_down_event(
        &mut self,
        ctx: &mut Context,
        button: MouseButton,
        x: f32,
        y: f32,
        engine_data: &mut EngineDataT,
    ) {
        let maybe_last_scene = self.scenes.pop();
        match maybe_last_scene {
            Some(mut scene) => {
                scene.mouse_button_down_event(ctx, button, x, y, engine_data);
                self.scenes.push(scene);
            }
            None => (),
        }
    }
}
