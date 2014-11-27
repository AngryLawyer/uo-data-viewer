use event::Event;
use conrod::UiContext;
use opengl_graphics::Gl;
use conrod::{
    Color,
    Colorable,
    Drawable,
    Label,
    Labelable,
    Positionable
};

pub type BoxedScene = Box<Scene + 'static>;

pub trait Scene {
    fn handle_event(&mut self, e: &Event, ui_context: &mut UiContext, gl: &mut Gl) -> Option<BoxedScene>;

    fn draw_label(&self, uic: &mut UiContext, gl: &mut Gl, label: &str, x: f64, y: f64) {
        uic.label(label)
            .position(x, y)
            .size(16u32)
            .color(Color::white())
            .draw(gl);
    }
}
