use crate::image_convert::image_to_surface;
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
use uorustlibs::tiledata::{MapTileData, TileDataReader};

static MAX_X: u32 = 15;
static MAX_Y: u32 = 8;

pub struct TileScene {
    reader: Result<ArtReader<File>>,
    data: Result<TileDataReader<File>>,
    index: u32,
    tiles: Vec<(Option<Image>, Text, Vec2)>,
    tile_data: Vec<Result<MapTileData>>,
    exiting: bool,
}

impl<'a> TileScene {
    pub fn new(ctx: &mut Context) -> BoxedScene<'a, SceneName, ()> {
        let reader = ArtReader::new(
            &Path::new("./assets/artidx.mul"),
            &Path::new("./assets/art.mul"),
        );
        let data = TileDataReader::new(&Path::new("./assets/tiledata.mul"));
        let mut scene = Box::new(TileScene {
            reader,
            data,
            index: 0,
            tiles: vec![],
            tile_data: vec![],
            exiting: false,
        });
        scene.create_slice(ctx);
        scene
    }

    fn create_slice(&mut self, ctx: &mut Context) {
        self.tile_data = vec![];
        self.tiles = vec![];
        if let (&mut Ok(ref mut reader), &mut Ok(ref mut data)) = (&mut self.reader, &mut self.data)
        {
            let limit = MAX_X * MAX_Y;
            let start = limit * self.index;
            for y in 0..MAX_Y {
                for x in 0..MAX_X {
                    let maybe_tile = reader.read_tile(start + x + (y * MAX_X));
                    let index = start + x + (y * MAX_X);
                    let tile_img = maybe_tile.map(|tile| image_to_surface(ctx, &tile.to_image()));
                    let label = Text::new(format!("{}", index));
                    self.tile_data.push(data.read_map_tile_data(index));
                    self.tiles.push((
                        tile_img.ok(),
                        label,
                        Vec2::new(44.0 * x as f32, (44.0 + 16.0) * y as f32),
                    ));
                }
            }
        }
    }
}

impl Scene<SceneName, ()> for TileScene {
    fn draw(&mut self, ctx: &mut Context, _engine_data: &mut ()) -> GameResult<()> {
        let mut canvas = Canvas::from_frame(ctx, Color::BLACK);
        if self.tiles.is_empty() {
            let text = Text::new("Could not create slice");
            canvas.draw(&text, DrawParam::default().color(Color::WHITE));
        } else {
            for (maybe_image, text, pos) in &self.tiles {
                if let Some(image) = maybe_image {
                    canvas.draw(image, DrawParam::default().dest(*pos));
                }
                canvas.draw(
                    text,
                    DrawParam::default()
                        .color(Color::WHITE)
                        .dest(*pos + Vec2::new(0.0, 44.0)),
                );
            }
        }
        canvas.finish(ctx)
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
                if self.index > 0 {
                    self.index -= 1;
                    self.create_slice(ctx);
                }
            }
            Some(KeyCode::Right) => {
                self.index += 1;
                self.create_slice(ctx);
            }
            _ => (),
        }
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        x: f32,
        y: f32,
        _engine_data: &mut (),
    ) {
        let actual_x = (x / 44.0) as u32;
        let actual_y = (y / (44.0 + 16.0)) as u32;
        if actual_x < MAX_X && actual_y < MAX_Y {
            let actual_index = (actual_x + (actual_y * MAX_X)) as usize;
            if actual_index < self.tile_data.len()
                && let Ok(ref data) = self.tile_data[actual_index]
            {
                println!("{}", data.name);
            }
        }
    }
}
