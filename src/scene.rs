use sdl2::render::{Renderer};
pub type BoxedScene = Box<Scene + 'static>;

pub trait Scene {
    fn render(&self, renderer: &mut Renderer);
}
