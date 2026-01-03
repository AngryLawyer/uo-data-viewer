use ggez::graphics::Image;

pub enum LoadingTexture {
    Waiting,
    Loaded(Image),
    Failed,
}
