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
use uorustlibs::skills::Skills;
use std::io::IoResult;

pub struct SkillsScene {
    skills: IoResult<Skills>
}

impl SkillsScene {
    pub fn new() -> BoxedScene {
        box SkillsScene {
            skills: Skills::new(&Path::new("./assets/skills.idx"), &Path::new("./assets/skills.mul"))
        }
    }

    fn render(&mut self, args: RenderArgs, uic: &mut UiContext, gl: &mut Gl) {
        uic.background().color(Color::black()).draw(gl);
        match self.skills {
            Ok(ref skills) => {
                for (idx, skill) in skills.skills.iter().enumerate() {
                    let glyph = if skill.clickable {
                        "+"
                    } else {
                        "-"
                    };
                    uic.label(format!("{} {}", glyph, skill.name).as_slice())
                        .position(((idx / 20) * 256) as f64, ((idx % 20) * 16) as f64)
                        .size(16u32)
                        .color(Color::white())
                        .draw(gl);
                }
            },
            Err(ref error) => {
                uic.label(format!("{}", error).as_slice())
                    .position(0.0, 0.0)
                    .size(16u32)
                    .color(Color::white())
                    .draw(gl);
            }
        }
    }
}

impl Scene for SkillsScene {
    fn handle_event(&mut self, e: &Event, ui_context: &mut UiContext, gl: &mut Gl) -> Option<BoxedScene> {
        match *e {
            Render(args) => {
                self.render(args, ui_context, gl);
                None
            },
            _ => None
        }
    }
}
