use scene::{BoxedScene, Scene, SceneChangeEvent, SceneName};
use engine::EngineData;
use text_renderer::TextRenderer;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::surface::Surface;
use std::io::Result;
use std::path::Path;
use uorustlibs::color::{Color16, Color as ColorTrait};
use uorustlibs::map::{MapReader, Block, RadarColReader, StaticReader, StaticLocation};
use uorustlibs::map::map_size::{TRAMMEL, FELUCCA, ILSHENAR, MALAS};

use std::io::{Error};
use sdl2::render::{Renderer, Texture, TextureQuery};
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use map::lens::MapLens;
use map::Facet;

const MAX_BLOCKS_WIDTH: u32 = 1024 / 8;
const MAX_BLOCKS_HEIGHT: u32 = 768 / 8;
const STEP_X: u32 = MAX_BLOCKS_WIDTH / 4;
const STEP_Y: u32 = MAX_BLOCKS_HEIGHT / 4;

enum MapRenderMode {
    HeightMap,
    RadarMap,
    StaticsMap,
}

const MAP_DETAILS: [(&'static str, &'static str, &'static str, (u32, u32)); 4] = [
    ("./assets/map0.mul", "./assets/staidx0.mul", "./assets/statics0.mul", TRAMMEL),
    ("./assets/map0.mul", "./assets/staidx0.mul", "./assets/statics0.mul", FELUCCA),
    ("./assets/map2.mul", "./assets/staidx2.mul", "./assets/statics2.mul", ILSHENAR),
    ("./assets/map3.mul", "./assets/staidx3.mul", "./assets/statics3.mul", MALAS),
];

pub struct MapScene {
    facet: Facet,
    map_id: u8,
    radar_colors: Result<Vec<Color16>>,
    mode: MapRenderMode,
    texture: Option<Texture>,
}

pub fn draw_heightmap_block(block: &Block, statics: &Vec<StaticLocation>, radar_cols: &Result<Vec<Color16>>) -> Surface<'static> {
    let mut surface = Surface::new(8, 8, PixelFormatEnum::RGBA8888).unwrap();
    surface.with_lock_mut(|bitmap| {
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

pub fn draw_radarcol_block(block: &Block, statics: &Vec<StaticLocation>, radar_cols: &Result<Vec<Color16>>) -> Surface<'static> {
    let mut surface = Surface::new(8, 8, PixelFormatEnum::RGBA8888).unwrap();
    surface.with_lock_mut(|bitmap| {
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

pub fn draw_statics_block(block: &Block, statics: &Vec<StaticLocation>, radar_cols: &Result<Vec<Color16>>) -> Surface<'static> {
    let mut surface = draw_radarcol_block(block, statics, radar_cols);
    let mut last_height_locs = vec![-127; 64];
    surface.with_lock_mut(|bitmap| {
        for stat in statics {
            let lookup = (stat.x + (stat.y * 8)) as usize;
            if last_height_locs[lookup] < stat.altitude {
                // Paint the cell as we're higher.
                // Inefficient, but probably no worse than trying to keep track of which items are in which cell
                let (r, g, b, _) = match radar_cols {
                    &Ok(ref colors) => colors[stat.color_idx() as usize].to_rgba(),
                    _ => (0, 0, 0, 0)
                };
                bitmap[lookup * 4] = 255;
                bitmap[lookup * 4 + 1] = b;
                bitmap[lookup * 4 + 2] = g;
                bitmap[lookup * 4 + 3] = r;
                
                last_height_locs[lookup] = stat.altitude;
            }
        }
    });
    surface
}

pub fn map_id_to_facet(id: u8) -> Facet {
    let corrected_id = if id as usize >= MAP_DETAILS.len() {
        0
    } else {
        id as usize
    };
    let (map, idx, statics, (width, height)) = MAP_DETAILS[corrected_id];
    Facet::new(&Path::new(map), &Path::new(idx), &Path::new(statics), width / 8, height / 8)
}

impl MapScene {
    pub fn new<'a>(renderer: &mut Renderer, engine_data: &mut EngineData<'a>) -> BoxedScene<SceneName, EngineData<'a>> {
        let colors = RadarColReader::new(&Path::new("./assets/radarcol.mul"))
            .and_then(|mut reader| reader.read_colors());

        let mut scene = Box::new(MapScene {
            facet: map_id_to_facet(0),
            map_id: 0,
            texture: None,
            mode: MapRenderMode::HeightMap,
            radar_colors: colors,
        });

        scene.draw_page(renderer, engine_data);
        scene
    }

    pub fn draw_page(&mut self, renderer: &mut Renderer, engine_data: &mut EngineData) {
        let lens = self.facet.get_lens();

        self.texture = match lens {
            &Some(ref lens) => {
                let mut surface = Surface::new(1024, 768, PixelFormatEnum::RGBA8888).unwrap();
                let block_drawer = match self.mode {
                    MapRenderMode::HeightMap => draw_heightmap_block,
                    MapRenderMode::RadarMap => draw_radarcol_block,
                    MapRenderMode::StaticsMap => draw_statics_block,
                };
                for y in 0..lens.height {
                    for x in 0..lens.width {
                        match &lens.blocks[(x + (y * MAX_BLOCKS_WIDTH)) as usize] {
                            &(Some(ref block), Some(ref statics)) => {
                                let block_surface = block_drawer(block, statics, &self.radar_colors);
                                block_surface.blit(None, &mut surface, Some(Rect::new(x as i32 * 8, y as i32* 8, block_surface.width(), block_surface.height()))).unwrap();
                            },
                            &(Some(ref block), _) => {
                                let block_surface = block_drawer(block, &vec![], &self.radar_colors);
                                block_surface.blit(None, &mut surface, Some(Rect::new(x as i32 * 8, y as i32* 8, block_surface.width(), block_surface.height()))).unwrap();
                            },
                            _ => ()
                        }
                    }
                }
                Some(renderer.create_texture_from_surface(&surface).unwrap())
            },
            &None => None
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
                if self.facet.x >= STEP_X as u32 {
                    self.facet.x -= STEP_X as u32;
                    self.draw_page(renderer, engine_data);
                }
                None
            },
            Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                self.facet.x += STEP_X as u32;
                self.draw_page(renderer, engine_data);
                None
            },
            Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                if self.facet.y >= STEP_Y as u32 {
                    self.facet.y -= STEP_Y as u32;
                    self.draw_page(renderer, engine_data);
                }
                None
            },
            Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                self.facet.y += STEP_Y as u32;
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
            Event::KeyDown { keycode: Some(Keycode::Num3), .. } => {
                self.mode = MapRenderMode::StaticsMap;
                self.draw_page(renderer, engine_data);
                None
            },
            Event::KeyDown { keycode: Some(Keycode::Tab), .. } => {
                self.mode = MapRenderMode::HeightMap;
                self.map_id = (self.map_id + 1) % MAP_DETAILS.len() as u8;
                self.facet = map_id_to_facet(self.map_id);
                self.draw_page(renderer, engine_data);
                None
            },
             _ => None
        }
    }
}
