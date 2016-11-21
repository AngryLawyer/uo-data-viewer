use scene::{Scene, BoxedScene, SceneChangeEvent, SceneName};
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::render::{Renderer, Texture, TextureQuery};
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::surface::Surface;
use sdl2_ttf::Font;

pub struct TextRenderer<'a> {
    font: &'a Font<'a>
}

impl<'a> TextRenderer<'a> {
    pub fn new(font: &'a Font) -> TextRenderer<'a> {
        TextRenderer {
            font: font
        }
    }

    pub fn create_text(&self, text: &str, color: Color) -> Surface {
        let mut xy = (0, 0);
        for line in text.lines() {
            let (width, height) = self.font.size_of(line).unwrap();
            if width > xy.0 {
                xy.0 = width;
            }
            xy.1 += height;
        }
        let mut surface = Surface::new(xy.0, xy.1, PixelFormatEnum::RGBA8888).unwrap();
        let mut idx = 0;
        for line in text.lines() {
            let s = self.font.render(line).blended(color).unwrap();
            s.blit(None, &mut surface, Some(Rect::new(0, (idx * s.height()) as i32, s.width(), s.height()))).unwrap();
            idx += 1;
        }
        surface
    }

    pub fn create_text_texture(&self, renderer: &mut Renderer, text: &str, color: Color) -> Texture {
        let surface = self.create_text(text, color);
        renderer.create_texture_from_surface(&surface).unwrap()
    }
}
