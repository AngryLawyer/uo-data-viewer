use scene::{BoxedScene, Scene, SceneChangeEvent, SceneName};
use engine::EngineData;
use text_renderer::TextRenderer;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::surface::Surface;
use std::io::Result;
use std::path::Path;
use uorustlibs::art::{ArtReader, Art};
use uorustlibs::color::Color as UOColor;

use sdl2::render::{Renderer, Texture, TextureQuery};
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

static MAX_X:u32 = 20;
static MAX_Y:u32 = 10;

pub struct TileScene {
    reader: Result<ArtReader>,
    index: u32,
    texture: Option<Texture>,
}

impl TileScene {
    pub fn new<'a>(renderer: &mut Renderer, engine_data: &mut EngineData<'a>) -> BoxedScene<SceneName, EngineData<'a>> {
        let reader = ArtReader::new(&Path::new("./assets/artidx.mul"), &Path::new("./assets/art.mul"));
        let mut scene = Box::new(TileScene {
            reader: reader,
            index: 0,
            texture: None,
        });
        scene.create_slice(renderer, engine_data);

        scene
    }

    fn create_slice(&mut self, renderer: &mut Renderer, engine_data: &mut EngineData) {
        match self.reader {
            Ok(ref mut reader) => {
                let limit = MAX_X * MAX_Y;
                let start = limit * self.index;
                let mut dest = Surface::new(1024, 768, PixelFormatEnum::RGBA8888).unwrap();
                dest.fill_rect(None, Color::RGB(0, 0, 0)).unwrap();

                for x in 0..MAX_X {
                    for y in 0..MAX_Y {
                        let maybe_tile = reader.read_tile(start + x + (y * MAX_X));
                        match maybe_tile {
                            Ok(tile) => {
                                /*let (width, height, data) = tile.to_32bit();
                                let mut surface = Surface::new(width, height, PixelFormatEnum::RGBA8888).unwrap();
                                surface.with_lock_mut(|bitmap| {
                                    for (i, cell) in data.iter().enumerate() {
                                        let (r, g, b, a) = cell.to_rgba();
                                        let offset = i * 4;
                                        bitmap[offset] = a;
                                        bitmap[offset + 1] = b;
                                        bitmap[offset + 2] = g;
                                        bitmap[offset + 3] = r;
                                    }
                                });*/
                                let surface = tile.to_surface();
                                surface.blit(None, &mut dest, Some(Rect::new(44 * x as i32, (44 + 16) * y as i32, 44, 44)));
                            },
                            _ => ()
                        }
                    }
                }

                self.texture = Some(renderer.create_texture_from_surface(&dest).unwrap());
            },
            Err(_) => {
                self.texture = None
            }
        }
    }


    /*fn render(&self, args: RenderArgs, uic: &mut UiContext, gl: &mut Gl) {
        gl.draw([0, 0, args.width as i32, args.height as i32], |c, gl| {
            let limit = MAX_X * MAX_Y;
            let start = limit * self.index;
            uic.background().color(Color::black()).draw(gl);
            match self.texture {
                Some(ref texture) => {
                    image(texture, &c, gl);
                    for x in range(0, MAX_X) {
                        for y in range(0, MAX_Y) {
                            let index = start + x + (y * MAX_X);
                            self.draw_label(uic, gl, format!("{}", index).as_slice(), (44 * x) as f64, (((44 + 16) * y) + 44) as f64)
                        }
                    }
                },
                None => ()
            }
        });
    }*/
}

impl<'a> Scene<SceneName, EngineData<'a>> for TileScene {
    fn render(&self, renderer: &mut Renderer, engine_data: &mut EngineData) {
        /*let TextureQuery {width, height, .. } = self.text.query();
        let target = Rect::new(0, 0, width, height);
        */
        renderer.clear();
        match self.texture {
            Some(ref texture) => {
                renderer.copy(texture, None, None).unwrap();
            },
            None => ()
        };
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
                    self.create_slice(renderer, engine_data);
                }
                None
            },
            Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                self.index += 1;
                self.create_slice(renderer, engine_data);
                None
            },
             _ => None
        }
    }
    /*fn handle_event(&mut self, e: &Event, ui_context: &mut UiContext, gl: &mut Gl) -> Option<BoxedScene> {
        match *e {
            Event::Render(args) => {
                self.render(args, ui_context, gl);
            },
            Event::Input(InputEvent::Release(Button::Keyboard(key))) => {
                match key {
                    Key::Left => {
                        if self.index > 0 {
                            self.index -= 1;
                            self.create_slice();
                        }
                    },
                    Key::Right => {
                        self.index += 1;
                        self.create_slice();
                    },
                    _ => ()
                }
            },
            _ => ()
        };
        None
    }*/
}
