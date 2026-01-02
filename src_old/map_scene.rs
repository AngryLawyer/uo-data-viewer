use cgmath::Point2;
use ggez::event::{KeyCode, KeyMods};
use ggez::graphics::{self, Canvas, DrawParam, Image};
use ggez::{Context, GameResult};
use map::{map_id_to_facet, Facet, MAP_DETAILS};
use scene::{BoxedScene, Scene, SceneChangeEvent, SceneName};
use std::io::Result;
use std::path::Path;
use uorustlibs::color::{Color as ColorTrait, Color16};
use uorustlibs::map::{Block, RadarColReader, StaticLocation};

const MAX_BLOCKS_WIDTH: u32 = 800 / 8;
const MAX_BLOCKS_HEIGHT: u32 = 600 / 8;
const STEP_X: u32 = MAX_BLOCKS_WIDTH / 4;
const STEP_Y: u32 = MAX_BLOCKS_HEIGHT / 4;

enum MapRenderMode {
    HeightMap,
    RadarMap,
    StaticsMap,
    FullMap,
}

pub struct MapScene {
    facet: Facet,
    map_id: u8,
    radar_colors: Result<Vec<Color16>>,
    mode: MapRenderMode,
    texture: Option<Canvas>,
    exiting: bool,
    x: u32,
    y: u32,
}

pub fn draw_heightmap_block(
    bitmap: &mut Vec<u8>,
    block: &Block,
    _statics: &Vec<StaticLocation>,
    _radar_cols: &Result<Vec<Color16>>,
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
    bitmap: &mut Vec<u8>,
    block: &Block,
    _statics: &Vec<StaticLocation>,
    radar_cols: &Result<Vec<Color16>>,
) {
    for y in 0..8 {
        for x in 0..8 {
            let target = x + (y * 8);
            let index = block.cells[target].graphic;
            let (r, g, b, _) = match radar_cols {
                &Ok(ref colors) => colors[index as usize].to_rgba(),
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
    bitmap: &mut Vec<u8>,
    _block: &Block,
    statics: &Vec<StaticLocation>,
    radar_cols: &Result<Vec<Color16>>,
) {
    let mut last_height_locs = vec![-127; 64];
    for stat in statics {
        let lookup = (stat.x + (stat.y * 8)) as usize;
        if last_height_locs[lookup] < stat.altitude {
            // Paint the cell as we're higher.
            // Inefficient, but probably no worse than trying to keep track of which items are in which cell
            let (r, g, b, _) = match radar_cols {
                &Ok(ref colors) => colors[stat.color_idx() as usize].to_rgba(),
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
    bitmap: &mut Vec<u8>,
    block: &Block,
    statics: &Vec<StaticLocation>,
    radar_cols: &Result<Vec<Color16>>,
) {
    draw_radarcol_block(bitmap, block, statics, radar_cols);
    draw_statics_block(bitmap, block, statics, radar_cols);
}

impl<'a> MapScene {
    pub fn new(ctx: &mut Context) -> BoxedScene<'a, SceneName, ()> {
        let colors = RadarColReader::new(&Path::new("./assets/radarcol.mul"))
            .and_then(|mut reader| reader.read_colors());

        let mut scene = Box::new(MapScene {
            facet: map_id_to_facet(0),
            map_id: 0,
            texture: None,
            mode: MapRenderMode::HeightMap,
            radar_colors: colors,
            exiting: false,
            x: 0,
            y: 0,
        });

        scene.draw_page(ctx).expect("Failed to draw map");
        scene
    }

    pub fn draw_page(&mut self, ctx: &mut Context) -> GameResult<()> {
        let dest = Canvas::with_window_size(ctx)?;
        graphics::set_canvas(ctx, Some(&dest));
        graphics::clear(ctx, graphics::BLACK);

        let block_drawer = match self.mode {
            MapRenderMode::HeightMap => draw_heightmap_block,
            MapRenderMode::RadarMap => draw_radarcol_block,
            MapRenderMode::StaticsMap => draw_statics_block,
            MapRenderMode::FullMap => draw_full_block,
        };
        for y in 0..MAX_BLOCKS_HEIGHT {
            for x in 0..MAX_BLOCKS_WIDTH {
                match self.facet.read_block(x + self.x, y + self.y) {
                    ((ref block, ref statics), _) => {
                        let mut bitmap = vec![0; 8 * 8 * 4];
                        block_drawer(&mut bitmap, block, statics, &self.radar_colors);
                        let block_surface = Image::from_rgba8(ctx, 8, 8, &bitmap)
                            .expect("Failed to create surface");
                        graphics::draw(
                            ctx,
                            &block_surface,
                            DrawParam::default().dest(Point2::new(x as f32 * 8.0, y as f32 * 8.0)),
                        )?;
                    }
                }
            }
        }
        graphics::set_canvas(ctx, None);
        self.texture = Some(dest);
        Ok(())
    }
}

impl Scene<SceneName, ()> for MapScene {
    fn draw(&mut self, ctx: &mut Context, _engine_data: &mut ()) -> GameResult<()> {
        match self.texture {
            Some(ref texture) => {
                graphics::draw(ctx, texture, DrawParam::default())?;
            }
            None => (),
        };
        Ok(())
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
                    self.draw_page(ctx).expect("Failed to draw map");
                }
            }
            KeyCode::Right => {
                self.x += STEP_X as u32;
                self.draw_page(ctx).expect("Failed to draw map");
            }
            KeyCode::Up => {
                if self.y >= STEP_Y as u32 {
                    self.y -= STEP_Y as u32;
                    self.draw_page(ctx).expect("Failed to draw map");
                }
            }
            KeyCode::Down => {
                self.y += STEP_Y as u32;
                self.draw_page(ctx).expect("Failed to draw map");
            }
            KeyCode::Key1 => {
                self.mode = MapRenderMode::HeightMap;
                self.draw_page(ctx).expect("Failed to draw map");
            }
            KeyCode::Key2 => {
                self.mode = MapRenderMode::RadarMap;
                self.draw_page(ctx).expect("Failed to draw map");
            }
            KeyCode::Key3 => {
                self.mode = MapRenderMode::StaticsMap;
                self.draw_page(ctx).expect("Failed to draw map");
            }
            KeyCode::Key4 => {
                self.mode = MapRenderMode::FullMap;
                self.draw_page(ctx).expect("Failed to draw map");
            }
            KeyCode::Tab => {
                self.mode = MapRenderMode::HeightMap;
                self.map_id = (self.map_id + 1) % MAP_DETAILS.len() as u8;
                self.facet = map_id_to_facet(self.map_id);
                self.draw_page(ctx).expect("Failed to draw map");
            }
            _ => (),
        }
    }
}
