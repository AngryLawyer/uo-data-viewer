use std::path::Path;
use image_convert::image_to_surface;
use scene::{SceneName, SceneChangeEvent, BoxedScene, Scene};
use cgmath::Point2;
use ggez::Context;
use ggez::graphics::{self, Canvas, Text, DrawParam};
use ggez::event::{KeyCode, KeyMods, MouseButton};
use std::io::Result;
use uorustlibs::art::{ArtReader, Art};
use std::fs::File;
use uorustlibs::gump::GumpReader;

pub struct GumpScene {
    reader: Result<GumpReader<File>>,
    index: u32,
    texture: Option<Canvas>,
    exiting: bool,
}

impl<'a> GumpScene {
    pub fn new(ctx: &mut Context) -> BoxedScene<'a, SceneName, ()> {
        let reader = GumpReader::new(&Path::new("./assets/gumpidx.mul"), &Path::new("./assets/gumpart.mul"));
        let mut scene = Box::new(GumpScene {
            reader: reader,
            index: 0,
            texture: None,
            exiting: false
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
                match reader.read_gump(self.index) {
                    Ok(gump) => {
                        let image = gump.to_image();
                        let surface = image_to_surface(ctx, &image);
                        graphics::draw(ctx, &surface, DrawParam::default()).expect("Failed to blit texture");
                        let label = Text::new(format!("{}", self.index));
                        graphics::draw(ctx, &label, (Point2::new(9.0, (surface.height() as f32 + 16.0)), graphics::WHITE));
                    },
                    _ => {
                        let label = Text::new(format!("Invalid gump {}", self.index));
                        graphics::draw(ctx, &label, (Point2::new(9.0, 16.0), graphics::WHITE));
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

impl Scene<SceneName, ()> for GumpScene {
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
}
