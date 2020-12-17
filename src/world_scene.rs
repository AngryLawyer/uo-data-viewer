use caches::art_cache::ArtCache;
use caches::facet_cache::Altitudes;
use caches::texmap_cache::TexMapCache;
use cgmath::Point2;
use ggez::event::{KeyCode, KeyMods};
use ggez::graphics::{self, DrawParam, Image, Skewable, Drawable};
use ggez::{Context, GameResult};
use map::{map_id_to_facet, Facet, MAP_DETAILS};
use scene::{BoxedScene, Scene, SceneChangeEvent, SceneName};
use uorustlibs::map::{Block, StaticLocation};
use uorustlibs::tiledata::{MapTileData, StaticTileData, TileDataReader, Flags};
use std::cmp::Ordering;

const STEP_X: u32 = 1;
const STEP_Y: u32 = 1;
const MAX_BLOCKS_WIDTH: u32 = 6;
const MAX_BLOCKS_HEIGHT: u32 = 6;

enum DrawableItem {
    Static(Image, StaticTileData),
    Tile(Image, MapTileData),
    Skewable(Skewable, MapTileData)
}

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

fn skew(ctx: &mut Context, tile: &Image, altitudes: &Altitudes) -> Skewable {
    let small_tile_ratio = 0.6875;
    let large_tile_ratio = 0.34375;
    let top_right = altitudes.x2y1 - altitudes.x1y1;
    let bottom_right = altitudes.x2y2 - altitudes.x1y1;
    let bottom_left = altitudes.x1y2 - altitudes.x1y1;
    
    let skewed = if (tile.width() == 64) {
        let increment = 4.0 / 64.0;
        Skewable::new(
            ctx,
            tile.clone(),
            [
                [0.5 * small_tile_ratio, 0.0],
                [1.0 * small_tile_ratio, 0.5 * small_tile_ratio - (increment * top_right as f32)],
                [0.5 * small_tile_ratio, 1.0 * small_tile_ratio - (increment * bottom_right as f32)],
                [0.0, 0.5 * small_tile_ratio - (increment * bottom_left as f32)],
            ],
        )
    } else {
        let increment = 4.0 / 128.0;
        Skewable::new(
            ctx,
            tile.clone(),
            [
                [0.5 * large_tile_ratio, 0.0],
                [1.0 * large_tile_ratio, 0.5 * large_tile_ratio - (increment * top_right as f32)],
                [0.5 * large_tile_ratio, 1.0 * large_tile_ratio - (increment * bottom_right as f32)],
                [0.0, 0.5 * large_tile_ratio - (increment * bottom_left as f32)],
            ],
        )
    };
    skewed
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
                self.draw_block(ctx, block, statics, altitudes, transform)?;
            }
        }
        Ok(())
    }

    pub fn draw_block(
        &mut self,
        ctx: &mut Context,
        block: &Block,
        statics: &Vec<StaticLocation>,
        altitudes: &Vec<Altitudes>,
        transform: Point2<f32>,
    ) -> GameResult<()> {
        for y in 0..(8 as usize) {
            for x in 0..(8 as usize) {
                let cell_statics = statics.iter().filter(|s| s.x == x as u8 && s.y == y as u8).collect::<Vec<&StaticLocation>>();
                let mut tiles: Vec<(DrawableItem, Point2<f32>, i8)> = vec![];
                let cell = block.cells[y * 8 + x];
                let altitudes = altitudes[y * 8 + x];
                let cell_height = altitudes.x1y1;
                let cell_x2y1_height = altitudes.x2y1;
                let cell_x1y2_height = altitudes.x1y2;
                let cell_x2y2_height = altitudes.x2y2;

                let data: Option<(Image, MapTileData)> = match self.art_cache.read_tile(ctx, cell.graphic as u32) {
                    Some((ref tile, ref tiledata)) => Some((tile.clone(), tiledata.clone())),
                    _ => None
                };
                
                
                data.map(|(tile, tiledata)| {
                    let new_transform = add(
                        add(cell_at(x as i32, y as i32), transform),
                        Point2::new(0.0, -(cell.altitude as f32 * 4.0)),
                    );
                    if (cell_height == cell_x1y2_height
                        && cell.altitude == cell_x2y1_height
                        && cell.altitude == cell_x2y2_height)
                    {
                        tiles.push((DrawableItem::Tile(tile.clone(), tiledata), new_transform, cell.altitude));
                    } else {
                        self.texmap_cache
                            .read_texmap(ctx, tiledata.texture_id as u32)
                            .as_ref()
                            .map(|tile| {
                                let skewed = skew(ctx, tile, &altitudes);
                                tiles.push((DrawableItem::Skewable(skewed, tiledata), new_transform, cell.altitude - 1));
                            });
                    }
                });
                for s in cell_statics {
                    self.art_cache
                        .read_static(ctx, s.object_id as u32)
                        .as_ref()
                        .map(|(ref art, ref tiledata)| {
                            let new_transform = add(
                                add(cell_at(x as i32, y as i32), transform),
                                Point2::new(0.0, -(s.altitude as f32 * 4.0) - art.height() as f32 + 44.0),
                            );
                            tiles.push((DrawableItem::Static(art.clone(), tiledata.clone()), new_transform, s.altitude));
                        });
                }
                tiles.sort_by(|a, b| match a.2.cmp(&b.2) {
                    Ordering::Equal => {
                        match (&a.0, &b.0) {
                            (&DrawableItem::Static(_, ref tiledata), DrawableItem::Static(_, _)) => {
                                if (tiledata.flags & Flags::BackgroundFlag as u32 != 0) {
                                    Ordering::Less
                                } else {
                                    Ordering::Greater
                                }
                            }
                            (&DrawableItem::Static(_, ref tiledata), _) => Ordering::Greater,
                            _ => Ordering::Less
                        }
                    },
                    otherwise => otherwise
                });
                for (gfx, point, _) in tiles {
                    match gfx {
                        DrawableItem::Static(ref img, ref tiledata) => {
                            graphics::draw(ctx, img, DrawParam::default().dest(point))
                        },
                        DrawableItem::Tile(ref img, ref tiledata) => graphics::draw(ctx, img, DrawParam::default().dest(point)),
                        DrawableItem::Skewable(ref img, ref tiledata) => graphics::draw(ctx, img, DrawParam::default().dest(point))
                    }?;
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
