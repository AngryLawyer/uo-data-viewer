use caches::art_cache::ArtCache;
use caches::texmap_cache::TexMapCache;
use cgmath::Point2;
use ggez::event::{KeyCode, KeyMods};
use ggez::graphics::{self};
use ggez::{Context, GameResult};
use map::render::draw_block;
use map::{map_id_to_facet, Facet, MAP_DETAILS};
use scene::{BoxedScene, Scene, SceneChangeEvent, SceneName};

const STEP_X: u32 = 1;
const STEP_Y: u32 = 1;
const MAX_BLOCKS_WIDTH: u32 = 6;
const MAX_BLOCKS_HEIGHT: u32 = 6;

pub struct WorldScene {
    art_cache: ArtCache,
    texmap_cache: TexMapCache,
    facet: Facet,
    x: u32,
    y: u32,
    map_id: u8,
    exiting: bool,
}

// TODO: Make this less nasty
fn block_at(x: i32, y: i32) -> Point2<f32> {
    Point2::new(
        ((22 * 8) * x - (y * (22 * 8)) + (22 * 16)) as f32,
        ((22 * 8) * y + (x * (22 * 8)) - (22 * 24)) as f32,
    )
}

impl<'a> WorldScene {
    pub fn new() -> BoxedScene<'a, SceneName, ()> {
        let scene = Box::new(WorldScene {
            exiting: false,
            map_id: 0,
            facet: map_id_to_facet(0),
            art_cache: ArtCache::new(),
            texmap_cache: TexMapCache::new(),
            x: 160,
            y: 208,
        });
        scene
    }

    pub fn draw_page(&mut self, ctx: &mut Context) -> GameResult<()> {
        for y in 0..MAX_BLOCKS_HEIGHT {
            for x in 0..MAX_BLOCKS_WIDTH {
                let ((ref block, ref statics), ref altitudes) =
                    self.facet.read_block(x + self.x, y + self.y);
                let transform = block_at(x as i32, y as i32);
                draw_block(
                    ctx,
                    &mut self.art_cache,
                    &mut self.texmap_cache,
                    Some(block),
                    statics,
                    altitudes,
                    transform,
                )?;
            }
        }
        Ok(())
    }
}

impl Scene<SceneName, ()> for WorldScene {
    fn draw(&mut self, ctx: &mut Context, _engine_data: &mut ()) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);
        self.draw_page(ctx)
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
            KeyCode::Escape => self.exiting = true,
            KeyCode::Left => {
                if self.x >= STEP_X as u32 {
                    self.x -= STEP_X as u32;
                }
            }
            KeyCode::Right => {
                self.x += STEP_X as u32;
            }
            KeyCode::Up => {
                if self.y >= STEP_Y as u32 {
                    self.y -= STEP_Y as u32;
                }
            }
            KeyCode::Down => {
                self.y += STEP_Y as u32;
            }
            KeyCode::Tab => {
                self.map_id = (self.map_id + 1) % MAP_DETAILS.len() as u8;
                self.facet = map_id_to_facet(self.map_id);
                self.x = 0;
                self.y = 0;
            }
            _ => (),
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
