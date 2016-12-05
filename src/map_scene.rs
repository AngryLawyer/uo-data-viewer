use scene::{BoxedScene, Scene, SceneChangeEvent, SceneName};
use engine::EngineData;
use text_renderer::TextRenderer;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::surface::Surface;
use std::io::Result;
use std::path::Path;
use uorustlibs::art::{ArtReader, Art};
use uorustlibs::hues::{HueReader, HueGroup, Hue};
use uorustlibs::color::Color as ColorTrait;
use uorustlibs::map::{MapReader, Block};

use std::io::{Error, };
use sdl2::render::{Renderer, Texture, TextureQuery};
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub struct MapScene {
    reader: Result<MapReader>,
    x: u32,
    y: u32,
    texture: Option<Texture>
}

impl MapScene {
    pub fn new<'a>(renderer: &mut Renderer, engine_data: &mut EngineData<'a>) -> BoxedScene<SceneName, EngineData<'a>> {
        let mut scene = Box::new(MapScene {
            reader: MapReader::new(&Path::new("./assets/map0.mul"), 768, 512),
            x: 0,
            y: 0,
            texture: None
        });
        scene.draw_page(renderer, engine_data);
        scene
    }

    pub fn draw_page(&mut self, renderer: &mut Renderer, engine_data: &mut EngineData) {
        let block = match self.reader {
            Ok(ref mut reader) => {
                reader.read_block_from_coordinates(self.x, self.y).ok()
            },
            _ => None
        };
        self.texture = match block {
            Some(block) => {
                let mut surface = Surface::new(1024, 768, PixelFormatEnum::RGBA8888).unwrap();
                let block_surface = self.draw_block(&block);
                block_surface.blit(None, &mut surface, Some(Rect::new(0, 0, block_surface.width(), block_surface.height())));
                Some(renderer.create_texture_from_surface(&surface).unwrap())
            },
            _ => None
        };
    }
    
    pub fn draw_block(&self, block: &Block) -> Surface {
        let mut surface = Surface::new(8, 8, PixelFormatEnum::RGBA8888).unwrap();
        surface.with_lock_mut(|bitmap| {
            let mut read_idx = 0;

            for y in 0..8 {
                for x in 0..8 {
                    let target = (x + (y * 8));
                    let height = (block.cells[target].altitude as i16 + 128) as u8;
                    bitmap[target * 4] = 255;
                    bitmap[target * 4 + 1] = height;
                    bitmap[target * 4 + 2] = height;
                    bitmap[target * 4 + 3] = height;
                }
            };
        });
        surface
    }
}

impl<'a> Scene<SceneName, EngineData<'a>> for MapScene {
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
                if self.x > 0 {
                    self.x -= 1;
                    self.draw_page(renderer, engine_data);
                }
                None
            },
            Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                self.x += 1;
                self.draw_page(renderer, engine_data);
                None
            },
            Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                if self.y > 0 {
                    self.y -= 1;
                    self.draw_page(renderer, engine_data);
                }
                None
            },
            Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                self.y += 1;
                self.draw_page(renderer, engine_data);
                None
            },
             _ => None
        }
    }
}
