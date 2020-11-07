use ggez::graphics::Image;
use ggez::Context;
use image::{Frame, RgbaImage};

pub fn image_to_surface(ctx: &mut Context, image: &RgbaImage) -> Image {
    let mut copied = image.clone().into_raw();
    Image::from_rgba8(ctx, image.width() as u16, image.height() as u16, &copied)
        .expect("Failed to create surface")
}

pub fn frame_to_surface(ctx: &mut Context, frame: &Frame) -> Image {
    image_to_surface(ctx, frame.buffer())
}
