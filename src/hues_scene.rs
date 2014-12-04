use scene::{BoxedScene, Scene};
use event::{Event, RenderArgs};
use conrod::UiContext;
use opengl_graphics::{Gl};
use graphics::{Rectangle, rectangle, Context};
use conrod::{
    Background,
    Color,
    Colorable,
    Drawable,
};
use input::{InputEvent, Button};
use input::keyboard::Key;
use uorustlibs::hues::{HueReader, HueGroup, Hue};
use uorustlibs::color::Color as ColorTrait;

use std::io::IoResult;

pub struct HuesScene {
    reader: IoResult<HueReader>,
    index: u32,
    current_group: Option<IoResult<HueGroup>>
}

impl HuesScene {
    pub fn new() -> BoxedScene {
        let mut scene = box HuesScene {
            reader: HueReader::new(&Path::new("./assets/hues.mul")),
            current_group: None,
            index: 0
        };
        scene.load_group();
        scene
    }

    fn load_group(&mut self) {
        match self.reader {
            Ok(ref mut hue_reader) => {
                self.current_group = Some(hue_reader.read_hue_group(self.index))
            },
            Err(_) => ()
        }
    }

    fn render(&self, args: RenderArgs, uic: &mut UiContext, gl: &mut Gl) {
        uic.background().color(Color::black()).draw(gl);
        gl.draw([0, 0, args.width as i32, args.height as i32], |c, gl| {
            match self.reader {
                Ok(ref _hue_reader) => {
                    self.render_hue_group(args, uic, gl, &c)
                },
                Err(ref error) => {
                    self.draw_label(uic, gl, format!("{}", error).as_slice(), 0.0, 0.0);
                }
            };
        });
    }

    fn render_hue_group(&self, args: RenderArgs, uic: &mut UiContext, gl: &mut Gl, c: &Context) {
        self.draw_label(uic, gl, format!("Hue group {}", self.index).as_slice(), 0.0, 0.0);

        match self.current_group {
            Some(Ok(ref hue_group)) => {
                self.draw_label(uic, gl, format!("Header {}", hue_group.header).as_slice(), 0.0, 16.0);
                for (idx, hue) in hue_group.entries.iter().enumerate() {
                    self.render_hue(args, uic, gl, c, idx as u32, hue);
                }
            },
            Some(Err(ref error)) => {
                self.draw_label(uic, gl, format!("{}", error).as_slice(), 0.0, 16.0);
            },
            None => {
                self.draw_label(uic, gl, "No Hue Group Loaded", 0.0, 16.0);
            }
        }
    }

    fn render_hue(&self, _args: RenderArgs, uic: &mut UiContext, gl: &mut Gl, c: &Context, index: u32, hue: &Hue) {
        self.draw_label(uic, gl, hue.name.as_slice(), 0.0, (32 + (index * 16)) as f64);
        self.draw_label(uic, gl, format!("{} - {}", hue.table_start, hue.table_end).as_slice(), 256.0, (32 + (index * 16)) as f64);

        for (col_idx, &color) in hue.color_table.iter().enumerate() {
            let (r, g, b, _) = color.to_rgba();
            Rectangle::new([r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, 1.0]).draw(rectangle::square((256.0 + 128.0) + (col_idx * 16) as f64, (32 + (index * 16)) as f64, 16.0), c, gl);
        }
    }

}

impl Scene for HuesScene {
    fn handle_event(&mut self, e: &Event, ui_context: &mut UiContext, gl: &mut Gl) -> Option<BoxedScene> {
        match *e {
            Event::Render(args) => {
                self.render(args, ui_context, gl);
            },
            Event::Input(InputEvent::Release(Button::Keyboard(key))) => {
                match key {
                    Key::Left => {
                        if self.index > 0 {
                            self.index -= 1;
                            self.load_group();
                        }
                    },
                    Key::Right => {
                        self.index += 1;
                        self.load_group();
                    },
                    _ => ()
                }
            },
            _ => ()
        };
        None
    }
}
