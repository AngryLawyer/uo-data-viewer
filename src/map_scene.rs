use engine::EngineData;
use scene::SceneName;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::render::{WindowCanvas, Texture, TextureCreator};
use sdl2::surface::Surface;
use sdl2::video::WindowContext;
use sdl2_engine_helpers::scene::{Scene, BoxedScene, SceneChangeEvent};
use std::io::Result;
use std::path::Path;
use uorustlibs::color::{Color16, Color as ColorTrait};
use uorustlibs::map::{Block, RadarColReader, StaticLocation};
use map::{Facet, map_id_to_facet, MAP_DETAILS};

const MAX_BLOCKS_WIDTH: u32 = 1024 / 8;
const MAX_BLOCKS_HEIGHT: u32 = 768 / 8;
const STEP_X: u32 = MAX_BLOCKS_WIDTH / 4;
const STEP_Y: u32 = MAX_BLOCKS_HEIGHT / 4;

enum MapRenderMode {
    HeightMap,
    RadarMap,
    StaticsMap,
}

pub struct MapScene<'a> {
    texture_creator: &'a TextureCreator<WindowContext>,
    facet: Facet,
    map_id: u8,
    radar_colors: Result<Vec<Color16>>,
    mode: MapRenderMode,
    texture: Option<Texture<'a>>,
    exiting: bool
}

pub fn draw_heightmap_block(block: &Block, _statics: &Vec<StaticLocation>, _radar_cols: &Result<Vec<Color16>>) -> Surface<'static> {
    let mut surface = Surface::new(8, 8, PixelFormatEnum::RGBA8888).unwrap();
    surface.with_lock_mut(|bitmap| {
        for y in 0..8 {
            for x in 0..8 {
                let target = x + (y * 8);
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

pub fn draw_radarcol_block(block: &Block, _statics: &Vec<StaticLocation>, radar_cols: &Result<Vec<Color16>>) -> Surface<'static> {
    let mut surface = Surface::new(8, 8, PixelFormatEnum::RGBA8888).unwrap();
    surface.with_lock_mut(|bitmap| {
        for y in 0..8 {
            for x in 0..8 {

                let target = x + (y * 8);
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

impl<'a> MapScene<'a> {
    pub fn new<'b>(engine_data: &mut EngineData<'b>, texture_creator: &'a TextureCreator<WindowContext>) -> BoxedScene<'a, Event, SceneName, EngineData<'b>> {
        let colors = RadarColReader::new(&Path::new("./assets/radarcol.mul"))
            .and_then(|mut reader| reader.read_colors());

        let mut scene = Box::new(MapScene {
            facet: map_id_to_facet(0),
            map_id: 0,
            texture: None,
            mode: MapRenderMode::HeightMap,
            radar_colors: colors,
            exiting: false,
            texture_creator
        });

        scene.draw_page(engine_data);
        scene
    }

    pub fn draw_page(&mut self, _engine_data: &mut EngineData) {
        let lens = self.facet.get_lens(MAX_BLOCKS_WIDTH, MAX_BLOCKS_HEIGHT);

        self.texture = match lens {
            Some(lens) => {
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
                Some(self.texture_creator.create_texture_from_surface(&surface).unwrap())
            },
            None => None
        }
    }
}

impl<'a, 'b> Scene<Event, SceneName, EngineData<'b>> for MapScene<'a> {
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
                self.exiting = true
            },
            Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                if self.facet.x >= STEP_X as u32 {
                    self.facet.x -= STEP_X as u32;
                    self.draw_page(engine_data);
                }
            },
            Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                self.facet.x += STEP_X as u32;
                self.draw_page(engine_data);
            },
            Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                if self.facet.y >= STEP_Y as u32 {
                    self.facet.y -= STEP_Y as u32;
                    self.draw_page(engine_data);
                }
            },
            Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                self.facet.y += STEP_Y as u32;
                self.draw_page(engine_data);
            },
            Event::KeyDown { keycode: Some(Keycode::Num1), .. } => {
                self.mode = MapRenderMode::HeightMap;
                self.draw_page(engine_data);
            },
            Event::KeyDown { keycode: Some(Keycode::Num2), .. } => {
                self.mode = MapRenderMode::RadarMap;
                self.draw_page(engine_data);
            },
            Event::KeyDown { keycode: Some(Keycode::Num3), .. } => {
                self.mode = MapRenderMode::StaticsMap;
                self.draw_page(engine_data);
            },
            Event::KeyDown { keycode: Some(Keycode::Tab), .. } => {
                self.mode = MapRenderMode::HeightMap;
                self.map_id = (self.map_id + 1) % MAP_DETAILS.len() as u8;
                self.facet = map_id_to_facet(self.map_id);
                self.draw_page(engine_data);
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
