use scene::{BoxedScene, Scene, SceneChangeEvent, SceneName};
use engine::EngineData;
use text_renderer::TextRenderer;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::surface::Surface;
use std::io::Result;
use std::path::Path;
use uorustlibs::art::{ArtReader, Art};
use uorustlibs::hues::{HueReader, HueGroup, Hue};
use uorustlibs::color::{Color16, Color as ColorTrait};
use uorustlibs::map::{MapReader, Block, RadarColReader};

use std::io::{Error, };
use sdl2::render::{Renderer, Texture, TextureQuery};
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

const MAX_BLOCKS_WIDTH: u32 = 1024 / 8;
const MAX_BLOCKS_HEIGHT: u32 = 768 / 8;
const STEP_X: u32 = MAX_BLOCKS_WIDTH / 4;
const STEP_Y: u32 = MAX_BLOCKS_HEIGHT / 4;

struct MapLens {
    pub blocks: Vec<Option<Block>>,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32
}

impl MapLens {
    pub fn new(map_reader: &mut MapReader, x: u32, y: u32, width: u32, height: u32) -> MapLens {
        let mut blocks = vec![];
        for yy in 0..height {
            for xx in 0..width {
                let block = map_reader.read_block_from_coordinates(x + xx, y + yy);
                blocks.push(block.ok());
            }
        }
        MapLens {
            blocks: blocks,
            x: x,
            y: y,
            width: width,
            height: height
        }
    }

    pub fn update(&self, map_reader: &mut MapReader, x: u32, y: u32) -> MapLens {
        let difference_x = x as i64 - self.x as i64;
        let difference_y = y as i64 - self.y as i64;
        println!("{} {}", difference_x, difference_y);
        MapLens {
            blocks: self.blocks.clone(),
            x: x,
            y: y,
            width: self.width,
            height: self.height
        }
    }
}

enum MapRenderMode {
    HeightMap,
    RadarMap,
}

pub struct MapScene {
    map_reader: Result<MapReader>,
    radar_colors: Result<Vec<Color16>>,
    x: u32,
    y: u32,
    mode: MapRenderMode,
    texture: Option<Texture>,
    map_lens: Option<MapLens>
}

pub fn draw_heightmap_block<'a>(block: &Block, radar_cols: &Result<Vec<Color16>>) -> Surface<'a> {
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

pub fn draw_radarcol_block<'a>(block: &Block, radar_cols: &Result<Vec<Color16>>) -> Surface<'a> {
    let mut surface = Surface::new(8, 8, PixelFormatEnum::RGBA8888).unwrap();
    surface.with_lock_mut(|bitmap| {
        let mut read_idx = 0;

        for y in 0..8 {
            for x in 0..8 {
                let target = (x + (y * 8));
                let index = block.cells[target].graphic;
                let (r, g, b, _) = match radar_cols {
                    &Ok(ref colors) => colors[index as usize].to_rgba(),
                    _ => index.to_rgba()
                };
                bitmap[target * 4] = 255;
                bitmap[target * 4 + 1] = b;
                bitmap[target * 4 + 2] = g;
                bitmap[target * 4 + 3] = r;
            }
        };
    });
    surface
}

impl MapScene {
    pub fn new<'a>(renderer: &mut Renderer, engine_data: &mut EngineData<'a>) -> BoxedScene<SceneName, EngineData<'a>> {
        let colors = RadarColReader::new(&Path::new("./assets/radarcol.mul"))
            .and_then(|mut reader| reader.read_colors());

        let mut scene = Box::new(MapScene {
            map_reader: MapReader::new(&Path::new("./assets/map0.mul"), 768, 512),
            x: 0,
            y: 0,
            texture: None,
            mode: MapRenderMode::HeightMap,
            radar_colors: colors,
            map_lens: None
        });

        scene.draw_page(renderer, engine_data);
        scene
    }

    pub fn refresh_lens(&mut self) {
        let map_lens = match (&mut self.map_lens, &mut self.map_reader) {
            (&mut Some(ref lens), &mut Ok(ref mut map_reader)) => {
                Some(lens.update(map_reader, self.x, self.y))
            },
            (&mut None, &mut Ok(ref mut map_reader)) => {
                Some(MapLens::new(map_reader, self.x, self.y, MAX_BLOCKS_WIDTH, MAX_BLOCKS_HEIGHT))
            },
            _ => None
        };
        self.map_lens = map_lens;
    }

    pub fn draw_page(&mut self, renderer: &mut Renderer, engine_data: &mut EngineData) {
        self.refresh_lens();

        self.texture = match self.map_lens {
            Some(ref lens) => {
                let mut surface = Surface::new(1024, 768, PixelFormatEnum::RGBA8888).unwrap();
                let block_drawer = match self.mode {
                    MapRenderMode::HeightMap => draw_heightmap_block,
                    MapRenderMode::RadarMap => draw_radarcol_block,
                };
                for y in 0..lens.height {
                    for x in 0..lens.width {
                        match lens.blocks[(x + (y * MAX_BLOCKS_WIDTH)) as usize] {
                            Some(ref block) => {
                                let block_surface = block_drawer(block, &self.radar_colors);
                                block_surface.blit(None, &mut surface, Some(Rect::new(x as i32 * 8, y as i32* 8, block_surface.width(), block_surface.height()))).unwrap();
                            },
                            _ => ()
                        }
                    }
                }
                Some(renderer.create_texture_from_surface(&surface).unwrap())
            },
            None => None
        }
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
                if self.x >= STEP_X as u32 {
                    self.x -= STEP_X as u32;
                    self.draw_page(renderer, engine_data);
                }
                None
            },
            Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                self.x += STEP_X as u32;
                self.draw_page(renderer, engine_data);
                None
            },
            Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                if self.y >= STEP_Y as u32 {
                    self.y -= STEP_Y as u32;
                    self.draw_page(renderer, engine_data);
                }
                None
            },
            Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                self.y += STEP_Y as u32;
                self.draw_page(renderer, engine_data);
                None
            },
            Event::KeyDown { keycode: Some(Keycode::Num1), .. } => {
                self.mode = MapRenderMode::HeightMap;
                self.draw_page(renderer, engine_data);
                None
            },
            Event::KeyDown { keycode: Some(Keycode::Num2), .. } => {
                self.mode = MapRenderMode::RadarMap;
                self.draw_page(renderer, engine_data);
                None
            },
             _ => None
        }
    }
}
