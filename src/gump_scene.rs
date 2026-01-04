use crate::image_convert::image_to_surface;
use crate::scene::{BoxedScene, Scene, SceneChangeEvent, SceneName};
use ggez::glam::Vec2;
use ggez::graphics::{Canvas, Color, DrawParam, Image, ScreenImage, Text};
use ggez::input::keyboard::{KeyCode, KeyInput};
use ggez::{Context, GameResult};
use std::fs::File;
use std::io::Result;
use std::path::Path;
use uorustlibs::gump::{Gump, GumpReader};

pub struct GumpScene {
    reader: Result<GumpReader<File>>,
    index: u32,
    current_gump: Option<Gump>,
    texture: Option<Image>,
    exiting: bool,
}

impl<'a> GumpScene {
    pub fn new(ctx: &mut Context) -> BoxedScene<'a, SceneName, ()> {
        let mut reader = GumpReader::new(
            &Path::new("./assets/gumpidx.mul"),
            &Path::new("./assets/gumpart.mul"),
        );
        let current_gump = if let Ok(ref mut r) = reader {
            r.read_gump(0).ok()
        } else {
            None
        };
        let mut scene = Box::new(GumpScene {
            reader,
            index: 0,
            current_gump,
            texture: None,
            exiting: false,
        });
        scene
    }

    fn cycle_backward(&mut self) {
        if let Ok(ref mut reader) = self.reader {
            while self.index > 0 {
                self.index -= 1;
                if let Ok(g) = reader.read_gump(self.index) {
                    self.current_gump = Some(g);
                    self.texture = None;
                    break;
                }
            }
        }
    }

    fn cycle_forward(&mut self) {
        if let Ok(ref mut reader) = self.reader {
            loop {
                self.index += 1;
                if let Ok(g) = reader.read_gump(self.index) {
                    self.current_gump = Some(g);
                    self.texture = None;
                    break;
                }
            }
        }
    }

    fn create_slice(&mut self, ctx: &mut Context) -> GameResult<()> {
        let mut img = ScreenImage::new(ctx, None, 1.0, 1.0, 1);
        let mut dest = Canvas::from_screen_image(ctx, &mut img, Color::BLACK);
        match self.reader {
            Ok(ref mut reader) => match reader.read_gump(self.index) {
                Ok(gump) => {
                    let image = gump.to_image();
                    let surface = image_to_surface(ctx, &image);
                    dest.draw(&surface, DrawParam::default());
                    let label = Text::new(format!("{}", self.index));
                    dest.draw(
                        &label,
                        DrawParam::default()
                            .dest(Vec2::new(9.0, surface.height() as f32 + 16.0))
                            .color(Color::WHITE),
                    );
                }
                _ => {
                    let label = Text::new(format!("Invalid gump {}", self.index));
                    dest.draw(
                        &label,
                        DrawParam::default()
                            .dest(Vec2::new(9.0, 16.0))
                            .color(Color::WHITE),
                    );
                }
            },
            _ => {
                let text = Text::new("Could not create slice");
                dest.draw(
                    &text,
                    DrawParam::default()
                        .dest(Vec2::new(0.0, 0.0))
                        .color(Color::WHITE),
                );
            }
        }
        dest.finish(ctx)?;
        self.texture = Some(img.image(ctx));
        Ok(())
    }
}

impl Scene<SceneName, ()> for GumpScene {
    fn draw(&mut self, ctx: &mut Context, _engine_data: &mut ()) -> GameResult<()> {
        let mut canvas = Canvas::from_frame(ctx, Color::BLACK);
        if self.texture.is_none() {
            self.create_slice(ctx)?;
        }
        if let Some(ref texture) = self.texture {
            canvas.draw(texture, DrawParam::default());
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
                self.cycle_backward();
            }
            Some(KeyCode::Right) => {
                self.cycle_forward();
            }
            _ => (),
        }
    }
}
