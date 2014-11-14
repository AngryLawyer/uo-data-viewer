#![feature(phase)]

#[phase(plugin, link)]
extern crate conrod;
extern crate event;
extern crate sdl2_window;
extern crate shader_version;
extern crate opengl_graphics;
extern crate input;
extern crate uorustlibs;

use event::{Events, WindowSettings};
use conrod::UiContext;
use opengl_graphics::Gl;
use sdl2_window::Sdl2Window as Window;

mod scene;
mod titlescene;
mod skillsscene;


fn main() {
    let opengl = shader_version::opengl::OpenGL_3_2;
    let window = Window::new(
        opengl,
        WindowSettings {
            title: "SuperMegaTag".to_string(),
            size: [1024, 768],
            fullscreen: false,
            exit_on_esc:true,
            samples: 4,
        }
    );
    let mut context = UiContext::new(&Path::new("./assets/Bretan.otf"), None).ok().expect("Couldn't get a graphics context!");
    let mut gl = Gl::new(opengl);

    let mut current_scene = titlescene::TitleScene::new();
    for ref e in Events::new(window) {
        context.handle_event(e);
        match current_scene.handle_event(e, &mut context, &mut gl) {
            Some(scene) => current_scene = scene,
            None => ()
        };
    }
}
