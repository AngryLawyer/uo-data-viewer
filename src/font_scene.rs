use cgmath::Point2;
use ggez::event::{KeyCode, KeyMods};
use ggez::graphics::{self, Canvas, Color, DrawParam, Text};
use ggez::{Context, GameResult};
use image_convert::image_to_surface;
use scene::{BoxedScene, Scene, SceneChangeEvent, SceneName};
use std::fs::File;
use std::io::Error;
use std::io::Result;
use std::path::Path;
use uorustlibs::color::Color as ColorTrait;
use uorustlibs::fonts::{Font, FontReader};

pub struct FontScene {
    reader: Result<FontReader<File>>,
    fonts: Vec<Font>,
    index: usize,
    texture: Option<Canvas>,
    exiting: bool,
}

impl<'a> FontScene {
    pub fn new(ctx: &mut Context) -> BoxedScene<'a, SceneName, ()> {
        let mut scene = Box::new(FontScene {
            reader: FontReader::new(&Path::new("./assets/fonts.mul")),
            texture: None,
            index: 0,
            exiting: false,
            fonts: vec![],
        });
        scene.load_fonts(ctx).expect("Failed to create slice");
        scene.draw_font(ctx).expect("Failed to create image");
        scene
    }

    fn load_fonts(&mut self, ctx: &mut Context) -> GameResult<()> {
        match &mut self.reader {
            Ok(reader) => {
                self.fonts = reader.read_fonts()?;
            }
            _ => (),
        }
        Ok(())
    }

    fn draw_font(&mut self, ctx: &mut Context) -> GameResult<()> {
        let dest = Canvas::with_window_size(ctx)?;
        match self.fonts.get(self.index) {
            Some(font) => {
                graphics::set_canvas(ctx, Some(&dest));
                graphics::clear(ctx, graphics::BLACK);
                let mut x = 0.0;
                let mut y = 0.0;
                for i in 0..font.characters.len() {
                    let image = font.characters[i].to_image();
                    if image.width() != 0 && image.height() != 0 {
                        let surface = image_to_surface(ctx, &image);
                        graphics::draw(ctx, &surface, DrawParam::default().dest(Point2::new(x, y)))?;
                        x += image.width() as f32;
                        if i % 20 == 0 && i > 0 {
                            x = 0.0;
                            y += image.height() as f32;
                        }
                    }
                }
                graphics::set_canvas(ctx, None);
            }
            None => {
                graphics::set_canvas(ctx, Some(&dest));
                graphics::clear(ctx, graphics::BLACK);
                graphics::set_canvas(ctx, None);
            }
        }
        self.texture = Some(dest);
        Ok(())
    }
}

impl Scene<SceneName, ()> for FontScene {
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
                if self.index > 0 {
                    self.index -= 1;
                    self.draw_font(ctx);
                    //self.load_group(ctx).expect("Failed to create slice");
                }
            }
            KeyCode::Right => {
                self.index += 1;
                self.draw_font(ctx);
                //self.load_group(ctx).expect("Failed to create slice");
            }
            _ => (),
        }
    }
}
