use caches::art_cache::ArtCache;
use caches::facet_cache::Altitudes;
use caches::texmap_cache::TexMapCache;
use cgmath::Point2;
use ggez::graphics::{self, DrawParam, Image, Mesh, MeshBuilder, Vertex};
use ggez::{Context, GameResult};
use std::cmp::Ordering;
use uorustlibs::map::{Block, StaticLocation};
use uorustlibs::tiledata::{Flags, MapTileData, StaticTileData};

pub const TILE_SIZE: f32 = 44.0;

enum DrawableItem {
    Static(Image, StaticTileData),
    Tile(Image, MapTileData),
    Skewable(Mesh, MapTileData),
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

pub fn generate_vertices(params: &[[f32; 2]; 4]) -> [Vertex; 4] {
    [
        Vertex {
            pos: params[0],
            uv: [0.0, 0.0],
            color: [1.0, 1.0, 1.0, 1.0],
        },
        Vertex {
            pos: params[1],
            uv: [1.0, 0.0],
            color: [1.0, 1.0, 1.0, 1.0],
        },
        Vertex {
            pos: params[2],
            uv: [1.0, 1.0],
            color: [1.0, 1.0, 1.0, 1.0],
        },
        Vertex {
            pos: params[3],
            uv: [0.0, 1.0],
            color: [1.0, 1.0, 1.0, 1.0],
        },
    ]
}

pub fn skew(ctx: &mut Context, tile: &Image, altitudes: &Altitudes) -> Mesh {
    let top_right = altitudes.x2y1 - altitudes.x1y1;
    let bottom_right = altitudes.x2y2 - altitudes.x1y1;
    let bottom_left = altitudes.x1y2 - altitudes.x1y1;

    let increment = 4.0;
    let vertices = generate_vertices(&[
        [0.5 * TILE_SIZE, 0.0],
        [
            1.0 * TILE_SIZE,
            0.5 * TILE_SIZE - (increment * top_right as f32),
        ],
        [
            0.5 * TILE_SIZE,
            1.0 * TILE_SIZE - (increment * bottom_right as f32),
        ],
        [0.0, 0.5 * TILE_SIZE - (increment * bottom_left as f32)],
    ]);
    MeshBuilder::new()
        .raw(&vertices, &[0, 1, 2, 0, 2, 3], Some(tile.clone()))
        .expect("Failed to create raw mesh")
        .build(ctx)
        .expect("Failed to generate mesh")
}

pub fn draw_block(
    ctx: &mut Context,
    art_cache: &mut ArtCache,
    texmap_cache: &mut TexMapCache,
    maybe_block: Option<&Block>,
    statics: &Vec<StaticLocation>,
    altitudes: &Vec<Altitudes>,
    transform: Point2<f32>,
) -> GameResult<()> {
    for y in 0..(8 as usize) {
        for x in 0..(8 as usize) {
            let cell_statics = statics
                .iter()
                .filter(|s| s.x == x as u8 && s.y == y as u8)
                .collect::<Vec<&StaticLocation>>();
            let mut tiles: Vec<(DrawableItem, Point2<f32>, i8)> = vec![];
            match maybe_block {
                Some(block) => {
                    let cell = block.cells[y * 8 + x];
                    let altitudes = altitudes[y * 8 + x];
                    let cell_height = altitudes.x1y1;
                    let cell_x2y1_height = altitudes.x2y1;
                    let cell_x1y2_height = altitudes.x1y2;
                    let cell_x2y2_height = altitudes.x2y2;

                    let data: Option<(Image, MapTileData)> = match art_cache
                        .read_tile(ctx, cell.graphic as u32)
                    {
                        Some((ref tile, ref tiledata)) => Some((tile.clone(), tiledata.clone())),
                        _ => None,
                    };

                    data.map(|(tile, tiledata)| {
                        let new_transform = add(
                            add(cell_at(x as i32, y as i32), transform),
                            Point2::new(0.0, -(cell.altitude as f32 * 4.0)),
                        );
                        if cell_height == cell_x1y2_height
                            && cell.altitude == cell_x2y1_height
                            && cell.altitude == cell_x2y2_height
                        {
                            tiles.push((
                                DrawableItem::Tile(tile.clone(), tiledata),
                                new_transform,
                                cell.altitude,
                            ));
                        } else {
                            texmap_cache
                                .read_texmap(ctx, tiledata.texture_id as u32)
                                .as_ref()
                                .map(|tile| {
                                    let skewed = skew(ctx, tile, &altitudes);
                                    tiles.push((
                                        DrawableItem::Skewable(skewed, tiledata),
                                        new_transform,
                                        cell.altitude - 1,
                                    ));
                                });
                        }
                    });
                }
                None => (),
            };
            for s in cell_statics {
                art_cache.read_static(ctx, s.object_id as u32).as_ref().map(
                    |(ref art, ref tiledata)| {
                        let new_transform = add(
                            add(cell_at(x as i32, y as i32), transform),
                            Point2::new(
                                0.0,
                                -(s.altitude as f32 * 4.0) - art.height() as f32 + TILE_SIZE,
                            ),
                        );
                        tiles.push((
                            DrawableItem::Static(art.clone(), tiledata.clone()),
                            new_transform,
                            s.altitude,
                        ));
                    },
                );
            }
            tiles.sort_by(|a, b| match a.2.cmp(&b.2) {
                Ordering::Equal => match (&a.0, &b.0) {
                    (&DrawableItem::Static(_, ref tiledata), DrawableItem::Static(_, _)) => {
                        if tiledata.flags & Flags::BackgroundFlag as u32 != 0 {
                            Ordering::Less
                        } else {
                            Ordering::Greater
                        }
                    }
                    (&DrawableItem::Static(_, _), _) => Ordering::Greater,
                    _ => Ordering::Less,
                },
                otherwise => otherwise,
            });
            for (gfx, point, _) in tiles {
                match gfx {
                    DrawableItem::Static(ref img, _) => {
                        graphics::draw(ctx, img, DrawParam::default().dest(point))
                    }
                    DrawableItem::Tile(ref img, _) => {
                        graphics::draw(ctx, img, DrawParam::default().dest(point))
                    }
                    DrawableItem::Skewable(ref img, _) => {
                        graphics::draw(ctx, img, DrawParam::default().dest(point))
                    }
                }?;
            }
        }
    }
    Ok(())
}
