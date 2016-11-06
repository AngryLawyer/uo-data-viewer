extern crate sdl2;
extern crate sdl2_ttf;
extern crate sdl2_engine_helpers;

mod scene;
mod title_scene;

use std::path::Path;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2_engine_helpers::game_loop::GameLoop;

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
    let font = ttf_subsystem.load_font(font_path, 24).unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let scene = Box::new(title_scene::TitleScene::new(&font, &mut renderer));

    let mut game_loop = GameLoop::new(30);
    game_loop.run(|frame| {
        let mut ended = false;
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    ended = true
                },
                _ => {}
            }
        }
        scene.render(&mut renderer);
        ended
    });
}
