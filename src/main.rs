extern crate cgmath;
extern crate ggez;
extern crate image;
extern crate uorustlibs;

mod anim_scene;
mod caches;
mod engine;
mod gump_scene;
mod hues_scene;
mod image_convert;
mod map;
mod map_scene;
mod scene;
mod skills_scene;
mod statics_scene;
mod texmaps_scene;
mod tile_scene;
mod title_scene;
mod font_scene;
mod world_scene;

use ggez::conf::WindowSetup;
use ggez::event;
use ggez::ContextBuilder;

fn main() {
    // Make a Context.
    let (mut ctx, mut event_loop) = ContextBuilder::new("UO Data Viewer", "Angry Lawyer")
        .window_setup(WindowSetup::default().title("UO Data Viewer"))
        .build()
        .expect("Could not create context");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let mut my_game = engine::Engine::new(&mut ctx);

    // Run!
    match event::run(&mut ctx, &mut event_loop, &mut my_game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e),
    }
}
