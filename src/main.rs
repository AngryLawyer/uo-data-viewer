extern crate sdl2;
extern crate uorustlibs;
extern crate sdl2_engine_helpers;

mod engine;
mod text_renderer;
mod scene;

mod title_scene;
mod skills_scene;
/*mod tile_scene;
mod statics_scene;
mod hues_scene;
mod map_scene;
mod gump_scene;
mod anim_scene;
mod map;*/

use std::path::Path;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let ttf_subsystem = sdl2::ttf::init().unwrap();

    let window = video_subsystem.window("UO Data Viewer", 1024, 768)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().accelerated().build().unwrap();
    let texture_creator = canvas.texture_creator();

    let font_path = Path::new("./assets/Bretan.otf");
    let font = ttf_subsystem.load_font(font_path, 16).unwrap();
    let text_renderer = text_renderer::TextRenderer::new(&font);
    let ref texture_creator = &canvas.texture_creator();

    let event_pump = sdl_context.event_pump().unwrap();
    let mut engine_data = engine::EngineData::new(text_renderer);
    let mut engine = engine::Engine::new(30, event_pump, title_scene::TitleScene::new(&mut canvas, &mut engine_data, texture_creator));

    engine.run(|scene, renderer, engine_data| {
        match scene {
            scene::SceneName::TitleScene => {
                title_scene::TitleScene::new(renderer, engine_data, texture_creator)
            },
            scene::SceneName::SkillsScene => {
                skills_scene::SkillsScene::new(renderer, engine_data, texture_creator)
            },/*
            scene::SceneName::TileScene => {
                tile_scene::TileScene::new(renderer, engine_data)
            },
            scene::SceneName::StaticsScene => {
                statics_scene::StaticsScene::new(renderer, engine_data)
            },
            scene::SceneName::HuesScene => {
                hues_scene::HuesScene::new(renderer, engine_data)
            },
            scene::SceneName::MapScene => {
                map_scene::MapScene::new(renderer, engine_data)
            },
            scene::SceneName::GumpScene => {
                gump_scene::GumpScene::new(renderer, engine_data)
            },
            scene::SceneName::AnimScene => {
                anim_scene::AnimScene::new(renderer, engine_data)
            },*/
        }
    }, &mut canvas, &mut engine_data);
}
