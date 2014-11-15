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
use skills_scene::SkillsScene;
use hues_scene::HuesScene;

pub struct TitleScene;


impl TitleScene {
    pub fn new() -> BoxedScene {
        box TitleScene
    }

    fn render(&mut self, args: RenderArgs, uic: &mut UiContext, gl: &mut Gl) {
        uic.background().color(Color::black()).draw(gl);
        for (idx, &label) in["1: skills.idx + skills.mul", "2: hues.mul"].iter().enumerate() {
            uic.label(label)
                .position(0.0, (idx * 16) as f64)
                .size(16u32)
                .color(Color::white())
                .draw(gl);
        }
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
                    keyboard::D2 => {
                        Some(HuesScene::new())
                    },
                    _ => None
                }
            },
            _ => None
        }
    }
}