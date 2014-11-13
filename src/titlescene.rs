use scene::{BoxedScene, Scene};

pub struct TitleScene {
}

impl TitleScene {
    pub fn new() -> BoxedScene {
        box TitleScene{
        }
    }
}

impl Scene for TitleScene {
    fn handle_event(&mut self, e: &Event) -> Option<BoxedScene> {
        None
    }
}
