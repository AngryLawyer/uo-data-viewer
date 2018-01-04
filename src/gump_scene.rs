use engine::EngineData;
use image_convert::image_to_surface;
use scene::SceneName;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::Rect;
use sdl2::render::{WindowCanvas, Texture, TextureCreator};
use sdl2::surface::Surface;
use sdl2::video::WindowContext;
use sdl2_engine_helpers::scene::{Scene, BoxedScene, SceneChangeEvent};
use std::fs::File;
use std::io::Result;
use std::path::Path;
use uorustlibs::gump::GumpReader;

pub struct GumpScene<'a> {
    reader: Result<GumpReader<File>>,
    index: u32,
    texture_creator: &'a TextureCreator<WindowContext>,
    texture: Option<Texture<'a>>,
    exiting: bool,
}

impl<'a> GumpScene<'a> {
    pub fn new<'b>(engine_data: &mut EngineData<'b>, texture_creator: &'a TextureCreator<WindowContext>) -> BoxedScene<'a, Event, SceneName, EngineData<'b>> {
        let reader = GumpReader::new(&Path::new("./assets/gumpidx.mul"), &Path::new("./assets/gumpart.mul"));
        let mut scene = Box::new(GumpScene {
            reader: reader,
            index: 0,
            texture: None,
            exiting: false,
            texture_creator,
        });
        scene.create_slice(engine_data);

        scene
    }

    fn create_slice(&mut self, engine_data: &mut EngineData) {
        let mut dest = Surface::new(1024, 768, PixelFormatEnum::RGBA8888).unwrap();
        match self.reader {
            Ok(ref mut reader) => {
                match reader.read_gump(self.index) {
                    Ok(gump) => {
                        dest.fill_rect(None, Color::RGB(0, 0, 0)).unwrap();
                        let image = gump.to_image();
                        let surface = image_to_surface(&image);
                        surface.blit(None, &mut dest, Some(Rect::new(0, 0, surface.width(), surface.height()))).expect("Could not blit surface");
                        let label = engine_data.text_renderer.create_text(&format!("{}", self.index), Color::RGBA(255, 255, 255, 255));
                        label.blit(None, &mut dest, Some(Rect::new(9, surface.height() as i32 + 16, label.width(), label.height()))).expect("Could not blit surface");
                    },
                    _ => {
                        let error = engine_data.text_renderer.create_text("Invalid gump", Color::RGBA(255, 255, 255, 255));
                        error.blit(None, &mut dest, Some(Rect::new(0, 0, error.width(), error.height()))).expect("Could not blit surface");
                        let label = engine_data.text_renderer.create_text(&format!("{}", self.index), Color::RGBA(255, 255, 255, 255));
                        label.blit(None, &mut dest, Some(Rect::new(0, 16, label.width(), label.height()))).expect("Could not blit surface");
                    }
                }
            },
            _ => {
                let error = engine_data.text_renderer.create_text("Could not create slice", Color::RGBA(255, 255, 255, 255));
                error.blit(None, &mut dest, Some(Rect::new(0, 0, error.width(), error.height()))).expect("Could not blit surface");
            }
        }
        self.texture = Some(self.texture_creator.create_texture_from_surface(&dest).unwrap());
    }
}

impl<'a, 'b> Scene<Event, SceneName, EngineData<'b>> for GumpScene<'a> {
    fn render(&self, renderer: &mut WindowCanvas, _engine_data: &EngineData, _tick: u64) {
        renderer.clear();
        match self.texture {
            Some(ref texture) => {
                renderer.copy(texture, None, None).unwrap();
            },
            None => ()
        };
        renderer.present();
    }

    fn handle_event(&mut self, event: &Event, engine_data: &mut EngineData, _tick: u64) {
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
