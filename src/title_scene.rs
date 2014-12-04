use scene::{BoxedScene, Scene};
use event::{Event, RenderArgs};
use conrod::UiContext;
use opengl_graphics::Gl;
use conrod::{
    Background,
    Color,
    Colorable,
    Drawable
};
use input::{InputEvent, Button};
use input::keyboard::Key;
use skills_scene::SkillsScene;
use hues_scene::HuesScene;
use tile_scene::TileScene;
use statics_scene::StaticsScene;

pub struct TitleScene;

impl TitleScene {
    pub fn new() -> BoxedScene {
        box TitleScene
    }

    fn render(&mut self, args: RenderArgs, uic: &mut UiContext, gl: &mut Gl) {
        uic.background().color(Color::black()).draw(gl);
        gl.draw([0, 0, args.width as i32, args.height as i32], |_c , gl| {
            for (idx, &label) in["1: skills.idx + skills.mul", "2: hues.mul", "3: art.mul (tiles)", "4: art.mul (statics)"].iter().enumerate() {
                self.draw_label(uic, gl, label, 0.0, (idx * 16) as f64);
            }
        });
    }
}

impl Scene for TitleScene {
    fn handle_event(&mut self, e: &Event, ui_context: &mut UiContext, gl: &mut Gl) -> Option<BoxedScene> {
        match *e {
            Event::Render(args) => {
                self.render(args, ui_context, gl);
                None
            },

            Event::Input(InputEvent::Release(Button::Keyboard(key))) => {
                match key {
                    Key::D1 => {
                        Some(SkillsScene::new())
                    },
                    Key::D2 => {
                        Some(HuesScene::new())
                    },
                    Key::D3 => {
                        Some(TileScene::new())
                    },
                    Key::D4 => {
                        Some(StaticsScene::new())
                    },
                    _ => None
                }
            },
            _ => None
        }
    }
}
