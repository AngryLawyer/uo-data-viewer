extern crate uorustlibs;
extern crate ggez;
extern crate cgmath;
//extern crate image;
mod engine;
mod scene;
mod title_scene;
/*
mod text_renderer;
mod image_convert;

mod title_scene;
mod skills_scene;
mod tile_scene;
mod statics_scene;
mod hues_scene;
mod map_scene;
mod gump_scene;
mod anim_scene;
mod texmaps_scene;
mod world_scene;

mod map;
mod caches;

use std::path::Path;*/

/*pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let ttf_subsystem = sdl2::ttf::init().unwrap();

    let window = video_subsystem.window("UO Data Viewer", 1024, 768)
Engine        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().accelerated().build().unwrap();

    let font_path = Path::new("./assets/Bretan.otf");
    let font = ttf_subsystem.load_font(font_path, 16).unwrap();
    let text_renderer = text_renderer::TextRenderer::new(&font);
    let ref texture_creator = &canvas.texture_creator();

    let event_pump = sdl_context.event_pump().unwrap();
    let mut engine_data = engine::EngineData::new(text_renderer);
    let mut engine = engine::Engine::new(30, event_pump, title_scene::TitleScene::new(&mut engine_data, texture_creator));

    engine.run(|scene, engine_data| {
        match scene {
            scene::SceneName::TitleScene => {
                title_scene::TitleScene::new(engine_data, texture_creator)
            },
            scene::SceneName::SkillsScene => {
                skills_scene::SkillsScene::new(engine_data, texture_creator)
            },
            scene::SceneName::TileScene => {
                tile_scene::TileScene::new(engine_data, texture_creator)
            },
            scene::SceneName::StaticsScene => {
                statics_scene::StaticsScene::new(engine_data, texture_creator)
            },
            scene::SceneName::HuesScene => {
                hues_scene::HuesScene::new(engine_data, texture_creator)
            },
            scene::SceneName::MapScene => {
                map_scene::MapScene::new(engine_data, texture_creator)
            },
            scene::SceneName::GumpScene => {
                gump_scene::GumpScene::new(engine_data, texture_creator)
            },
            scene::SceneName::AnimScene => {
                anim_scene::AnimScene::new(engine_data, texture_creator)
            },
            scene::SceneName::TexMapsScene => {
                texmaps_scene::TexMapsScene::new(engine_data, texture_creator)
            },
            scene::SceneName::WorldScene => {
                world_scene::WorldScene::new(engine_data, texture_creator)
            },
        }
    }, &mut canvas, &mut engine_data);
}*/
use ggez::{graphics, Context, ContextBuilder, GameResult};
use ggez::event::{self, EventHandler};

fn main() {
    // Make a Context.
    let (mut ctx, mut event_loop) = ContextBuilder::new("UO Data Viewer", "Angry Lawyer")
		.build()
		.expect("Could not create context");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let mut my_game = engine::Engine::new(&mut ctx);

    // Run!
    match event::run(&mut ctx, &mut event_loop, &mut my_game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e)
    }
}
