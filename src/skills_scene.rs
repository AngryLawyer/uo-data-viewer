use ggez::glam::Vec2;
use std::path::Path;
use uorustlibs::skills::Skills;

use crate::scene::{BoxedScene, Scene, SceneChangeEvent, SceneName};
use ggez::graphics::{Canvas, Color, DrawParam, Text};
use ggez::input::keyboard::{KeyCode, KeyInput};
use ggez::{Context, GameResult};

pub struct SkillsScene {
    pages: Vec<Text>,
    exiting: bool,
}

impl<'a> SkillsScene {
    pub fn new() -> BoxedScene<'a, SceneName, ()> {
        let skills = Skills::new(
            Path::new("./assets/skills.idx"),
            Path::new("./assets/skills.mul"),
        );
        let text = match skills {
            Ok(skills) => {
                let items: Vec<Text> = skills
                    .skills
                    .chunks(30)
                    .map(|chunk| {
                        let skills: Vec<String> = chunk
                            .iter()
                            .map(|skill| {
                                let glyph = if skill.clickable { "+" } else { "-" };
                                format!("{} {}", glyph, skill.name)
                            })
                            .collect();
                        Text::new(skills.join("\n"))
                    })
                    .collect();
                items
            }
            Err(error) => {
                let text = format!("{}", error);
                let texture = Text::new(text);
                vec![texture]
            }
        };
        Box::new(SkillsScene {
            pages: text,
            exiting: false,
        })
    }
}

impl Scene<SceneName, ()> for SkillsScene {
    fn draw(&mut self, ctx: &mut Context, _engine_data: &mut ()) -> GameResult<()> {
        let mut canvas = Canvas::from_frame(ctx, Color::BLACK);
        let mut last_width = 0;
        for page in self.pages.iter() {
            let width = page.measure(ctx)?.x;
            canvas.draw(
                page,
                DrawParam::default()
                    .dest(Vec2::new(last_width as f32, 0.0))
                    .color(Color::WHITE),
            );
            last_width += width as i32;
        }
        canvas.finish(ctx)
    }

    fn update(
        &mut self,
        _ctx: &mut Context,
        _engine_data: &mut (),
    ) -> GameResult<Option<SceneChangeEvent<SceneName>>> {
        if self.exiting {
            Ok(Some(SceneChangeEvent::PopScene))
        } else {
            Ok(None)
        }
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keyinput: KeyInput,
        _repeat: bool,
        _engine_data: &mut (),
    ) {
        if let Some(KeyCode::Escape) = keyinput.keycode {
            self.exiting = true;
        }
    }
}
