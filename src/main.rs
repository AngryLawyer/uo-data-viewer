extern crate sdl2;
extern crate sdl2_ttf;
extern crate sdl2_engine_helpers;
extern crate uorustlibs;

mod engine;
mod scene;
mod title_scene;
mod skills_scene;
mod tile_scene;
mod text_renderer;

use std::path::Path;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let ttf_subsystem = sdl2_ttf::init().unwrap();

    let window = video_subsystem.window("UO Data Viewer", 1024, 768)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut renderer = window.renderer().build().unwrap();
    let font_path = Path::new("./assets/Bretan.otf");
    let font = ttf_subsystem.load_font(font_path, 16).unwrap();
    let text_renderer = text_renderer::TextRenderer::new(&font);

    let event_pump = sdl_context.event_pump().unwrap();
    let mut engine = engine::Engine::new(30, event_pump, title_scene::TitleScene::new(&text_renderer, &mut renderer));

    engine.run(|scene, renderer| {
        match scene {
            scene::SceneName::TitleScene => {
                title_scene::TitleScene::new(&text_renderer, renderer)
            },
            scene::SceneName::SkillsScene => {
                skills_scene::SkillsScene::new(&text_renderer, renderer)
            },
            scene::SceneName::TileScene => {
                tile_scene::TileScene::new()
            },
        }
    }, &mut renderer);
}
