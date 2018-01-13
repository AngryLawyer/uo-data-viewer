use engine::EngineData;
use image_convert::frame_to_surface;
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

use uorustlibs::anim::AnimReader;

pub struct AnimScene<'a> {
    reader: Result<AnimReader<File>>,
    file_index: u8,
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
            file_index: 0,
        });
        scene.load_reader(&Path::new("./assets/anim.idx"), &Path::new("./assets/Anim.mul"), engine_data);

        scene
    }

    fn load_reader(&mut self, idx: &Path, mul: &Path, engine_data: &mut EngineData) {
        self.reader = AnimReader::new(idx, mul);
        self.create_slice(engine_data);
    }

    fn set_file_index(&mut self, idx: u8, engine_data: &mut EngineData) {
        self.file_index = idx;
        let (idx, mul) = match idx {
            0 => (Path::new("./assets/anim.idx"), Path::new("./assets/Anim.mul")),
            1 => (Path::new("./assets/anim2.idx"), Path::new("./assets/anim2.mul")),
            2 => (Path::new("./assets/anim3.idx"), Path::new("./assets/anim3.mul")),
            _ => panic!("NO")
        };
        self.load_reader(&idx, &mul, engine_data);
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
                self.textures = anim.to_frames().enumerate().map(|(idx, frame)| {
                    let mut dest = Surface::new(1024, 768, PixelFormatEnum::RGBA8888).unwrap();
                    dest.fill_rect(None, Color::RGB(0, 0, 0)).unwrap();
                    let surface = frame_to_surface(&frame);
                    surface.blit(None, &mut dest, Some(Rect::new(0, 0, surface.width(), surface.height()))).expect("Could not blit surface");

                    engine_data.text_renderer.draw_text(&mut dest, &Point::new(9, offset as i32 + surface.height() as i32 + 16), &format!("{}", self.index));
                    engine_data.text_renderer.draw_text(&mut dest, &Point::new(9, offset as i32 + surface.height() as i32 + 32), &format!("{} / {}", idx, anim.frame_count));
                    self.texture_creator.create_texture_from_surface(&dest).unwrap()
                }).collect::<_>();
            },
            Err(e) => {
                let mut dest = Surface::new(1024, 768, PixelFormatEnum::RGBA8888).unwrap();
                dest.fill_rect(None, Color::RGB(0, 0, 0)).unwrap();
                engine_data.text_renderer.draw_text(&mut dest, &Point::new(0, 0), e);
                engine_data.text_renderer.draw_text(&mut dest, &Point::new(0, 16), &format!("{}", self.index));
                self.textures = vec![self.texture_creator.create_texture_from_surface(&dest).unwrap()];
            }
        }
        self.current_frame = 0;
    }
}

impl<'a, 'b> Scene<Event, SceneName, EngineData<'b>> for AnimScene<'a> {
    fn render(&mut self, renderer: &mut WindowCanvas, _engine_data: &mut EngineData, _tick: u64) {
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
            Event::KeyDown { keycode: Some(Keycode::Comma), .. } => {
                if self.index >= 10 {
                    self.index -= 10;
                    self.create_slice(engine_data);
                }
            },
            Event::KeyDown { keycode: Some(Keycode::Period), .. } => {
                self.index += 10;
                self.create_slice(engine_data);
            },
            Event::KeyDown { keycode: Some(Keycode::Tab), .. } => {
                let idx = self.file_index;
                self.set_file_index((idx + 1) % 3, engine_data);
            },
             _ => ()
        }
    }

    fn think(&mut self, _engine_data: &mut EngineData, tick: u64) -> Option<SceneChangeEvent<SceneName>> {
        if self.exiting {
            return Some(SceneChangeEvent::PopScene);
        }

        if tick % 2 == 0 {
            self.current_frame += 1;
            if self.current_frame >= self.textures.len() {
                self.current_frame = 0;
            }
        }

        None
    }
}
