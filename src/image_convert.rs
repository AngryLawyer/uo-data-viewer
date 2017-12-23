use image::{RgbaImage, Rgba};
use sdl2::surface::Surface;
use sdl2::pixels::{PixelFormatEnum};

pub fn image_to_surface(image: &RgbaImage) -> Surface {
    let mut surface = Surface::new(image.width(), image.height(), PixelFormatEnum::RGBA8888).expect("Failed to create surface");
    let mut idx = 0;

    surface.with_lock_mut(|data| {
        let mut copied = image.clone().into_raw();
        // TODO: This appears to be endian-specific, and should be sorted
        for chunk in copied.chunks_mut(4) {
            chunk.reverse();
        }
        data.copy_from_slice(&copied);
    });
    surface
}
