use scene::{BoxedScene, Scene, SceneChangeEvent, SceneName};
use engine::EngineData;
use text_renderer::TextRenderer;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::surface::Surface;
use std::io::Result;
use std::path::Path;
use uorustlibs::art::{ArtReader, Art};
use uorustlibs::tiledata::{TileDataReader, MapTileData};

use sdl2::render::{Renderer, Texture, TextureQuery};
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

static MAX_X:u32 = 20;
static MAX_Y:u32 = 10;

pub struct TileScene {
    reader: Result<ArtReader>,
    data: Result<TileDataReader>,
    index: u32,
    texture: Option<Texture>,
    tile_data: Vec<Result<MapTileData>>
}

impl TileScene {
    pub fn new<'a>(renderer: &mut Renderer, engine_data: &mut EngineData<'a>) -> BoxedScene<SceneName, EngineData<'a>> {
        let reader = ArtReader::new(&Path::new("./assets/artidx.mul"), &Path::new("./assets/art.mul"));
        let data = TileDataReader::new(&Path::new("./assets/tiledata.mul"));
        let mut scene = Box::new(TileScene {
            reader: reader,
            data: data,
            index: 0,
            texture: None,
            tile_data: vec![],
        });
        scene.create_slice(renderer, engine_data);

        scene
    }

    fn create_slice(&mut self, renderer: &mut Renderer, engine_data: &mut EngineData) {
        self.tile_data = vec![];
        match (&mut self.reader, &mut self.data) {
            (&mut Ok(ref mut reader), &mut Ok(ref mut data)) => {
                let limit = MAX_X * MAX_Y;
                let start = limit * self.index;
                let mut dest = Surface::new(1024, 768, PixelFormatEnum::RGBA8888).unwrap();
                dest.fill_rect(None, Color::RGB(0, 0, 0)).unwrap();

                for y in 0..MAX_Y {
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
                        self.tile_data.push(data.read_map_tile_data(index));
                    }
                }

                self.texture = Some(renderer.create_texture_from_surface(&dest).unwrap());
            },
            _ => {
                let texture = engine_data.text_renderer.create_text_texture(renderer, "Could not create slice", Color::RGBA(255, 255, 255, 255));
                self.texture = Some(texture);
            }
        }
    }

    fn handle_click(&mut self, x: i32, y: i32) {
        let actual_x = x / 44;
        let actual_y = y / (44 + 16);
        if actual_x < MAX_X as i32 && actual_y < MAX_Y as i32 {
            let actual_index = (actual_x + (actual_y * MAX_X as i32)) as usize;
            if actual_index < self.tile_data.len() {
                match self.tile_data[actual_index] {
                    Ok(ref data) => {
                        println!("{}", data.name);
                    },
                    _ => ()
                }
            }
        }
    }
}

impl<'a> Scene<SceneName, EngineData<'a>> for TileScene {
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
            Event::MouseButtonDown { x: x, y: y, .. } => {
                self.handle_click(x, y);
                None
            },       
             _ => None
        }
    }
}
