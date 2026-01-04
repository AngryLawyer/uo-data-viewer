use ggez::input::keyboard::{KeyCode, KeyInput};
use ggez::graphics::{Canvas, Color, DrawParam, Image, ImageFormat, ScreenImage};
use ggez::{Context, GameResult};
use ggez::glam::Vec2;
use image::math::Rect;
use crate::scene::{BoxedScene, Scene, SceneChangeEvent, SceneName};
use std::collections::HashMap;
use std::path::Path;

use crate::map::{map_id_to_facet, Facet, MAP_DETAILS};
use uorustlibs::color::{Color as ColorTrait, Color16};
use uorustlibs::map::{Block, RadarColReader, StaticLocation};

const MAX_BLOCKS_WIDTH: u32 = 800 / 8;
const MAX_BLOCKS_HEIGHT: u32 = 600 / 8;
const STEP_X: u32 = MAX_BLOCKS_WIDTH / 4;
const STEP_Y: u32 = MAX_BLOCKS_HEIGHT / 4;

enum MapRenderMode {
    Height,
    Radar,
    Statics,
    Full,
}

pub struct MapScene {
    facet: Option<Facet>,
    map_id: u8,
    radar_colors: Option<Vec<Color16>>,
    mode: MapRenderMode,
    rendered_blocks: HashMap<(u32, u32), Image>,
    exiting: bool,
    x: u32,
    y: u32,
}

pub fn draw_heightmap_block(
    bitmap: &mut [u8; 4 * 8 * 8],
    block: &Block,
    _statics: &[StaticLocation],
    _radar_cols: Option<&[Color16]>,
) {
    for y in 0..8 {
        for x in 0..8 {
            let target = x + (y * 8);
            let height = (block.cells[target].altitude as i16 + 128) as u8;
            bitmap[target * 4] = height;
            bitmap[target * 4 + 1] = height;
            bitmap[target * 4 + 2] = height;
            bitmap[target * 4 + 3] = 255;
        }
    }
}

pub fn draw_radarcol_block(
    bitmap: &mut [u8; 4 * 8 * 8],
    block: &Block,
    _statics: &[StaticLocation],
    radar_cols: Option<&[Color16]>,
) {
    for y in 0..8 {
        for x in 0..8 {
            let target = x + (y * 8);
            let index = block.cells[target].graphic;
            let (r, g, b, _) = match radar_cols {
                Some(colors) => colors[index as usize].to_rgba(),
                _ => index.to_rgba(),
            };
            bitmap[target * 4] = r;
            bitmap[target * 4 + 1] = g;
            bitmap[target * 4 + 2] = b;
            bitmap[target * 4 + 3] = 255;
        }
    }
}

pub fn draw_statics_block(
    bitmap: &mut [u8; 4 * 8 * 8],
    _block: &Block,
    statics: &[StaticLocation],
    radar_cols: Option<&[Color16]>,
) {
    let mut last_height_locs = [-127; 64];
    for stat in statics {
        let lookup = (stat.x + (stat.y * 8)) as usize;
        if last_height_locs[lookup] < stat.altitude {
            // Paint the cell as we're higher.
            // Inefficient, but probably no worse than trying to keep track of which items are in which cell
            let (r, g, b, _) = match radar_cols {
                Some(colors) => colors[stat.color_idx() as usize].to_rgba(),
                _ => (0, 0, 0, 0),
            };
            bitmap[lookup * 4] = r;
            bitmap[lookup * 4 + 1] = g;
            bitmap[lookup * 4 + 2] = b;
            bitmap[lookup * 4 + 3] = 255;

            last_height_locs[lookup] = stat.altitude;
        }
    }
}

pub fn draw_full_block(
    bitmap: &mut [u8; 4 * 8 * 8],
    block: &Block,
    statics: &[StaticLocation],
    radar_cols: Option<&[Color16]>,
) {
    draw_radarcol_block(bitmap, block, statics, radar_cols);
    draw_statics_block(bitmap, block, statics, radar_cols);
}

