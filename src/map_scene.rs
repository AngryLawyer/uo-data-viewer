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

const MAX_BLOCKS_WIDTH: usize = 1024 / 8;
const MAX_BLOCKS_HEIGHT: usize = 768 / 8;
const STEP_X: usize = MAX_BLOCKS_WIDTH / 4;
const STEP_Y: usize = MAX_BLOCKS_HEIGHT / 4;

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
    texture: Option<Texture>
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
            radar_colors: colors
        });
        scene.draw_page(renderer, engine_data);
        scene
    }

    pub fn get_blocks(&mut self) -> Vec<Result<Block>> {
        let mut blocks = vec![];

        match self.map_reader {
            Ok(ref mut map_reader) => {
                for y in 0..MAX_BLOCKS_HEIGHT {
                    for x in 0..MAX_BLOCKS_WIDTH {
                        let block = map_reader.read_block_from_coordinates(self.x + x as u32, self.y + y as u32);
                        blocks.push(block);
                    }
                }
            },
            _ => ()
        };
        blocks
    }

    pub fn draw_page(&mut self, renderer: &mut Renderer, engine_data: &mut EngineData) {
        let blocks = self.get_blocks();
        let block_drawer = match self.mode {
            MapRenderMode::HeightMap => draw_heightmap_block,
            MapRenderMode::RadarMap => draw_radarcol_block,
        };
        let mut surface = Surface::new(1024, 768, PixelFormatEnum::RGBA8888).unwrap();
        for y in 0..MAX_BLOCKS_HEIGHT {
            for x in 0..MAX_BLOCKS_WIDTH {
                match blocks[x + (y * MAX_BLOCKS_WIDTH)] {
                    Ok(ref block) => {
                        let block_surface = block_drawer(block, &self.radar_colors);
                        block_surface.blit(None, &mut surface, Some(Rect::new(x as i32 * 8, y as i32* 8, block_surface.width(), block_surface.height()))).unwrap();
                    },
                    _ => ()
                }
            }
        }
        self.texture = Some(renderer.create_texture_from_surface(&surface).unwrap())
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
