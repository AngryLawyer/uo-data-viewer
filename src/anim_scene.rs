use cgmath::Point2;
use ggez::event::{KeyCode, KeyMods, MouseButton};
use ggez::graphics::{self, Canvas, DrawParam, Text};
use ggez::timer;
use ggez::Context;
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
            index: 110,
            textures: vec![],
            exiting: false,
            current_frame: 0,
            file_index: 0,
        });
        scene.load_reader(
            ctx,
            &Path::new("./assets/anim.idx"),
            &Path::new("./assets/Anim.mul"),
        );

        scene
    }

    fn load_reader(&mut self, ctx: &mut Context, idx: &Path, mul: &Path) {
        self.reader = AnimReader::new(idx, mul);
        self.create_slice(ctx);
    }

    fn set_file_index(&mut self, ctx: &mut Context, idx: u8) {
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
        self.load_reader(ctx, &idx, &mul);
    }

    fn create_slice(&mut self, ctx: &mut Context) {
        let offset = 100;
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
                        let mut dest = Canvas::with_window_size(ctx).unwrap();
                        graphics::set_canvas(ctx, Some(&dest));
                        graphics::clear(ctx, graphics::BLACK);
                        let surface = frame_to_surface(ctx, &frame);
                        graphics::draw(ctx, &surface, DrawParam::default())
                            .expect("Failed to blit texture");

                        let label = Text::new(format!("{}", self.index));
                        graphics::draw(
                            ctx,
                            &label,
                            (
                                Point2::new(0.0, surface.height() as f32 + 16.0),
                                graphics::WHITE,
                            ),
                        );
                        let label = Text::new(format!("{} / {}", idx + 1, anim.frame_count));
                        graphics::draw(
                            ctx,
                            &label,
                            (
                                Point2::new(0.0, surface.height() as f32 + 32.0),
                                graphics::WHITE,
                            ),
                        );
                        dest
                    })
                    .collect::<_>();
            }
            Err(e) => {
                let mut dest = Canvas::with_window_size(ctx).unwrap();
                graphics::set_canvas(ctx, Some(&dest));
                graphics::clear(ctx, graphics::BLACK);
                let label = Text::new(e);
                graphics::draw(ctx, &label, (Point2::new(0.0, 0.0), graphics::WHITE));
                let label = Text::new(format!("{}", self.index));
                graphics::draw(ctx, &label, (Point2::new(0.0, 16.0), graphics::WHITE));
                self.textures = vec![dest];
            }
        };
        graphics::set_canvas(ctx, None);
        self.current_frame = 0;
    }
}

impl Scene<SceneName, ()> for AnimScene {
    fn draw(&mut self, ctx: &mut Context, engine_data: &mut ()) {
        if self.textures.len() > 0 {
            graphics::draw(
                ctx,
                &self.textures[self.current_frame],
                DrawParam::default(),
            )
            .unwrap();
        }
    }

    fn update(
        &mut self,
        ctx: &mut Context,
        engine_data: &mut (),
    ) -> Option<SceneChangeEvent<SceneName>> {
        const DESIRED_FPS: u32 = 15;

        while timer::check_update_time(ctx, DESIRED_FPS) {
            self.current_frame += 1;
            if (self.current_frame == self.textures.len()) {
                self.current_frame = 0;
            }
        }

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
                    self.current_frame = 0;
                    self.create_slice(ctx);
                }
            }
            KeyCode::Right => {
                self.index += 1;
                self.current_frame = 0;
                self.create_slice(ctx);
            }
            KeyCode::PageDown => {
                if self.index > 0 {
                    self.index = cmp::max(self.index - 10, 0);
                    self.current_frame = 0;
                    self.create_slice(ctx);
                }
            }
            KeyCode::PageUp => {
                self.index += 10;
                self.current_frame = 0;
                self.create_slice(ctx);
            }
            KeyCode::Tab => {
                let idx = self.file_index;
                self.set_file_index(ctx, (idx + 1) % 3);
            }
            _ => (),
        }
    }
}
