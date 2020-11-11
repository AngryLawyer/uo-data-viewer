use cgmath::Point2;
use ggez::event::{KeyCode, KeyMods};
use ggez::graphics::{self, Canvas, DrawParam, Text};
use ggez::{timer, Context, GameResult};
use image_convert::frame_to_surface;
use scene::{BoxedScene, Scene, SceneChangeEvent, SceneName};
use std::cmp;
use std::fs::File;
use std::io::Result;
use std::path::Path;
use map::{Facet, map_id_to_facet, MAP_DETAILS};
use uorustlibs::map::{Block, StaticLocation};
use uorustlibs::art::{ArtReader, Art};
use caches::art_cache::ArtCache;

const STEP_X: u32 = 1;
const STEP_Y: u32 = 1;
const MAX_BLOCKS_WIDTH: u32 = 6;
const MAX_BLOCKS_HEIGHT: u32 = 6;

pub struct WorldScene {
    art_cache: ArtCache,
    facet: Facet,
    map_id: u8,
    exiting: bool,
}

fn add(a: Point2<f32>, b: Point2<f32>) -> Point2<f32> {
    Point2::new(a.x + b.x, a.y + b.y)
}

fn cell_at(x: i32, y: i32) -> Point2<f32> {
    Point2::new(
        ((22 * 7) + (22 * x) - (y * 22)) as f32,
        ((22 * y) + (x * 22)) as f32
    )
}

// TODO: Make this less nasty
fn block_at(x: i32, y: i32) -> Point2<f32> {
    Point2::new(
        ((22 * 8) * x - (y * (22 * 8)) + (22 * 16)) as f32,
        ((22 * 8) * y + (x * (22 * 8)) - (22 * 24)) as f32
    )
}

impl<'a> WorldScene {
    pub fn new() -> BoxedScene<'a, SceneName, ()> {
        let scene = Box::new(WorldScene {
            exiting: false,
            map_id: 0,
            facet: map_id_to_facet(0),
            art_cache: ArtCache::new(),
        });
        scene
    }

    pub fn draw_page(&mut self, ctx: &mut Context) {
        self.facet.get_lens(MAX_BLOCKS_WIDTH, MAX_BLOCKS_HEIGHT).map(|lens| {
            for y in 0..lens.height {
                for x in 0..lens.width {
                    let (ref block, ref statics) = lens.blocks[(x + (y * MAX_BLOCKS_WIDTH)) as usize];
                    let transform = block_at(x as i32, y as i32);
                    self.draw_block(ctx, block, statics, transform);
                }
            }
        });
    }

    pub fn draw_block(&mut self, ctx: &mut Context, block: &Option<Block>, statics: &Option<Vec<StaticLocation>>, transform: Point2<f32>){
        block.map(|block| {
            for y in 0..8 {
                for x in 0..8 {
                    let cell = block.cells[y * 8 + x];
                    self.art_cache.read_tile(ctx, cell.graphic as u32).as_ref().map(|tile| {
                        let new_transform = add(add(cell_at(x as i32, y as i32), transform), Point2::new(0.0, -cell.altitude as f32));
                        graphics::draw(
                            ctx,
                            tile,
                            DrawParam::default().dest(new_transform),
                        );
                    });
                }
            }
        });
    }
}

impl Scene<SceneName, ()> for WorldScene {
    fn draw(&mut self, ctx: &mut Context, _engine_data: &mut ()) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);
        self.draw_page(ctx);
        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
        _engine_data: &mut (),
    ) {
        match keycode {
            KeyCode::Escape => {
                self.exiting = true
            },
            KeyCode::Left => {
                if self.facet.x >= STEP_X as u32 {
                    self.facet.x -= STEP_X as u32;
                }
            },
            KeyCode::Right => {
                self.facet.x += STEP_X as u32;
            },
            KeyCode::Up => {
                if self.facet.y >= STEP_Y as u32 {
                    self.facet.y -= STEP_Y as u32;
                }
            },
            KeyCode::Down => {
                self.facet.y += STEP_Y as u32;
            },
            KeyCode::Tab => {
                self.map_id = (self.map_id + 1) % MAP_DETAILS.len() as u8;
                self.facet = map_id_to_facet(self.map_id);
            },
            _ => ()
        }
    }

    fn update(
        &mut self,
        _ctx: &mut Context,
        _engine_data: &mut (),
        ) -> GameResult<Option<SceneChangeEvent<SceneName>>> {
        if self.exiting {
            Ok(Some(SceneChangeEvent::PopScene))
        } else {
            Ok(None)
        }
    }
}
