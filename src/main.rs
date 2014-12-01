#![feature(phase)]

#[phase(plugin, link)]
extern crate conrod;
extern crate event;
extern crate sdl2_window;
extern crate shader_version;
extern crate opengl_graphics;
extern crate input;
extern crate uorustlibs;
extern crate graphics;
extern crate image;

use event::{Events, WindowSettings};
use conrod::{UiContext, Theme};
use opengl_graphics::Gl;
use opengl_graphics::glyph_cache::GlyphCache;
use sdl2_window::Sdl2Window as Window;

mod scene;
mod title_scene;
mod skills_scene;
mod hues_scene;
mod tile_scene;
mod statics_scene;


fn main() {
    let opengl = shader_version::opengl::OpenGL_3_2;
    let window = Window::new(
        opengl,
        WindowSettings {
            title: "UO Data Viewer".to_string(),
            size: [1024, 768],
            fullscreen: false,
            exit_on_esc:true,
            samples: 4,
        }
    );
   
    let font_path = Path::new("./assets/Bretan.otf");
    let theme = Theme::default();
    let glyph_cache = GlyphCache::new(&font_path).unwrap();
    let mut context = UiContext::new(glyph_cache, theme);
    let mut gl = Gl::new(opengl);
    gl.enable_alpha_blend();

    let mut current_scene = title_scene::TitleScene::new();
    for ref e in Events::new(window) {
        context.handle_event(e);
        match current_scene.handle_event(e, &mut context, &mut gl) {
            Some(scene) => current_scene = scene,
            None => ()
        };
    }
}
