use scene::{BoxedScene, Scene, SceneChangeEvent, SceneName};
use engine::EngineData;
use text_renderer::TextRenderer;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::surface::Surface;
use std::io::Result;
use std::path::Path;
use uorustlibs::art::{ArtReader, Art};

use sdl2::render::{Renderer, Texture, TextureQuery};
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

static MAX_X:u32 = 8;
static MAX_Y:u32 = 4;


pub struct StaticsScene {
    reader: Result<ArtReader>,
    index: u32,
    texture: Option<Texture>
}

impl StaticsScene {
    pub fn new<'a>(renderer: &mut Renderer, engine_data: &mut EngineData<'a>) -> BoxedScene<SceneName, EngineData<'a>> {
        let reader = ArtReader::new(&Path::new("./assets/artidx.mul"), &Path::new("./assets/art.mul"));
        let mut scene = Box::new(StaticsScene {
            reader: reader,
            index: 0,
            texture: None
        });
        scene.create_slice(renderer, engine_data);

        scene
    }

    fn create_slice(&mut self, renderer: &mut Renderer, engine_data: &mut EngineData) {

        match self.reader {
            Ok(ref mut reader) => {
                let limit = MAX_X * MAX_Y;
                let start = limit * self.index;
                let mut dest = Surface::new(1024, 768, PixelFormatEnum::RGBA8888).unwrap();
                dest.fill_rect(None, Color::RGB(0, 0, 0)).unwrap();

                for x in 0..MAX_X {
                    for y in 0..MAX_Y {
                        let index = start + x + (y * MAX_X);
                        let maybe_static = reader.read_static(index);
                        match maybe_static {
                            Ok(stat) => {
                                let surface = stat.to_surface();
                                surface.blit(None, &mut dest, Some(Rect::new(128 * x as i32, (128 + 16) * y as i32, surface.width(), surface.height())));
                            },
                            _ => ()
                        }
                        let label = engine_data.text_renderer.create_text(&format!("{}", index), Color::RGBA(255, 255, 255, 255));
                        label.blit(None, &mut dest, Some(Rect::new(128 * x as i32, ((128 + 16) * y as i32) + 128, label.width(), label.height())));
                    }
                }

                self.texture = Some(renderer.create_texture_from_surface(&dest).unwrap());
            },
            Err(_) => {
                let texture = engine_data.text_renderer.create_text_texture(renderer, "Could not create slice", Color::RGBA(255, 255, 255, 255));
                self.texture = Some(texture);
            }
        }
    }
}

impl<'a> Scene<SceneName, EngineData<'a>> for StaticsScene {
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
