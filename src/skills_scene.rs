use scene::{BoxedScene, Scene};
use std::io::Result;
use std::path::Path;
use uorustlibs::skills::Skills;

use sdl2::render::{Renderer, Texture, TextureQuery};
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2_ttf::Font;

pub struct SkillsScene {
    skills: Result<Skills>
}

impl SkillsScene {
    pub fn new() -> BoxedScene {
        Box::new(SkillsScene {
            skills: Skills::new(&Path::new("./assets/skills.idx"), &Path::new("./assets/skills.mul"))
        })
    }

    /*fn render(&mut self, _args: RenderArgs, uic: &mut UiContext, gl: &mut Gl) {
        gl.enable_alpha_blend();
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
    }*/
}

impl Scene for SkillsScene {
    /*fn handle_event(&mut self, e: &Event, ui_context: &mut UiContext, gl: &mut Gl) -> Option<BoxedScene> {
        match *e {
            Event::Render(args) => {
                self.render(args, ui_context, gl);
                None
            },
            _ => None
        }
    }*/

    fn render(&self, renderer: &mut Renderer) {
        /*let TextureQuery {width, height, .. } = self.text.query();
        let target = Rect::new(0, 0, width, height);
        renderer.clear();
        renderer.copy(&self.text, None, Some(target)).unwrap();
        renderer.present();*/
    }

    fn handle_event(&self, event: &Event) {
         match *event {
            Event::KeyDown { keycode: Some(Keycode::Num1), .. } => {
                println!("2");
            },
             _ => ()
         };
    }
}
