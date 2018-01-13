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
use map::{Facet, map_id_to_facet, MAP_DETAILS};
use uorustlibs::map::{Block, StaticLocation};
use uorustlibs::art::{ArtReader, Art};
use caches::art_cache::ArtCache;

const STEP_X: u32 = 1;
const STEP_Y: u32 = 1;
const MAX_BLOCKS_WIDTH: u32 = 2;
const MAX_BLOCKS_HEIGHT: u32 = 2;

pub struct WorldScene<'a> {
    art_cache: ArtCache<'a>,
    facet: Facet,
    map_id: u8,
    exiting: bool,
}

impl<'a> WorldScene<'a> {
    pub fn new<'b>(engine_data: &mut EngineData<'b>, texture_creator: &'a TextureCreator<WindowContext>) -> BoxedScene<'a, Event, SceneName, EngineData<'b>> {
        let reader = ArtReader::new(&Path::new("./assets/artidx.mul"), &Path::new("./assets/art.mul")).expect("Could not load art");
        let scene = Box::new(WorldScene {
            exiting: false,
            map_id: 0,
            facet: map_id_to_facet(0),
            art_cache: ArtCache::new(texture_creator),
        });
        scene
    }

    pub fn draw_page(&mut self, renderer: &mut WindowCanvas) {
        self.facet.get_lens(MAX_BLOCKS_WIDTH, MAX_BLOCKS_HEIGHT).map(|lens| {
            for y in 0..lens.height {
                for x in 0..lens.width {
                    let (ref block, ref statics) = lens.blocks[(x + (y * MAX_BLOCKS_WIDTH)) as usize];
                    self.draw_block(renderer, block, statics, &Point::new((22 * 8) + (22 * 8) * x as i32 - (y as i32 * (22 * 8)), (22 * 8) * y as i32 + (x as i32 * (22 * 8))));
                }
            }
        });
    }

    pub fn draw_block(&mut self, renderer: &mut WindowCanvas, block: &Option<Block>, statics: &Option<Vec<StaticLocation>>, target: &Point){
        block.map(|block| {
            for y in 0..8 {
                for x in 0..8 {
                    let cell = block.cells[y * 8 + x];
                    self.art_cache.read_tile(cell.graphic as u32).as_ref().map(|tile| {
                        let query = tile.query();
                        let (offset_x, offset_y) = (target.x(), target.y());
                        renderer.copy(&tile, None, Rect::new((22 * 7) + (22 * x as i32) - (y as i32 * 22) + offset_x, (22 * y as i32) + (x as i32 * 22) + offset_y, query.width, query.height));
                    });
                    /*self.reader.read_tile(cell.graphic as u32).map(|tile| {
                        let image = tile.to_image();
                        let tile_image = image_to_surface(&image);
                        tile_image.blit(None, &mut surface, Some(Rect::new((22 * 7) + (22 * x as i32) - (y as i32 * 22), (22 * y as i32) + (x as i32 * 22), 44, 44))).expect("Failed to blit texture");
                    });*/
                }
            }
        });
    }

}

impl<'a, 'b> Scene<Event, SceneName, EngineData<'b>> for WorldScene<'a> {
    fn render(&mut self, renderer: &mut WindowCanvas, _engine_data: &mut EngineData, _tick: u64) {
        renderer.clear();
        self.draw_page(renderer);
        /*match self.texture {
            Some(ref texture) => {
                renderer.copy(texture, None, None).unwrap();
            },
            None => ()
        };*/
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
                }
            },
            Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                self.facet.x += STEP_X as u32;
            },
            Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                if self.facet.y >= STEP_Y as u32 {
                    self.facet.y -= STEP_Y as u32;
                }
            },
            Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                self.facet.y += STEP_Y as u32;
            },
            Event::KeyDown { keycode: Some(Keycode::Tab), .. } => {
                self.map_id = (self.map_id + 1) % MAP_DETAILS.len() as u8;
                self.facet = map_id_to_facet(self.map_id);
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
