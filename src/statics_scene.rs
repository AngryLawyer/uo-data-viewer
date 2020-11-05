use std::path::Path;
use uorustlibs::skills::Skills;

use image_convert::image_to_surface;
use scene::{SceneName, SceneChangeEvent, BoxedScene, Scene};
use cgmath::Point2;
use ggez::Context;
use ggez::graphics::{self, Canvas, Text, DrawParam};
use ggez::event::{KeyCode, KeyMods, MouseButton};
use std::io::Result;
use uorustlibs::art::{ArtReader, Art};
use uorustlibs::tiledata::{TileDataReader, StaticTileData};
use std::fs::File;

static MAX_X:u32 = 6;
static MAX_Y:u32 = 3;


pub struct StaticsScene {
    reader: Result<ArtReader<File>>,
    data: Result<TileDataReader>,
    index: u32,
    texture: Option<Canvas>,
    tile_data: Vec<Result<StaticTileData>>,
    exiting: bool
}

impl<'a> StaticsScene {
    pub fn new(ctx: &mut Context) -> BoxedScene<'a, SceneName, ()> {
        let reader = ArtReader::new(&Path::new("./assets/artidx.mul"), &Path::new("./assets/art.mul"));
        let data = TileDataReader::new(&Path::new("./assets/tiledata.mul"));
        let mut scene = Box::new(StaticsScene {
            reader,
            data,
            index: 0,
            texture: None,
            tile_data: vec![],
            exiting: false
        });
        scene.create_slice(ctx);

        scene
    }

    fn create_slice(&mut self, ctx: &mut Context) {
        self.tile_data = vec![];
        let mut dest = Canvas::with_window_size(ctx).unwrap();
        graphics::set_canvas(ctx, Some(&dest));
        graphics::clear(ctx, graphics::BLACK);
        match (&mut self.reader, &mut self.data) {
            (&mut Ok(ref mut reader), &mut Ok(ref mut data)) => {
                let limit = MAX_X * MAX_Y;
                let start = limit * self.index;

                for y in 0..MAX_Y {
                    for x in 0..MAX_X {
                        let index = start + x + (y * MAX_X);
                        let maybe_static = reader.read_static(index);
                        match maybe_static {
                            Ok(stat) => {
                                let image = stat.to_image();
                                let surface = image_to_surface(ctx, &image);
                                graphics::draw(ctx, &surface, DrawParam::default().dest(Point2::new(128.0 * x as f32, (128.0 + 16.0) * y as f32))).expect("Failed to blit texture");
                            },
                            _ => ()
                        }

                        let label = Text::new(format!("{}", index));
                        graphics::draw(ctx, &label, (Point2::new(128.0 * x as f32, ((128.0 + 16.0) * y as f32) + 128.0), graphics::WHITE));
                        self.tile_data.push(data.read_static_tile_data(index));
                    }
                }
            },
            _ => {
                let text = Text::new("Could not create slice");
                graphics::draw(ctx, &text, (Point2::new(0.0, 0.0), graphics::WHITE));
            }
        }
        graphics::set_canvas(ctx, None);
        self.texture = Some(dest);
    }
}

impl Scene<SceneName, ()> for StaticsScene {
    fn draw(&mut self, ctx: &mut Context, engine_data: &mut ()) {
        match self.texture {
            Some(ref texture) => {
                graphics::draw(ctx, texture, DrawParam::default()).unwrap();
            },
            None => ()
        };
    }

    fn update(&mut self, ctx: &mut Context, engine_data: &mut ()) -> Option<SceneChangeEvent<SceneName>> {
        if self.exiting {
            Some(SceneChangeEvent::PopScene)
        } else {
            None
        }
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        keymods: KeyMods,
        repeat: bool,
        engine_data: &mut ()
    ) {
        match keycode {
            KeyCode::Escape => {
                self.exiting = true
            },
            KeyCode::Left => {
                if self.index > 0 {
                    self.index -= 1;
                    self.create_slice(ctx);
                }
            },
            KeyCode::Right => {
                self.index += 1;
                self.create_slice(ctx);
            },
            _ => ()
        }
    }

    fn mouse_button_down_event(
        &mut self,
        ctx: &mut Context,
        button: MouseButton,
        x: f32,
        y: f32,
        engine_data: &mut () 
    ) {
        let actual_x = (x / 128.0) as u32;
        let actual_y = (y / (128.0 + 16.0)) as u32;
        if actual_x < MAX_X  && actual_y < MAX_Y {
            let actual_index = (actual_x + (actual_y * MAX_X)) as usize;
            if actual_index < self.tile_data.len() {
                match self.tile_data[actual_index] {
                    Ok(ref data) => {
                        println!("{}", data.name);
                    },
                    _ => ()
                }
            }
        }
    }
}
