use sdl2::render::{Renderer};
use sdl2::event::Event;
pub type BoxedScene = Box<Scene + 'static>;

pub trait Scene {
    fn render(&self, renderer: &mut Renderer);
    fn handle_event(&self, event: &Event);
}
