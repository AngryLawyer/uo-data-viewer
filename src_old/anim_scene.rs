use cgmath::Point2;
use ggez::event::{KeyCode, KeyMods};
use ggez::graphics::{self, Canvas, DrawParam, Text};
use ggez::{timer, Context, GameResult};
use image_convert::frame_to_surface;
use scene::{BoxedScene, Scene, SceneChangeEvent, SceneName};
use std::cmp;
use std::fs::File;
use std::io::Result;
use std::path::Path;

use uorustlibs::anim::AnimReader;

pub struct AnimScene {
    reader: Result<AnimReader<File>>,
    file_index: u8,
    index: u32,
    textures: Vec<Canvas>,
    exiting: bool,
    current_frame: usize,
}

impl<'a> AnimScene {
    pub fn new(ctx: &mut Context) -> BoxedScene<'a, SceneName, ()> {
        let reader = AnimReader::new(
            &Path::new("./assets/anim.idx"),
            &Path::new("./assets/Anim.mul"),
        ); //FIXME: Multiple anim muls!
        let mut scene = Box::new(AnimScene {
            reader: reader,
            index: 0,
            textures: vec![],
            exiting: false,
            current_frame: 0,
            file_index: 0,
        });
        scene
            .load_reader(
                ctx,
                &Path::new("./assets/anim.idx"),
                &Path::new("./assets/Anim.mul"),
            )
            .expect("Failed to create slice");

        scene
    }

    fn load_reader(&mut self, ctx: &mut Context, idx: &Path, mul: &Path) -> GameResult<()> {
        self.reader = AnimReader::new(idx, mul);
        self.create_slice(ctx)
    }

    fn set_file_index(&mut self, ctx: &mut Context, idx: u8) -> GameResult<()> {
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
        self.index = 0;
        self.load_reader(ctx, &idx, &mul)
    }

    fn create_slice(&mut self, ctx: &mut Context) -> GameResult<()> {
        let offset = 100.0;
        let anim = match self.reader {
            Ok(ref mut reader) => reader.read(self.index).or_else(|_| Err("Invalid anim")),
            _ => Err("No reader"),
        };
        match anim {
            Ok(anim) => {
                self.textures = anim
                    .to_frames()
                    .enumerate()
                    .map(|(idx, frame)| {
                        let dest = Canvas::with_window_size(ctx)?;
                        graphics::set_canvas(ctx, Some(&dest));
                        graphics::clear(ctx, graphics::BLACK);
                        let top = match frame {
                            Ok(f) => {
                                let surface = frame_to_surface(ctx, &f);
                                graphics::draw(
                                    ctx,
                                    &surface,
                                    DrawParam::default().dest(Point2::new(offset, 0.0)),
                                )?;
                                surface.height() as f32
                            }
                            _ => 0.0,
                        };

                        let label = Text::new(format!("{}", self.index));
                        graphics::draw(
                            ctx,
                            &label,
                            (Point2::new(offset, top + 16.0), graphics::WHITE),
                        )?;
                        let label = Text::new(format!("{} / {}", idx + 1, anim.frame_count));
                        graphics::draw(
                            ctx,
                            &label,
                            (Point2::new(offset, top + 32.0), graphics::WHITE),
                        )?;
                        Ok(dest)
                    })
                    .collect::<GameResult<Vec<_>>>()?;
            }
            Err(e) => {
                let dest = Canvas::with_window_size(ctx)?;
                graphics::set_canvas(ctx, Some(&dest));
                graphics::clear(ctx, graphics::BLACK);
                let label = Text::new(e);
                graphics::draw(ctx, &label, (Point2::new(0.0, 0.0), graphics::WHITE))?;
                let label = Text::new(format!("{}", self.index));
                graphics::draw(ctx, &label, (Point2::new(0.0, 16.0), graphics::WHITE))?;
                self.textures = vec![dest];
            }
        };
        graphics::set_canvas(ctx, None);
        self.current_frame = 0;
        Ok(())
    }

    fn cycle_backward(&mut self) {
        match self.reader {
            Ok(ref mut reader) => {
                while self.index > 0 {
                    self.index -= 1;
                    match reader.read(self.index) {
                        Ok(_) => {
                            break;
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }

    fn cycle_forward(&mut self) {
        match self.reader {
            Ok(ref mut reader) => loop {
                self.index += 1;
                match reader.read(self.index) {
                    Ok(_) => {
                        break;
                    }
                    _ => {}
                }
            },
            _ => {}
        }
    }
}

impl Scene<SceneName, ()> for AnimScene {
    fn draw(&mut self, ctx: &mut Context, _engine_data: &mut ()) -> GameResult<()> {
        if self.textures.len() > 0 {
            graphics::draw(
                ctx,
                &self.textures[self.current_frame],
                DrawParam::default(),
            )?;
        }
        Ok(())
    }

    fn update(
        &mut self,
        ctx: &mut Context,
        _engine_data: &mut (),
    ) -> GameResult<Option<SceneChangeEvent<SceneName>>> {
        const DESIRED_FPS: u32 = 15;

        while timer::check_update_time(ctx, DESIRED_FPS) {
            self.current_frame += 1;
            if self.current_frame == self.textures.len() {
                self.current_frame = 0;
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
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
        _engine_data: &mut (),
    ) {
        match keycode {
            KeyCode::Escape => self.exiting = true,
            KeyCode::Left => {
                if self.index > 0 {
                    self.cycle_backward();
                    self.current_frame = 0;
                    self.create_slice(ctx).expect("Failed to create slice");
                }
            }
            KeyCode::Right => {
                self.cycle_forward();
                self.current_frame = 0;
                self.create_slice(ctx).expect("Failed to create slice");
            }
            KeyCode::Tab => {
                let idx = self.file_index;
                self.set_file_index(ctx, (idx + 1) % 3)
                    .expect("Failed to create slice");
            }
            _ => (),
        }
    }
}
