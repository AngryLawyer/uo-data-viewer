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
use uorustlibs::color::Color16;
use uorustlibs::color::Color as ColorTrait;
use image::{GenericImage, ImageBuf, Rgba};

use std::io::IoResult;

pub struct TileScene {
    reader: IoResult<ArtReader>,
    image: ImageBuf<Rgba<u8>>,
    texture: Texture
}

impl TileScene {
    pub fn new() -> BoxedScene {
        let mut reader = ArtReader::new(&Path::new("./assets/artidx.mul"), &Path::new("./assets/art.mul"));
        let mut image = match reader {
            Ok(ref mut reader) => {
                let tile = match reader.read(0).ok().expect("Couldn't load tile 0") {
                    TileOrStatic::Tile(tile) => tile,
                    TileOrStatic::Static(tile) => panic!("LOL")
                };
                let (width, height, data) = tile.to_32bit();
                ImageBuf::from_pixels(data, width, height)
            }
            _ => ImageBuf::new(44, 44)
        };
        let mut texture = Texture::from_image(&image);

        let mut scene = box TileScene {
            reader: reader,
            image: image,
            texture: texture
        };

        scene
    }

    fn render(&self, args: RenderArgs, uic: &mut UiContext, gl: &mut Gl) {
        uic.background().color(Color::black()).draw(gl);
        let c = Context::abs(args.width as f64, args.height as f64);
        c.image(&self.texture).draw(gl);   
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
                    },
                    keyboard::Right => {
                    },
                    _ => ()
                }
            },
            _ => ()
        };
        None
    }
}
