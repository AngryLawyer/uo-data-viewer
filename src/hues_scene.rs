use scene::{BoxedScene, Scene};
use event::{Event, RenderArgs};
use conrod::UiContext;
use opengl_graphics::{Gl};
use graphics::{Context, AddColor, Draw, AddRectangle};
use conrod::{
    Background,
    Color,
    Colorable,
    Drawable,
    Label,
    Labelable,
    Positionable
};
use input::{Release, Keyboard, keyboard};
use uorustlibs::hues::{HueReader, HueGroup, Hue};
use uorustlibs::color::Color16;
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
        match self.reader {
            Ok(ref hue_reader) => {
                self.render_hue_group(args, uic, gl)
            },
            Err(ref error) => {
                self.draw_label(uic, gl, format!("{}", error).as_slice(), 0.0, 16.0);
            }
        };
    }

    fn render_hue_group(&self, args: RenderArgs, uic: &mut UiContext, gl: &mut Gl) {
        self.draw_label(uic, gl, format!("Hue group {}", self.index).as_slice(), 0.0, 0.0);

        match self.current_group {
            Some(Ok(ref hue_group)) => {
                self.draw_label(uic, gl, format!("Header {}", hue_group.header).as_slice(), 0.0, 16.0);
                for (idx, hue) in hue_group.entries.iter().enumerate() {
                    self.render_hue(args, uic, gl, idx as u32, hue);
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

    fn render_hue(&self, args: RenderArgs, uic: &mut UiContext, gl: &mut Gl, index: u32, hue: &Hue) {
        self.draw_label(uic, gl, hue.name.as_slice(), 0.0, (32 + (index * 16)) as f64);
        self.draw_label(uic, gl, format!("{} - {}", hue.table_start, hue.table_end).as_slice(), 256.0, (32 + (index * 16)) as f64);
        let c = Context::abs(args.width as f64, args.height as f64);

        for (col_idx, &color) in hue.color_table.iter().enumerate() {
            let (r, g, b, _) = color.to_rgba();
            c.rect((256.0 + 128.0) + (col_idx * 16) as f64, (32 + (index * 16)) as f64, 16.0, 16.0)
            .rgb(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0)
            .draw(gl);
        }
    }

}

impl Scene for HuesScene {
    fn handle_event(&mut self, e: &Event, ui_context: &mut UiContext, gl: &mut Gl) -> Option<BoxedScene> {
        match *e {
            Event::Render(args) => {
                self.render(args, ui_context, gl);
            },
            Event::Input(Release(Keyboard(key))) => {
                match key {
                    keyboard::Left => {
                        if self.index > 0 {
                            self.index -= 1;
                            self.load_group();
                        }
                    },
                    keyboard::Right => {
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
