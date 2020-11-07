use std::path::Path;
use uorustlibs::skills::Skills;

use cgmath::Point2;
use ggez::event::{KeyCode, KeyMods};
use ggez::graphics::{self, Text};
use ggez::{Context, GameResult};
use scene::{BoxedScene, Scene, SceneChangeEvent, SceneName};

pub struct SkillsScene {
    pages: Vec<Text>,
    exiting: bool,
}

impl<'a> SkillsScene {
    pub fn new() -> BoxedScene<'a, SceneName, ()> {
        let skills = Skills::new(
            &Path::new("./assets/skills.idx"),
            &Path::new("./assets/skills.mul"),
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
        graphics::clear(ctx, graphics::BLACK);
        let mut last_width = 0;
        for page in self.pages.iter() {
            let width = page.width(ctx);
            graphics::draw(
                ctx,
                page,
                (Point2::new(last_width as f32, 0.0), graphics::WHITE),
            )?;
            last_width += width as i32;
        }
        Ok(())
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
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
        _engine_data: &mut (),
    ) {
        match keycode {
            KeyCode::Escape => self.exiting = true,
            _ => (),
        }
    }
}
