use scene::{BoxedScene, Scene};
use event::{Event, Render, RenderArgs, Input};
use conrod::UiContext;
use opengl_graphics::Gl;
use conrod::{
    Background,
    Color,
    Colorable,
    Drawable,
    Label,
    Labelable,
    Positionable
};
use input::{Release, Keyboard, keyboard};
use skillsscene::SkillsScene;

pub struct TitleScene;


impl TitleScene {
    pub fn new() -> BoxedScene {
        box TitleScene
    }

    fn render(&mut self, args: RenderArgs, uic: &mut UiContext, gl: &mut Gl) {
        uic.background().color(Color::black()).draw(gl);
        uic.label("1: skills.mul")
            .position(0.0, 0.0)
            .size(16u32)
            .color(Color::white())
            .draw(gl);
    }
}

impl Scene for TitleScene {
    fn handle_event(&mut self, e: &Event, ui_context: &mut UiContext, gl: &mut Gl) -> Option<BoxedScene> {
        match *e {
            Render(args) => {
                self.render(args, ui_context, gl);
                None
            },

            Input(Release(Keyboard(key))) => {
                match key {
                    keyboard::D1 => {
                        Some(SkillsScene::new())
                    },
                    _ => None
                }
            },
            _ => None
        }
    }
}
