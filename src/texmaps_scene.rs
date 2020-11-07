use cgmath::Point2;
use ggez::event::{KeyCode, KeyMods, MouseButton};
use ggez::graphics::{self, Canvas, DrawParam, Text};
use ggez::Context;
use image_convert::image_to_surface;
use scene::{BoxedScene, Scene, SceneChangeEvent, SceneName};
use std::fs::File;
use std::io::Result;
use std::path::Path;

use uorustlibs::texmaps::TexMapsReader;

static MAX_X: u32 = 8;
static MAX_Y: u32 = 5;

pub struct TexMapsScene {
    reader: Result<TexMapsReader<File>>,
    index: u32,
    texture: Option<Canvas>,
    exiting: bool,
}

impl<'a> TexMapsScene {
    pub fn new(ctx: &mut Context) -> BoxedScene<'a, SceneName, ()> {
        let reader = TexMapsReader::new(
            &Path::new("./assets/texidx.mul"),
            &Path::new("./assets/texmaps.mul"),
        );
        let mut scene = Box::new(TexMapsScene {
            reader: reader,
            index: 0,
            texture: None,
            exiting: false,
        });
        scene.create_slice(ctx);
        scene
    }

    fn create_slice(&mut self, ctx: &mut Context) {
        let mut dest = Canvas::with_window_size(ctx).unwrap();
        graphics::set_canvas(ctx, Some(&dest));
        graphics::clear(ctx, graphics::BLACK);
        match self.reader {
            Ok(ref mut reader) => {
                let limit = MAX_X * MAX_Y;
                let start = limit * self.index;
                for y in 0..MAX_Y {
                    for x in 0..MAX_X {
                        let maybe_tile = reader.read(start + x + (y * MAX_X));
                        let index = start + x + (y * MAX_X);
                        match maybe_tile {
                            Ok(tile) => {
                                let image = tile.to_image();
                                let surface = image_to_surface(ctx, &image);
                                graphics::draw(
                                    ctx,
                                    &surface,
                                    DrawParam::default().dest(Point2::new(
                                        128.0 * x as f32,
                                        (128.0 + 16.0) * y as f32,
                                    )),
                                )
                                .expect("Failed to blit texture");
                            }
                            _ => (),
                        };
                        let label = Text::new(format!("{}", index));
                        graphics::draw(
                            ctx,
                            &label,
                            (
                                Point2::new(128.0 * x as f32, ((128.0 + 16.0) * y as f32) + 128.0),
                                graphics::WHITE,
                            ),
                        );
                    }
                }
            }
            _ => {
                let text = Text::new("Could not create slice");
                graphics::draw(ctx, &text, (Point2::new(0.0, 0.0), graphics::WHITE));
            }
        }
        graphics::set_canvas(ctx, None);
        self.texture = Some(dest);
    }
}

impl Scene<SceneName, ()> for TexMapsScene {
    fn draw(&mut self, ctx: &mut Context, engine_data: &mut ()) {
        match self.texture {
            Some(ref texture) => {
                graphics::draw(ctx, texture, DrawParam::default()).unwrap();
            }
            None => (),
        };
    }

    fn update(
        &mut self,
        ctx: &mut Context,
        engine_data: &mut (),
    ) -> Option<SceneChangeEvent<SceneName>> {
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
        engine_data: &mut (),
    ) {
        match keycode {
            KeyCode::Escape => self.exiting = true,
            KeyCode::Left => {
                if self.index > 0 {
                    self.index -= 1;
                    self.create_slice(ctx);
                }
            }
            KeyCode::Right => {
                self.index += 1;
                self.create_slice(ctx);
            }
            _ => (),
        }
    }
}
