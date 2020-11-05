use image_convert::image_to_surface;
use scene::{SceneName, SceneChangeEvent, BoxedScene, Scene};
use cgmath::Point2;
use ggez::Context;
use ggez::graphics::{self, Canvas, Text, DrawParam, Image, Color};
use ggez::event::{KeyCode, KeyMods, MouseButton};
use ggez::conf::{NumSamples};
use std::io::Result;
use uorustlibs::hues::{HueReader, HueGroup, Hue};
use uorustlibs::color::Color as ColorTrait;
use std::fs::File;
use std::io::Error;
use std::path::Path;

static HEIGHT:f32 = 8.0;

pub struct HuesScene {
    reader: Result<HueReader<File>>,
    index: u32,
    texture: Option<Canvas>,
    exiting: bool,
}

impl<'a> HuesScene {
    pub fn new(ctx: &mut Context) -> BoxedScene<'a, SceneName, ()> {
        let mut scene = Box::new(HuesScene {
            reader: HueReader::new(&Path::new("./assets/hues.mul")),
            texture: None,
            index: 0,
            exiting: false,
        });
        scene.load_group(ctx);
        scene
    }

    fn load_group(&mut self, ctx: &mut Context) {
        let mut dest = Canvas::with_window_size(ctx).unwrap();
        let maybe_group = match self.reader {
            Ok(ref mut hue_reader) => {
                hue_reader.read_hue_group(self.index)
            },
            Err(ref x) => Err(Error::new(x.kind(), "Whoops"))
        };
        match maybe_group {
            Ok(group) => {
                let drawn_group = self.draw_hue_group(ctx, self.index, &group);
                graphics::set_canvas(ctx, Some(&dest));
                graphics::clear(ctx, graphics::BLACK);
                graphics::draw(ctx, &drawn_group, DrawParam::default()).expect("Failed to blit texture");
                graphics::set_canvas(ctx, None);
            },
            Err(_) => {
                graphics::set_canvas(ctx, Some(&dest));
                graphics::clear(ctx, graphics::BLACK);
                graphics::set_canvas(ctx, None);
            }
        };
        self.texture = Some(dest);
    }

    fn draw_hue_group(&self, ctx: &mut Context, group_idx: u32, group: &HueGroup) -> Image {
        let mut dest = Canvas::new(ctx, 256, HEIGHT as u16 * 9, NumSamples::One).unwrap();
        graphics::set_canvas(ctx, Some(&dest));
        graphics::clear(ctx, graphics::BLACK);
        for (idx, hue) in group.entries.iter().enumerate() {
            let drawn_hue = self.draw_hue(ctx, &hue);
            graphics::draw(ctx, &drawn_hue, DrawParam::default().dest(Point2::new(0.0, (idx * drawn_hue.height() as usize) as f32))).unwrap();
        }
        let label = Text::new(format!("Group {} - {}", group_idx, group.header));
        graphics::draw(ctx, &label, (Point2::new(0.0, HEIGHT * 8.0 + 4.0), graphics::WHITE));
        graphics::set_canvas(ctx, None);
        dest.into_inner()
    }

    fn draw_hue(&self, ctx: &mut Context, hue: &Hue) -> Image {
        let mut dest = Canvas::new(ctx, 256, HEIGHT as u16, NumSamples::One).unwrap();
        graphics::set_canvas(ctx, Some(&dest));
        graphics::clear(ctx, graphics::WHITE);
        /*for (col_idx, &color) in hue.color_table.iter().enumerate() {
            let (r, g, b, _) = color.to_rgba();
            let rect = graphics::Rect::new(col_idx as f32 * 16.0, 0.0, 16.0, HEIGHT);
            let r1 = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, Color::from_rgba(r, g, b, 255)).unwrap();
            graphics::draw(ctx, &r1, DrawParam::default()).unwrap();
        };*/
        let label_text = format!("{}: {} - {}", if hue.name.trim().len() > 0 { &hue.name } else { "NONE" }, hue.table_start, hue.table_end);
        let label = Text::new(format!("{}", label_text));
        graphics::draw(ctx, &label, (Point2::new(0.0, 48.0), graphics::WHITE));
        graphics::set_canvas(ctx, None);
        dest.into_inner()
    }
}

impl Scene<SceneName, ()> for HuesScene {
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
                    self.load_group(ctx);
                }
            },
            KeyCode::Right => {
                self.index += 1;
                self.load_group(ctx);
            },
            _ => ()
        }
    }
}
