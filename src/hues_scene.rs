use scene::{BoxedScene, Scene, SceneChangeEvent, SceneName};
use engine::EngineData;
use text_renderer::TextRenderer;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::surface::Surface;
use std::io::Result;
use std::path::Path;
use uorustlibs::art::{ArtReader, Art};
use uorustlibs::hues::{HueReader, HueGroup, Hue};
use uorustlibs::color::Color as ColorTrait;

use sdl2::render::{Renderer, Texture, TextureQuery};
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub struct HuesScene {
    reader: Result<HueReader>,
    index: u32,
    current_group: Option<Result<HueGroup>>
}

impl HuesScene {
    pub fn new<'a>(renderer: &mut Renderer, engine_data: &mut EngineData<'a>) -> BoxedScene<SceneName, EngineData<'a>> {
        let mut scene = Box::new(HuesScene {
            reader: HueReader::new(&Path::new("./assets/hues.mul")),
            current_group: None,
            index: 0
        });
        //scene.load_group();
        scene
    }

    /*fn load_group(&mut self) {
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
    }*/

}

impl<'a> Scene<SceneName, EngineData<'a>> for HuesScene {
    fn render(&self, renderer: &mut Renderer, engine_data: &mut EngineData) {
        renderer.clear();
        /*match self.texture {
            Some(ref texture) => {
                renderer.copy(texture, None, None).unwrap();
            },
            None => ()
        };*/
        renderer.present();
    }

    fn handle_event(&mut self, event: &Event, renderer: &mut Renderer, engine_data: &mut EngineData) -> Option<SceneChangeEvent<SceneName>> {
        match *event {
            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                Some(SceneChangeEvent::PopScene)
            },
            Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                if self.index > 0 {
                    self.index -= 1;
                    //self.create_slice(renderer, engine_data);
                }
                None
            },
            Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                self.index += 1;
                //self.create_slice(renderer, engine_data);
                None
            },
             _ => None
        }
    }
}
