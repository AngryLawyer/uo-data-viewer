use scene::{BoxedScene, Scene};
use event::{Event, RenderArgs};
use conrod::UiContext;
use opengl_graphics::{Gl, Texture};
use graphics::{Context, AddImage, Draw};
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
use uorustlibs::art::{ArtReader, TileOrStatic, Art};
use uorustlibs::color::{Color16, Color32};
use uorustlibs::color::Color as ColorTrait;
use image::{GenericImage, ImageBuf, Rgba,};
use image::imageops::overlay;

use std::io::IoResult;


pub struct TileScene {
    reader: IoResult<ArtReader>,
    index: u32,
    texture: Option<Texture>
}

impl TileScene {
    pub fn new() -> BoxedScene {
        let mut reader = ArtReader::new(&Path::new("./assets/artidx.mul"), &Path::new("./assets/art.mul"));
        let mut scene = box TileScene {
            reader: reader,
            index: 0,
            texture: None
        };
        scene.create_slice();

        scene
    }

    fn create_slice(&mut self) {
        match self.reader {
            Ok(ref mut reader) => {
                let max_x = 18;
                let max_y = 13;
                let limit = max_x * max_y;
                let start = limit * self.index;
                let mut dest = ImageBuf::new(800, 600);

                for x in range(0, max_x) {
                    for y in range(0, max_y) {
                        let maybe_tile = reader.read(start + x + (y * max_x));
                        match maybe_tile {
                            Ok(TileOrStatic::Tile(tile)) => {
                                let (width, height, data) = tile.to_32bit();
                                let buf = ImageBuf::from_fn(width, height, |x, y| {
                                    let (r, g, b, a) = data[((x % height) + (y * width)) as uint].to_rgba();
                                    Rgba(r, g, b, a)
                                });
                                overlay(&mut dest, &buf, 44 * x, 44 * y)
                            },
                            _ => ()
                        }
                    }
                }

                self.texture = Some(Texture::from_image(&dest))
            },
            Err(_) => {
                self.texture = None
            }
        }
    }


    fn render(&self, args: RenderArgs, uic: &mut UiContext, gl: &mut Gl) {
        uic.background().color(Color::black()).draw(gl);
        let c = Context::abs(args.width as f64, args.height as f64);
        match self.texture {
            Some(ref texture) => {
                c.image(texture).draw(gl);
            },
            None => ()
        }
    }
}

impl Scene for TileScene {
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
                            self.create_slice();
                        }
                    },
                    keyboard::Right => {
                        self.index += 1;
                        self.create_slice();
                    },
                    _ => ()
                }
            },
            _ => ()
        };
        None
    }
}
