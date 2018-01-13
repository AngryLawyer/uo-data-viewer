use scene::SceneName;
use engine::EngineData;
use image_convert::image_to_surface;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::surface::Surface;
use std::io::Result;
use std::path::Path;
use std::fs::File;

use uorustlibs::texmaps::TexMapsReader;

use sdl2::render::{WindowCanvas, Texture, TextureCreator};
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::video::WindowContext;
use sdl2_engine_helpers::scene::{Scene, BoxedScene, SceneChangeEvent};

static MAX_X:u32 = 8;
static MAX_Y:u32 = 5;

pub struct TexMapsScene<'a> {
    texture_creator: &'a TextureCreator<WindowContext>,
    reader: Result<TexMapsReader<File>>,
    index: u32,
    texture: Option<Texture<'a>>,
    exiting: bool,
}

impl<'a> TexMapsScene<'a> {
    pub fn new<'b>(engine_data: &mut EngineData<'b>, texture_creator: &'a TextureCreator<WindowContext>) -> BoxedScene<'a, Event, SceneName, EngineData<'b>> {
        let reader = TexMapsReader::new(&Path::new("./assets/texidx.mul"), &Path::new("./assets/texmaps.mul"));
        let mut scene = Box::new(TexMapsScene {
            reader: reader,
            index: 0,
            texture: None,
            texture_creator,
            exiting: false
        });
        scene.create_slice(engine_data);

        scene
    }

    fn create_slice(&mut self, engine_data: &mut EngineData) {
        match self.reader {
            Ok(ref mut reader) => {
                let limit = MAX_X * MAX_Y;
                let start = limit * self.index;
                let mut dest = Surface::new(1024, 768, PixelFormatEnum::RGBA8888).unwrap();
                dest.fill_rect(None, Color::RGB(0, 0, 0)).unwrap();

                for y in 0..MAX_Y {
                    for x in 0..MAX_X {
                        let maybe_map = reader.read(start + x + (y * MAX_X));
                        let index = start + x + (y * MAX_X);
                        match maybe_map {
                            Ok(texmap) => {
                                let image = texmap.to_image();
                                let surface = image_to_surface(&image);
                                surface.blit(None, &mut dest, Some(Rect::new(128 * x as i32, (128 + 16) * y as i32, surface.width(), surface.height()))).expect("Failed to blit texture");
                            },
                            _ => ()
                        };
                        let label = engine_data.text_renderer.create_text(&format!("{}", index), Color::RGBA(255, 255, 255, 255));
                        label.blit(None, &mut dest, Some(Rect::new(128 * x as i32, ((128 + 16) * y as i32) + 128, label.width(), label.height()))).expect("Failed to blit texture");
                    }
                }

                self.texture = Some(self.texture_creator.create_texture_from_surface(&dest).unwrap());
            },
            _ => {
                let texture = engine_data.text_renderer.create_text_texture(self.texture_creator, "Could not create slice", Color::RGBA(255, 255, 255, 255));
                self.texture = Some(texture);
            }
        }
    }
}

impl<'a, 'b> Scene<Event, SceneName, EngineData<'b>> for TexMapsScene<'a> {
    fn render(&mut self, renderer: &mut WindowCanvas, _engine_data: &mut EngineData, _tick: u64) {
        renderer.clear();
        match self.texture {
            Some(ref texture) => {
                renderer.copy(texture, None, None).unwrap();
            },
            None => ()
        };
        renderer.present();
    }

    fn handle_event(&mut self, event: &Event, engine_data: &mut EngineData<'b>, _tick: u64){
        match *event {
            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                self.exiting = true;
            },
            Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                if self.index > 0 {
                    self.index -= 1;
                    self.create_slice(engine_data);
                }
            },
            Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                self.index += 1;
                self.create_slice(engine_data);
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
