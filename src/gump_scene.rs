use scene::{BoxedScene, Scene, SceneChangeEvent, SceneName};
use engine::EngineData;
use text_renderer::TextRenderer;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::surface::Surface;
use std::io::Result;
use std::path::Path;
use std::fs::File;

use uorustlibs::gump::{GumpReader, Gump};

use sdl2::render::{Renderer, Texture, TextureQuery};
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub struct GumpScene {
    reader: Result<GumpReader<File>>,
    index: u32,
    texture: Option<Texture>,
}

impl GumpScene {
    pub fn new<'a>(renderer: &mut Renderer, engine_data: &mut EngineData<'a>) -> BoxedScene<SceneName, EngineData<'a>> {
        let reader = GumpReader::new(&Path::new("./assets/gumpidx.mul"), &Path::new("./assets/gumpart.mul"));
        let mut scene = Box::new(GumpScene {
            reader: reader,
            index: 0,
            texture: None,
        });
        scene.create_slice(renderer, engine_data);

        scene
    }

    fn create_slice(&mut self, renderer: &mut Renderer, engine_data: &mut EngineData) {
        let mut dest = Surface::new(1024, 768, PixelFormatEnum::RGBA8888).unwrap();
        match self.reader {
            Ok(ref mut reader) => {
                match reader.read_gump(self.index) {
                    Ok(gump) => {
                        dest.fill_rect(None, Color::RGB(0, 0, 0)).unwrap();
                        let surface = gump.to_surface();
                        surface.blit(None, &mut dest, Some(Rect::new(0, 0, surface.width(), surface.height())));
                        let label = engine_data.text_renderer.create_text(&format!("{}", self.index), Color::RGBA(255, 255, 255, 255));
                        label.blit(None, &mut dest, Some(Rect::new(9, surface.height() as i32 + 16, label.width(), label.height())));
                    },
                    _ => {
                        let error = engine_data.text_renderer.create_text("Invalid gump", Color::RGBA(255, 255, 255, 255));
                        error.blit(None, &mut dest, Some(Rect::new(0, 0, error.width(), error.height())));
                        let label = engine_data.text_renderer.create_text(&format!("{}", self.index), Color::RGBA(255, 255, 255, 255));
                        label.blit(None, &mut dest, Some(Rect::new(0, 16, label.width(), label.height())));
                    }
                }
            },
            _ => {
                let error = engine_data.text_renderer.create_text("Could not create slice", Color::RGBA(255, 255, 255, 255));
            }
        }
        self.texture = Some(renderer.create_texture_from_surface(&dest).unwrap());
    }
}

impl<'a> Scene<SceneName, EngineData<'a>> for GumpScene {
    fn render(&self, renderer: &mut Renderer, engine_data: &mut EngineData) {
        renderer.clear();
        match self.texture {
            Some(ref texture) => {
                renderer.copy(texture, None, None).unwrap();
            },
            None => ()
        };
        renderer.present();
    }

    fn handle_event(&mut self, event: &Event, renderer: &mut Renderer, engine_data: &mut EngineData) -> Option<SceneChangeEvent<SceneName>> {
        match *event {
            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                Some(SceneChangeEvent::PopScene)
            },
            Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                if self.index > 0 {
                    self.index -= 1;
                    self.create_slice(renderer, engine_data);
                }
                None
            },
            Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                self.index += 1;
                self.create_slice(renderer, engine_data);
                None
            },
             _ => None
        }
    }
}