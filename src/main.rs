extern crate sdl2;
extern crate sdl2_ttf;
extern crate sdl2_engine_helpers;
extern crate uorustlibs;

mod scene;
mod title_scene;
mod skills_scene;
mod text_renderer;

use std::path::Path;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::Renderer;
use sdl2_engine_helpers::game_loop::GameLoop;

pub fn make_scene(scene: scene::SceneName, text_renderer: &text_renderer::TextRenderer, renderer: &mut Renderer) -> scene::BoxedScene<scene::SceneName> {
    match scene {
        scene::SceneName::TitleScene => {
            title_scene::TitleScene::new(text_renderer, renderer)
        },
        scene::SceneName::SkillsScene => {
            skills_scene::SkillsScene::new(text_renderer, renderer)
        },
    }
}

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
    let text_renderer = text_renderer::TextRenderer::new(font);

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut scene_stack = scene::SceneStack::<scene::SceneName>::new();
    let scene = title_scene::TitleScene::new(&text_renderer, &mut renderer);
    scene_stack.push(scene);

    let mut game_loop = GameLoop::new(30);
    game_loop.run(|frame| {
        let mut ended = false;
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..}  => {
                    ended = true
                },
                _ => {
                    let scene_event = scene_stack.handle_event(&event);
                    match scene_event {
                        Some(scene::SceneChangeEvent::PopScene) => {
                            scene_stack.pop();
                        },
                        Some(scene::SceneChangeEvent::PushScene(scene)) => {
                            scene_stack.push(make_scene(scene, &text_renderer, &mut renderer))
                        },
                        Some(scene::SceneChangeEvent::SwapScene(scene)) => {
                            scene_stack.pop();
                            scene_stack.push(make_scene(scene, &text_renderer, &mut renderer))
                        },
                        _ => ()
                    }
                }
            }
        }
        scene_stack.render(&mut renderer);
        ended || scene_stack.is_empty()
    });
}
