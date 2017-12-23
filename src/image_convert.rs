use image::RgbaImage;
use sdl2::surface::Surface;
use sdl2::pixels::{PixelFormatEnum};

pub fn image_to_surface(image: &RgbaImage) -> Surface {
    let mut surface = Surface::new(image.width(), image.height(), PixelFormatEnum::RGBA8888).expect("Failed to create surface");
    surface.with_lock_mut(|data| {
        data.copy_from_slice(&image.to_vec());
    });
    surface
}
