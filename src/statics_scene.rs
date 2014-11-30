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
};
use input::{Release, Keyboard, keyboard};
use uorustlibs::art::{ArtReader, Art};
use uorustlibs::color::Color as ColorTrait;
use image::{ImageBuf, Rgba};
use image::imageops::overlay;

use std::io::IoResult;

static MAX_X:u32 = 18;
static MAX_Y:u32 = 10;


pub struct StaticsScene {
    reader: IoResult<ArtReader>,
    index: u32,
    texture: Option<Texture>
}

impl StaticsScene {
    pub fn new() -> BoxedScene {
        let reader = ArtReader::new(&Path::new("./assets/artidx.mul"), &Path::new("./assets/art.mul"));
        let mut scene = box StaticsScene {
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
                let limit = MAX_X * MAX_Y;
                let start = limit * self.index;
                let mut dest = ImageBuf::new(1024, 768);
                let maybe_tile = reader.read_static(0);
                match maybe_tile {
                    Ok(tile) => println!("OK"),
                    Err(e) => println!("{}", e)
                }
                //println!("{}", maybe_tile);
                /*for x in range(0, MAX_X) {
                    for y in range(0, MAX_Y) {
                        let maybe_tile = reader.read_tile(start + x + (y * MAX_X));
                        match maybe_tile {
                            Ok(tile) => {
                                let (width, height, data) = tile.to_32bit();
                                let buf = ImageBuf::from_fn(width, height, |x, y| {
                                    let (r, g, b, a) = data[((x % height) + (y * width)) as uint].to_rgba();
                                    Rgba(r, g, b, a)
                                });
                                overlay(&mut dest, &buf, 44 * x, (44 + 16) * y)
                            },
                            _ => ()
                        }
                    }
                }*/

                self.texture = Some(Texture::from_image(&dest))
            },
            Err(_) => {
                self.texture = None
            }
        }
    }


    fn render(&self, args: RenderArgs, uic: &mut UiContext, gl: &mut Gl) {
        let limit = MAX_X * MAX_Y;
        let start = limit * self.index;
        uic.background().color(Color::black()).draw(gl);
        let c = Context::abs(args.width as f64, args.height as f64);
        match self.texture {
            Some(ref texture) => {
                c.image(texture).draw(gl);
                /*for x in range(0, MAX_X) {
                    for y in range(0, MAX_Y) {
                        let index = start + x + (y * MAX_X);
                        self.draw_label(uic, gl, format!("{}", index).as_slice(), (44 * x) as f64, (((44 + 16) * y) + 44) as f64)
                    }
                }*/
            },
            None => ()
        }
    }
}

impl Scene for StaticsScene {
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
