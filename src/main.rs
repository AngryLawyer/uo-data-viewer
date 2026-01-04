mod caches;
mod engine;
mod image_convert;
mod loading_texture;
mod map;
mod scene;

mod anim_scene;
mod gump_scene;
mod hues_scene;
mod map_scene;
mod skills_scene;
mod statics_scene;
mod tile_scene;
mod title_scene;
/*
mod font_scene;
mod map_diff_scene;
mod texmaps_scene;
*/
mod world_scene;

use ggez::ContextBuilder;
use ggez::conf::WindowSetup;
use ggez::event;

fn main() {
    // Make a Context.
    let (mut ctx, event_loop) = ContextBuilder::new("UO Data Viewer", "Angry Lawyer")
        .window_setup(WindowSetup::default().title("UO Data Viewer"))
        .build()
        .expect("Could not create context");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let my_game = engine::Engine::new(&mut ctx);

    // Run!
    event::run(ctx, event_loop, my_game);
}
