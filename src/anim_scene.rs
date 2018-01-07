use engine::EngineData;
use image_convert::image_to_surface;
use scene::SceneName;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::{Rect, Point};
use sdl2::render::{WindowCanvas, Texture, TextureCreator};
use sdl2::surface::Surface;
use sdl2::video::WindowContext;
use sdl2_engine_helpers::scene::{Scene, BoxedScene, SceneChangeEvent};
use std::fs::File;
use std::io::Result;
use std::path::Path;

use uorustlibs::anim::{AnimReader, AnimGroup,};

pub struct AnimScene<'a> {
    reader: Result<AnimReader<File>>,
    index: u32,
    texture_creator: &'a TextureCreator<WindowContext>,
    textures: Vec<Texture<'a>>,
    exiting: bool,
    current_frame: usize,
}

impl<'a> AnimScene<'a> {
    pub fn new<'b>(engine_data: &mut EngineData<'b>, texture_creator: &'a TextureCreator<WindowContext>) -> BoxedScene<'a, Event, SceneName, EngineData<'b>> {
        let reader = AnimReader::new(&Path::new("./assets/anim.idx"), &Path::new("./assets/Anim.mul"));  //FIXME: Multiple anim muls!
        let mut scene = Box::new(AnimScene {
            reader: reader,
            index: 110,
            textures: vec![],
            exiting: false,
            texture_creator,
            current_frame: 0,
        });
        scene.create_slice(engine_data);

        scene
    }

    fn create_slice(&mut self, engine_data: &mut EngineData) {
        let offset = 100;
        let anim = match self.reader {
            Ok(ref mut reader) => {
                reader.read(self.index).or_else(|_| Err("Invalid anim"))
            },
            _ => Err("No reader")
        };
        match anim {
            Ok(anim) => {
                self.textures = anim.frames.iter().enumerate().map(|(idx, frame)| {
                    let mut dest = Surface::new(1024, 768, PixelFormatEnum::RGBA8888).unwrap();
                    dest.fill_rect(None, Color::RGB(0, 0, 0)).unwrap();
                    dest.fill_rect(Rect::new(offset - frame.image_centre_x as i32, offset - frame.image_centre_y as i32, frame.width as u32, frame.height as u32), Color::RGB(255, 0, 0));
                    engine_data.text_renderer.draw_text(&mut dest, &Point::new(9, offset as i32 + frame.height as i32 + 16), &format!("{}", self.index));
                    engine_data.text_renderer.draw_text(&mut dest, &Point::new(9, offset as i32 + frame.height as i32 + 32), &format!("{} / {}", idx, anim.frame_count));
                    self.texture_creator.create_texture_from_surface(&dest).unwrap()
                }).collect::<_>();
            },
            Err(e) => {
                let mut dest = Surface::new(1024, 768, PixelFormatEnum::RGBA8888).unwrap();
                dest.fill_rect(None, Color::RGB(0, 0, 0)).unwrap();
                engine_data.text_renderer.draw_text(&mut dest, &Point::new(0, 0), e);
                self.textures = vec![self.texture_creator.create_texture_from_surface(&dest).unwrap()];
            }
        }
        self.current_frame = 0;
    }
}

impl<'a, 'b> Scene<Event, SceneName, EngineData<'b>> for AnimScene<'a> {
    fn render(&self, renderer: &mut WindowCanvas, _engine_data: &EngineData, _tick: u64) {
        renderer.clear();
        if self.textures.len() > 0 {
            renderer.copy(&self.textures[self.current_frame], None, None).unwrap();
        }
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

    fn think(&mut self, _engine_data: &mut EngineData, tick: u64) -> Option<SceneChangeEvent<SceneName>> {
        if self.exiting {
            return Some(SceneChangeEvent::PopScene);
        }

        self.current_frame += 1;
        if self.current_frame >= self.textures.len() {
            self.current_frame = 0;
        }

        None
    }
}
