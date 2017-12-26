use scene::SceneName;
use engine::EngineData;
use image_convert::image_to_surface;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::surface::Surface;
use std::io::Result;
use std::path::Path;
use std::fs::File;

use uorustlibs::art::{ArtReader, Art};
use uorustlibs::tiledata::{TileDataReader, MapTileData};

use sdl2::render::{WindowCanvas, Texture, TextureCreator};
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::video::WindowContext;
use sdl2_engine_helpers::scene::{Scene, BoxedScene, SceneChangeEvent};

static MAX_X:u32 = 20;
static MAX_Y:u32 = 10;

pub struct TileScene<'a> {
    texture_creator: &'a TextureCreator<WindowContext>,
    reader: Result<ArtReader<File>>,
    data: Result<TileDataReader>,
    index: u32,
    texture: Option<Texture<'a>>,
    tile_data: Vec<Result<MapTileData>>,
    exiting: bool,
}

impl<'a> TileScene<'a> {
    pub fn new<'b>(engine_data: &mut EngineData<'b>, texture_creator: &'a TextureCreator<WindowContext>) -> BoxedScene<'a, Event, SceneName, EngineData<'b>> {
        let reader = ArtReader::new(&Path::new("./assets/artidx.mul"), &Path::new("./assets/art.mul"));
        let data = TileDataReader::new(&Path::new("./assets/tiledata.mul"));
        let mut scene = Box::new(TileScene {
            reader: reader,
            data: data,
            index: 0,
            texture: None,
            tile_data: vec![],
            texture_creator,
            exiting: false
        });
        scene.create_slice(engine_data);

        scene
    }

    fn create_slice(&mut self, engine_data: &mut EngineData) {
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
                                let image = tile.to_image();
                                let surface = image_to_surface(&image);
                                surface.blit(None, &mut dest, Some(Rect::new(44 * x as i32, (44 + 16) * y as i32, 44, 44))).expect("Failed to blit texture");
                            },
                            _ => ()
                        };
                        let label = engine_data.text_renderer.create_text(&format!("{}", index), Color::RGBA(255, 255, 255, 255));
                        label.blit(None, &mut dest, Some(Rect::new(44 * x as i32, ((44 + 16) * y as i32) + 44, label.width(), label.height()))).expect("Failed to blit texture");
                        self.tile_data.push(data.read_map_tile_data(index));
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

impl<'a, 'b> Scene<Event, SceneName, EngineData<'b>> for TileScene<'a> {
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
            Event::MouseButtonDown { x, y, .. } => {
                self.handle_click(x, y);
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
