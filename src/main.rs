#![feature(phase)]

#[phase(plugin, link)]
extern crate conrod;
extern crate event;
extern crate sdl2_window;
extern crate shader_version;

mod scene;
mod titlescene;

use event::{Events, WindowSettings};
use conrod::UiContext;
use sdl2_window::Sdl2Window as Window;

fn main() {
    let opengl = shader_version::opengl::OpenGL_3_2;
    let window = Window::new(
        opengl,
        WindowSettings {
            title: "SuperMegaTag".to_string(),
            size: [800, 600],
            fullscreen: false,
            exit_on_esc: true,
            samples: 4,
        }
    );
    let context = UiContext::new(&Path::new("./assets/Dense-Regular.otf"), None).ok().expect("Couldn't get a graphics context!");

    let mut current_scene = titlescene::TitleScene::new();
    for ref e in Events::new(window) {
        match current_scene.handle_event(e) {
            Some(scene) => current_scene = scene,
            None => ()
        };
    }
}
