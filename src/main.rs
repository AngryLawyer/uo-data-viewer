extern crate sdl2;
extern crate sdl2_ttf;

mod scene;
mod title_scene;

use std::path::Path;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

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

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...
        scene.render(&mut renderer);
    }
}
