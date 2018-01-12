use engine::EngineData;
use image_convert::image_to_surface;
use scene::SceneName;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::Rect;
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

const STEP_X: u32 = 1;
const STEP_Y: u32 = 1;
const MAX_BLOCKS_WIDTH: u32 = 2;
const MAX_BLOCKS_HEIGHT: u32 = 2;

pub struct WorldScene<'a> {
    texture_creator: &'a TextureCreator<WindowContext>,
    reader: ArtReader<File>,
    facet: Facet,
    map_id: u8,
    texture: Option<Texture<'a>>,
    exiting: bool,
}

impl<'a> WorldScene<'a> {
    pub fn new<'b>(engine_data: &mut EngineData<'b>, texture_creator: &'a TextureCreator<WindowContext>) -> BoxedScene<'a, Event, SceneName, EngineData<'b>> {
        let reader = ArtReader::new(&Path::new("./assets/artidx.mul"), &Path::new("./assets/art.mul")).expect("Could not load art");
        let mut scene = Box::new(WorldScene {
            texture: None,
            exiting: false,
            map_id: 0,
            texture_creator,
            reader,
            facet: map_id_to_facet(0),
        });
        scene.draw_page(engine_data);
        scene
    }

    pub fn draw_page(&mut self, _engine_data: &mut EngineData) {
        let lens = self.facet.get_lens(MAX_BLOCKS_WIDTH, MAX_BLOCKS_HEIGHT);

        self.texture = match lens {
            Some(lens) => {
                let mut surface = Surface::new(1024, 768, PixelFormatEnum::RGBA8888).unwrap();
                for y in 0..lens.height {
                    for x in 0..lens.width {
                        match &lens.blocks[(x + (y * MAX_BLOCKS_WIDTH)) as usize] {
                            &(Some(ref block), Some(ref statics)) => {
                                let block_surface = self.draw_block(block, statics);
                                block_surface.blit(None, &mut surface, Some(Rect::new((22 * 8) + (22 * 8) * x as i32 - (y as i32 * (22 * 8)), (22 * 8) * y as i32 + (x as i32 * (22 * 8)), block_surface.width(), block_surface.height()))).unwrap();
                            },
                            &(Some(ref block), _) => {
                                let block_surface = self.draw_block(block, &vec![]);
                                block_surface.blit(None, &mut surface, Some(Rect::new((22 * 8) + (22 * 8) * x as i32 - (y as i32 * (22 * 8)), (22 * 8) * y as i32 + (x as i32 * (22 * 8)), block_surface.width(), block_surface.height()))).unwrap();
                                //block_surface.blit(None, &mut surface, Some(Rect::new(x as i32 * 8, y as i32* 8, block_surface.width(), block_surface.height()))).unwrap();
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

    pub fn draw_block(&mut self, block: &Block, statics: &Vec<StaticLocation>) -> Surface<'static> {
        let mut surface = Surface::new(44 * 8, 44 * 8, PixelFormatEnum::RGBA8888).unwrap();
        for y in 0..8 {
            for x in 0..8 {
                let cell = block.cells[y * 8 + x];
                self.reader.read_tile(cell.graphic as u32).map(|tile| {
                    let image = tile.to_image();
                    let tile_image = image_to_surface(&image);
                    tile_image.blit(None, &mut surface, Some(Rect::new((22 * 7) + (22 * x as i32) - (y as i32 * 22), (22 * y as i32) + (x as i32 * 22), 44, 44))).expect("Failed to blit texture");
                });
            }
        }
        surface
    }

}

impl<'a, 'b> Scene<Event, SceneName, EngineData<'b>> for WorldScene<'a> {
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
            Event::KeyDown { keycode: Some(Keycode::Tab), .. } => {
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
