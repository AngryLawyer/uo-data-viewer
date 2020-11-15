use caches::art_cache::ArtCache;
use caches::texmap_cache::TexMapCache;
use caches::facet_cache::Altitudes;
use cgmath::Point2;
use ggez::event::{KeyCode, KeyMods};
use ggez::graphics::{self, DrawParam};
use ggez::{Context, GameResult};
use map::{map_id_to_facet, Facet, MAP_DETAILS};
use scene::{BoxedScene, Scene, SceneChangeEvent, SceneName};
use uorustlibs::map::{Block, StaticLocation};

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

fn add(a: Point2<f32>, b: Point2<f32>) -> Point2<f32> {
    Point2::new(a.x + b.x, a.y + b.y)
}

fn cell_at(x: i32, y: i32) -> Point2<f32> {
    Point2::new(
        ((22 * 7) + (22 * x) - (y * 22)) as f32,
        ((22 * y) + (x * 22)) as f32,
    )
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
            x: 70,
            y: 70
        });
        scene
    }

    pub fn draw_page(&mut self, ctx: &mut Context) -> GameResult<()> {
        for y in 0..MAX_BLOCKS_HEIGHT {
            for x in 0..MAX_BLOCKS_WIDTH {
                let ((ref block, ref statics), ref altitudes) = self.facet.read_block(x + self.x, y + self.y);
                let transform = block_at(x as i32, y as i32);
                self.draw_block(ctx, block, statics, altitudes, transform)?
            }
        }
        Ok(())
    }

    pub fn draw_block(
        &mut self,
        ctx: &mut Context,
        block: &Block,
        _statics: &Vec<StaticLocation>,
        altitudes: &Vec<Altitudes>,
        transform: Point2<f32>,
    ) -> GameResult<()> {
        for y in 0..8 {
            for x in 0..8 {
                let cell = block.cells[y * 8 + x];
                let altitudes = altitudes[y * 8 + x];
                let cell_height = altitudes.x1y1;
                let cell_x2y1_height = altitudes.x2y1;
                let cell_x1y2_height = altitudes.x1y2;
                let cell_x2y2_height = altitudes.x2y2;
                if (cell_height == cell_x1y2_height && cell.altitude == cell_x2y1_height && cell.altitude == cell_x2y2_height) {
                    self.art_cache
                        .read_tile(ctx, cell.graphic as u32)
                        .as_ref()
                        .map(|tile| {
                                let new_transform = add(
                                    add(cell_at(x as i32, y as i32), transform),
                                    Point2::new(0.0, -cell.altitude as f32)
                                );
                                graphics::draw(ctx, tile, DrawParam::default().dest(new_transform))
                                })
                    .unwrap_or(Ok(()))?;
                } else {
                    self.texmap_cache
                        .read_texmap(ctx, cell.graphic as u32)
                        .as_ref()
                        .map(|tile| {
                                let new_transform = add(
                                        add(cell_at(x as i32, y as i32), transform),
                                        Point2::new(0.0, -cell.altitude as f32),
                                        );
                                graphics::draw(ctx, tile, DrawParam::default().dest(new_transform))
                                })
                    .unwrap_or(Ok(()))?;
                }
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
