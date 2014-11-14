use event::Event;
use conrod::UiContext;
use opengl_graphics::Gl;

pub type BoxedScene = Box<Scene + 'static>;

pub trait Scene {
    fn handle_event(&mut self, e: &Event, ui_context: &mut UiContext, gl: &mut Gl) -> Option<BoxedScene>;
}
