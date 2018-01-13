use engine::EngineData;
use scene::SceneName;
use std::path::Path;
use uorustlibs::skills::Skills;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{WindowCanvas, Texture, TextureQuery, TextureCreator};
use sdl2::video::WindowContext;
use sdl2_engine_helpers::scene::{Scene, BoxedScene, SceneChangeEvent};

pub struct SkillsScene<'b> {
    pages: Vec<Texture<'b>>,
    exiting: bool
}

impl<'b> SkillsScene<'b> {
    pub fn new<'a>(engine_data: &mut EngineData<'a>, texture_creator: &'b TextureCreator<WindowContext>) -> BoxedScene<'b, Event, SceneName, EngineData<'a>> {
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
                    engine_data.text_renderer.create_text_texture(texture_creator, &skills.join("\n"), Color::RGBA(255, 255, 255, 255))
                }).collect();
                items
            },
            Err(error) => {
                let text = format!("{}", error);
                let texture = engine_data.text_renderer.create_text_texture(texture_creator, &text, Color::RGBA(255, 255, 255, 255));
                vec![texture]
            }
        };
        Box::new(SkillsScene {
            pages: text,
            exiting: false
        })
    }
}

impl<'a, 'b> Scene<Event, SceneName, EngineData<'a>> for SkillsScene<'b> {
    fn render(&mut self, renderer: &mut WindowCanvas, _engine_data: &mut EngineData, _tick: u64) {
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

    fn handle_event(&mut self, event: &Event, _engine_data: &mut EngineData, _tick: u64) {
        match *event {
            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                self.exiting = true
            },
             _ => ()
        }
    }

    fn think(&mut self, _engine_data: &mut EngineData, _tick: u64) -> Option<SceneChangeEvent<SceneName>> {
        if self.exiting {
            Some(SceneChangeEvent::PopScene)
        } else {
            None
        }
    }
}
