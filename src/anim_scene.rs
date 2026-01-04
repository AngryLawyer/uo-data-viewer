use crate::image_convert::frame_to_surface;
use crate::scene::{BoxedScene, Scene, SceneChangeEvent, SceneName};
use ggez::glam::Vec2;
use ggez::graphics::{Canvas, Color, DrawParam, Image, ScreenImage, Text};
use ggez::input::keyboard::{KeyCode, KeyInput};
use ggez::{Context, GameResult};
use std::fs::File;
use std::io;
use std::path::Path;

use uorustlibs::anim::{AnimGroup, AnimReader};

pub struct AnimScene {
    reader: Option<AnimReader<File>>,
    file_index: u8,
    index: u32,
    textures: Option<Vec<Image>>,
    current_anim: Result<AnimGroup, io::Error>,
    exiting: bool,
    current_frame: usize,
}

impl<'a> AnimScene {
    pub fn new(ctx: &mut Context) -> BoxedScene<'a, SceneName, ()> {
        let mut scene = Box::new(AnimScene {
            reader: None,
            index: 0,
            textures: None,
            exiting: false,
            current_frame: 0,
            current_anim: Err(io::Error::other("Not loaded")),
            file_index: 0,
        });
        scene.set_file_index(0);
        scene.cycle_forward();

        scene
    }

    fn load_reader(&mut self, idx: &Path, mul: &Path) {
        self.reader = AnimReader::new(idx, mul).ok();
        self.index = 0;
        self.current_frame = 0;
        self.textures = None;
    }

    fn set_file_index(&mut self, idx: u8) {
        self.file_index = idx;
        let (idx, mul) = match idx {
            0 => (
                Path::new("./assets/anim.idx"),
                Path::new("./assets/Anim.mul"),
            ),
            1 => (
                Path::new("./assets/anim2.idx"),
                Path::new("./assets/anim2.mul"),
            ),
            2 => (
                Path::new("./assets/anim3.idx"),
                Path::new("./assets/anim3.mul"),
            ),
            _ => panic!("NO"),
        };
        self.load_reader(idx, mul);
    }

    fn create_slice(&mut self, ctx: &mut Context) -> GameResult<()> {
        let offset = 100.0;
        match self.current_anim {
            Ok(ref anim) => {
                self.textures = Some(
                    anim.to_frames()
                        .enumerate()
                        .map(|(idx, frame)| {
                            let mut img = ScreenImage::new(ctx, None, 1.0, 1.0, 1);
                            let mut canvas = Canvas::from_screen_image(ctx, &mut img, Color::BLACK);
                            let top = match frame {
                                Ok(f) => {
                                    let surface = frame_to_surface(ctx, &f);
                                    canvas.draw(
                                        &surface,
                                        DrawParam::default().dest(Vec2::new(offset, 0.0)),
                                    );
                                    surface.height() as f32
                                }
                                _ => 0.0,
                            };

                            let label = Text::new(format!("{}", self.index));
                            canvas.draw(
                                &label,
                                DrawParam::default()
                                    .dest(Vec2::new(offset, top + 16.0))
                                    .color(Color::WHITE),
                            );
                            let label = Text::new(format!("{} / {}", idx + 1, anim.frame_count));
                            canvas.draw(
                                &label,
                                DrawParam::default()
                                    .dest(Vec2::new(offset, top + 32.0))
                                    .color(Color::WHITE),
                            );
                            canvas.finish(ctx)?;
                            Ok(img.image(ctx))
                        })
                        .collect::<GameResult<Vec<_>>>()?,
                );
            }
            Err(ref e) => {
                let mut img = ScreenImage::new(ctx, None, 1.0, 1.0, 1);
                let mut dest = Canvas::from_screen_image(ctx, &mut img, Color::BLACK);
                let label = Text::new(e.to_string());
                dest.draw(&label, DrawParam::default().color(Color::WHITE));
                let label = Text::new(format!("{}", self.index));
                dest.draw(
                    &label,
                    DrawParam::default()
                        .dest(Vec2::new(0.0, 16.0))
                        .color(Color::WHITE),
                );
                dest.finish(ctx)?;
                self.textures = Some(vec![img.image(ctx)]);
            }
        };
        self.current_frame = 0;
        Ok(())
    }

    fn cycle_backward(&mut self) {
        if let Some(ref mut reader) = self.reader {
            while self.index > 0 {
                self.index -= 1;
                let maybe_anim = reader.read(self.index);
                if maybe_anim.is_ok() {
                    self.current_anim = maybe_anim;
                    self.current_frame = 0;
                    self.textures = None;
                    break;
                }
            }
        }
    }

    fn cycle_forward(&mut self) {
        if let Some(ref mut reader) = self.reader {
            loop {
                self.index += 1;
                let maybe_anim = reader.read(self.index);
                if maybe_anim.is_ok() {
                    self.current_anim = maybe_anim;
                    self.current_frame = 0;
                    self.textures = None;
                    break;
                }
            }
        }
    }
}

impl Scene<SceneName, ()> for AnimScene {
    fn draw(&mut self, ctx: &mut Context, _engine_data: &mut ()) -> GameResult<()> {
        let mut canvas = Canvas::from_frame(ctx, Color::BLACK);
        if let Some(ref textures) = self.textures {
            canvas.draw(&textures[self.current_frame], DrawParam::default());
        } else {
            self.create_slice(ctx)?;
        }
        canvas.finish(ctx)
    }

    fn update(
        &mut self,
        ctx: &mut Context,
        _engine_data: &mut (),
    ) -> GameResult<Option<SceneChangeEvent<SceneName>>> {
        const DESIRED_FPS: u32 = 15;

        if let Some(ref textures) = self.textures {
            while ctx.time.check_update_time(DESIRED_FPS) {
                self.current_frame += 1;
                if self.current_frame == textures.len() {
                    self.current_frame = 0;
                }
            }
        }

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
                    self.cycle_backward();
                }
            }
            Some(KeyCode::Right) => {
                self.cycle_forward();
            }
            Some(KeyCode::Tab) => {
                let idx = self.file_index;
                self.set_file_index((idx + 1) % 3);
                self.index = 0;
                self.cycle_forward();
            }
            _ => (),
        }
    }
}
