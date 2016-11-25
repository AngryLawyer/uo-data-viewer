use scene::{BoxedScene, Scene, SceneChangeEvent, SceneName};
use engine::EngineData;
use text_renderer::TextRenderer;
use sdl2::pixels::Color;
use std::io::Result;
use std::path::Path;
use uorustlibs::skills::Skills;

use sdl2::render::{Renderer, Texture, TextureQuery};
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub struct SkillsScene {
    pages: Vec<Texture>,
}

impl SkillsScene {
    pub fn new<'a>(renderer: &mut Renderer, engine_data: &mut EngineData<'a>) -> BoxedScene<SceneName, EngineData<'a>> {
        let skills = Skills::new(&Path::new("./assets/skills.idx"), &Path::new("./assets/skills.mul"));
        let text = match skills {
            Ok(skills) => {
                let items: Vec<Texture> = skills.skills.chunks(30).map(|chunk| {
                    let skills: Vec<String> = chunk.iter().map(|skill| {
                        let glyph = if skill.clickable {
                            "+"
                        } else {
                            "-"
                        };
                        format!("{} {}", glyph, skill.name)
                    }).collect();
                    engine_data.text_renderer.create_text_texture(renderer, &skills.join("\n"), Color::RGBA(255, 255, 255, 255))
                }).collect();
                items
            },
            Err(error) => {
                let text = format!("{}", error);
                let texture = engine_data.text_renderer.create_text_texture(renderer, &text, Color::RGBA(255, 255, 255, 255));
                vec![texture]
            }
        };
        Box::new(SkillsScene {
            pages: text,
        })
    }
}

impl<SceneName, EngineData> Scene<SceneName, EngineData> for SkillsScene {
    fn render(&self, renderer: &mut Renderer, engine_data: &mut EngineData) {
        renderer.clear();
        let mut last_width = 0;
        for page in self.pages.iter() {
            let TextureQuery {width, height, .. } = page.query();
            let target = Rect::new(last_width, 0, width, height);
            renderer.copy(page, None, Some(target)).unwrap();
            last_width += width as i32;
        }
        renderer.present();
    }

    fn handle_event(&mut self, event: &Event, renderer: &mut Renderer, engine_data: &mut EngineData) -> Option<SceneChangeEvent<SceneName>> {
        match *event {
            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                Some(SceneChangeEvent::PopScene)
            },
             _ => None
        }
    }
}
