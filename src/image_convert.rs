use ggez::Context;
use ggez::graphics::{Image, ImageFormat};
use image::{Frame, RgbaImage};

pub fn image_to_surface(ctx: &mut Context, image: &RgbaImage) -> Image {
    Image::from_pixels(
        ctx,
        image.as_raw(),
        ImageFormat::Rgba8Unorm,
        image.width(),
        image.height(),
    )
}

pub fn frame_to_surface(ctx: &mut Context, frame: &Frame) -> Image {
    image_to_surface(ctx, frame.buffer())
}
