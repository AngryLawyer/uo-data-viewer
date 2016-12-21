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
        match self.reader {
            Ok(ref mut reader) => {
                match reader.read_gump(self.index) {
                    Ok(gump) => {
                        let mut dest = Surface::new(1024, 768, PixelFormatEnum::RGBA8888).unwrap();
                        dest.fill_rect(None, Color::RGB(0, 0, 0)).unwrap();
                        let surface = gump.to_surface();
                        surface.blit(None, &mut dest, Some(Rect::new(0, 0, surface.width(), surface.height())));
                        let label = engine_data.text_renderer.create_text(&format!("{}", self.index), Color::RGBA(255, 255, 255, 255));
                        label.blit(None, &mut dest, Some(Rect::new(9, surface.height() as i32 + 16, label.width(), label.height())));
                        self.texture = Some(renderer.create_texture_from_surface(&dest).unwrap());
                    },
                    _ => {
                        let texture = engine_data.text_renderer.create_text_texture(renderer, "Could not create gump", Color::RGBA(255, 255, 255, 255));
                        self.texture = Some(texture);
                    }
                }

                /*for y in 0..MAX_Y {
                    for x in 0..MAX_X {
                        let maybe_tile = reader.read_tile(start + x + (y * MAX_X));
                        let index = start + x + (y * MAX_X);
                        match maybe_tile {
                            Ok(tile) => {
                                let surface = tile.to_surface();
                                surface.blit(None, &mut dest, Some(Rect::new(44 * x as i32, (44 + 16) * y as i32, 44, 44)));
                            },
                            _ => ()
                        };
                        let label = engine_data.text_renderer.create_text(&format!("{}", index), Color::RGBA(255, 255, 255, 255));
                        label.blit(None, &mut dest, Some(Rect::new(44 * x as i32, ((44 + 16) * y as i32) + 44, label.width(), label.height())));
                    }
                }*/

            },
            _ => {
                let texture = engine_data.text_renderer.create_text_texture(renderer, "Could not create slice", Color::RGBA(255, 255, 255, 255));
                self.texture = Some(texture);
            }
        }
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
