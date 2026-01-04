use crate::caches::art_cache::ArtCache;
use crate::caches::texmap_cache::TexMapCache;
use crate::image_convert::image_to_surface;
use crate::map::render::draw_block;
use crate::map::{Facet, MAP_DETAILS, map_id_to_facet};
use crate::scene::{BoxedScene, Scene, SceneChangeEvent, SceneName};
use ggez::event::MouseButton;
use ggez::glam::Vec2;
use ggez::graphics::{Canvas, Color, DrawParam, Image, Text};
use ggez::input::keyboard::{KeyCode, KeyInput};
use ggez::{Context, GameResult};
use std::fs::File;
use std::io::Result;
use std::path::Path;
use uorustlibs::art::{Art, ArtReader};
use uorustlibs::tiledata::{StaticTileData, TileDataReader};

const STEP_X: u32 = 1;
const STEP_Y: u32 = 1;
const MAX_BLOCKS_WIDTH: u32 = 6;
const MAX_BLOCKS_HEIGHT: u32 = 6;

pub struct WorldScene {
    art_cache: ArtCache,
    texmap_cache: TexMapCache,
    facet: Result<Facet>,
    x: u32,
    y: u32,
    map_id: u8,
    exiting: bool,
}

// TODO: Make this less nasty
fn block_at(x: i32, y: i32) -> Vec2 {
    Vec2::new(
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
        let mut canvas = Canvas::from_frame(ctx, Color::BLACK);
        if let Ok(ref mut facet) = self.facet {
            for y in 0..MAX_BLOCKS_HEIGHT {
                for x in 0..MAX_BLOCKS_WIDTH {
                    let (block_details, altitudes) = facet.read_block(x + self.x, y + self.y);
                    let transform = block_at(x as i32, y as i32);
                    draw_block(
                        ctx,
                        &mut canvas,
                        &mut self.art_cache,
                        &mut self.texmap_cache,
                        block_details.0.as_ref(),
                        &block_details.1,
                        &*altitudes.unwrap(),
                        transform,
                    )?;
                }
            }
        }
        canvas.finish(ctx)
    }
}

impl Scene<SceneName, ()> for WorldScene {
    fn draw(&mut self, ctx: &mut Context, _engine_data: &mut ()) -> GameResult<()> {
        self.draw_page(ctx)
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keyinput: KeyInput,
        _repeat: bool,
        _engine_data: &mut (),
    ) {
        match keyinput.keycode {
            Some(KeyCode::Escape) => self.exiting = true,
            Some(KeyCode::Left) => {
                if self.x >= STEP_X as u32 {
                    self.x -= STEP_X as u32;
                }
            }
            Some(KeyCode::Right) => {
                self.x += STEP_X as u32;
            }
            Some(KeyCode::Up) => {
                if self.y >= STEP_Y as u32 {
                    self.y -= STEP_Y as u32;
                }
            }
            Some(KeyCode::Down) => {
                self.y += STEP_Y as u32;
            }
            Some(KeyCode::Tab) => {
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
