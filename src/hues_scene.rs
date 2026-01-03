use ggez::input::keyboard::{KeyCode, KeyInput};
use ggez::graphics::{Canvas, Color, DrawParam, Text, Image, Rect, ScreenImage, Quad};
use ggez::{Context, GameResult};
use ggez::glam::Vec2;
use crate::scene::{BoxedScene, Scene, SceneChangeEvent, SceneName};
use crate::loading_texture::LoadingTexture;
use std::fs::File;
use std::io::Result;
use std::path::Path;
use uorustlibs::color::Color as ColorTrait;
use uorustlibs::hues::{Hue, HueGroup, HueReader};

static HEIGHT: f32 = 16.0;

pub struct HuesScene {
    reader: Result<HueReader<File>>,
    index: u32,
    texture: LoadingTexture,
    exiting: bool,
}

impl<'a> HuesScene {
    pub fn new(ctx: &mut Context) -> BoxedScene<'a, SceneName, ()> {
        let scene = Box::new(HuesScene {
            reader: HueReader::new(&Path::new("./assets/hues.mul")),
            texture: LoadingTexture::Waiting,
            index: 0,
            exiting: false,
        });
        scene
    }

    fn load_group(&mut self, ctx: &mut Context) {
        let maybe_group = self.reader.as_mut().ok().and_then(|hue_reader| {
            hue_reader.read_hue_group(self.index).ok()
        });
        match maybe_group {
            Some(group) => {
                self.texture = LoadingTexture::Loaded(self.draw_hue_group(ctx, self.index, &group));
            }
            None => {
                self.texture = LoadingTexture::Failed;
            }
        };
    }

    fn draw_hue_group(
        &self,
        ctx: &mut Context,
        group_idx: u32,
        group: &HueGroup,
    ) -> Image {
        let mut img = ScreenImage::new(ctx, None, 1.0, 1.0, 1);
        let mut canvas = Canvas::from_screen_image(ctx, &mut img, Color::BLACK);
        for (idx, hue) in group.entries.iter().enumerate() {
            self.draw_hue(&mut canvas, hue, idx as u32);
        }
        let label = Text::new(format!("Group {} - {}", group_idx, group.header));
        canvas.draw(&label, DrawParam::new().dest(Vec2::new(0.0, HEIGHT * 8.0 + 4.0)).color(Color::WHITE));
        canvas.finish(ctx).unwrap();
        img.image(ctx)
    }

    fn draw_hue(&self, canvas: &mut Canvas, hue: &Hue, hue_idx: u32) {
        for (col_idx, &color) in hue.color_table.iter().enumerate() {
            let (r, g, b, _) = color.to_rgba();
            let rect = Rect::new(col_idx as f32 * 16.0, hue_idx as f32 * HEIGHT, 16.0, HEIGHT);
            canvas.draw(&Quad, DrawParam::new().dest(rect.point()).scale(rect.size()).color(Color::from_rgba(r, g, b, 255)));
        }
        let label_text = format!(
            "{}: {} - {}",
            if !hue.name.trim().is_empty() {
                &hue.name
            } else {
                "NONE"
            },
            hue.table_start,
            hue.table_end
        );
        let label = Text::new(label_text);
        canvas.draw(
            &label,
            DrawParam::new().dest(
                Vec2::new(hue.color_table.len() as f32 * 16.0, hue_idx as f32 * HEIGHT),
            ).color(
                Color::WHITE,
            ),
        );
    }
}

impl Scene<SceneName, ()> for HuesScene {
    fn draw(&mut self, ctx: &mut Context, _engine_data: &mut ()) -> GameResult<()> {
        let mut canvas = Canvas::from_frame(ctx, Color::BLACK);
        match self.texture {
            LoadingTexture::Waiting => {
                self.load_group(ctx);
            },
            LoadingTexture::Loaded(ref texture) => {
                canvas.draw(texture, DrawParam::default());
            },
            LoadingTexture::Failed => (),
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
                    self.texture = LoadingTexture::Waiting;
                }
            }
            Some(KeyCode::Right) => {
                self.index += 1;
                self.texture = LoadingTexture::Waiting;
            }
            _ => (),
        }
    }
}
