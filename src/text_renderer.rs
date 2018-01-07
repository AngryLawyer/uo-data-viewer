use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::render::{Texture, TextureCreator};
use sdl2::rect::{Rect, Point};
use sdl2::surface::{Surface, SurfaceRef};
use sdl2::ttf::Font;

pub struct TextRenderer<'a> {
    font: &'a Font<'a, 'a>
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

    pub fn create_text_texture<'b, T>(&self, texture_creator: &'b TextureCreator<T>, text: &str, color: Color) -> Texture<'b> {
        let surface = self.create_text(text, color);
        texture_creator.create_texture_from_surface(&surface).unwrap()
    }

    pub fn draw_text(&self, target: &mut SurfaceRef, coords: &Point, content: &str) {
        let label = self.create_text(content, Color::RGBA(255, 255, 255, 255));

        label.blit(None, target, Some(Rect::new(coords.x(), coords.y(), label.width(), label.height()))).expect("Could not blit label to surface");
    }
}