impl<'a> MapScene {
    pub fn new(ctx: &mut Context) -> BoxedScene<'a, SceneName, ()> {
        let colors = RadarColReader::new(&Path::new("./assets/radarcol.mul"))
            .and_then(|mut reader| reader.read_colors())
            .ok();

        let scene = Box::new(MapScene {
            facet: map_id_to_facet(0).ok(),
            map_id: 0,
            rendered_blocks: HashMap::new(),
            mode: MapRenderMode::Full,
            radar_colors: colors,
            exiting: false,
            x: 0,
            y: 0,
        });

        scene
    }

    fn block_in_bounds(&self, x: u32, y: u32, screen_bounds: Rect) -> bool {
        // Screen only ever moves in something divisible by 8
        let pixel_x = x * 8;
        let pixel_y = y * 8;
        pixel_x >= screen_bounds.x &&
            pixel_x <= (screen_bounds.x + screen_bounds.width + 8) &&
            pixel_y >= screen_bounds.y &&
            pixel_y <= (screen_bounds.y + screen_bounds.height + 8)
    }

    pub fn draw_page(&mut self, ctx: &mut Context) -> GameResult<()> {
        let mut screen_canvas = Canvas::from_frame(ctx, Color::BLACK);
        if self.facet.is_none() {
            return screen_canvas.finish(ctx)
        }

        let (screen_width, screen_height) = ctx.gfx.drawable_size();
        let screen_bounds = Rect {
            x: self.x * 8,
            y: self.y * 8,
            width: screen_width as u32,
            height: screen_height as u32
        };

        let block_drawer = match self.mode {
            MapRenderMode::Height => draw_heightmap_block,
            MapRenderMode::Radar => draw_radarcol_block,
            MapRenderMode::Statics => draw_statics_block,
            MapRenderMode::Full => draw_full_block,
        };
        for y in 0..(screen_bounds.height / 8) {
            for x in 0..(screen_bounds.width / 8) {
                let block_surface = self.rendered_blocks.entry((x + self.x, y + self.y)).or_insert_with(|| {
                    let mut img = ScreenImage::new(
                        ctx,
                        None,
                        8.0 / screen_bounds.width as f32,
                        8.0 / screen_bounds.height as f32,
                        1
                    );
                    let mut canvas = Canvas::from_screen_image(ctx, &mut img, Color::BLACK);
                    let (block_data, _) = self.facet.as_mut().unwrap().read_block(x + self.x, y + self.y);
                    let mut bitmap = [0; 8 * 8 * 4];
                    if block_data.0.is_some() {
                        block_drawer(&mut bitmap, &block_data.0.unwrap(), &block_data.1, self.radar_colors.as_deref());
                    }
                    let block_surface = Image::from_pixels(ctx, &bitmap, ImageFormat::Rgba8Unorm, 8, 8);
                    canvas.draw(
                        &block_surface,
                        DrawParam::default(),
                    );
                    canvas.finish(ctx).unwrap(); // FIXME: This could blow up
                    img.image(ctx)
                });
                screen_canvas.draw(
                    block_surface,
                    DrawParam::default().dest(Vec2::new(x as f32 * 8.0, y as f32 * 8.0)),
                );
            }
        }
        screen_canvas.finish(ctx)
    }
}

impl Scene<SceneName, ()> for MapScene {
    fn draw(&mut self, ctx: &mut Context, _engine_data: &mut ()) -> GameResult<()> {
        self.draw_page(ctx)
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
                if self.x >= STEP_X {
                    self.x -= STEP_X;
                }
            }
            Some(KeyCode::Right) => {
                self.x += STEP_X;
            }
            Some(KeyCode::Up) => {
                if self.y >= STEP_Y {
                    self.y -= STEP_Y;
                }
            }
            Some(KeyCode::Down) => {
                self.y += STEP_Y;
            }
            Some(KeyCode::Key1) => {
                self.mode = MapRenderMode::Height;
                self.rendered_blocks.clear();
            }
            Some(KeyCode::Key2) => {
                self.mode = MapRenderMode::Radar;
                self.rendered_blocks.clear();
            }
            Some(KeyCode::Key3) => {
                self.mode = MapRenderMode::Statics;
                self.rendered_blocks.clear();
            }
            Some(KeyCode::Key4) => {
                self.mode = MapRenderMode::Full;
                self.rendered_blocks.clear();
            }
            Some(KeyCode::Tab) => {
                self.map_id = (self.map_id + 1) % MAP_DETAILS.len() as u8;
                self.facet = map_id_to_facet(self.map_id).ok();
                self.rendered_blocks.clear();
            }
            _ => (),
        }
    }
}
