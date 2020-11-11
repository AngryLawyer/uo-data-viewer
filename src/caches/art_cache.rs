use ggez::graphics::Image;
use ggez::Context;
use image_convert::image_to_surface;
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use uorustlibs::art::{Art, ArtReader};

pub struct ArtCache {
    tile_cache: HashMap<u32, Option<Image>>,
    reader: ArtReader<File>,
}

impl ArtCache {
    pub fn new() -> ArtCache {
        let reader = ArtReader::new(
            &Path::new("./assets/artidx.mul"),
            &Path::new("./assets/art.mul"),
        )
        .expect("Could not load art");
        ArtCache {
            tile_cache: HashMap::new(),
            reader,
        }
    }

    pub fn read_tile(&mut self, ctx: &mut Context, id: u32) -> &Option<Image> {
        if self.tile_cache.contains_key(&id) {
            self.tile_cache.get(&id).unwrap()
        } else {
            match self.reader.read_tile(id) {
                Ok(tile) => {
                    let image = tile.to_image();
                    let tile_image = image_to_surface(ctx, &image);
                    self.tile_cache.insert(id, Some(tile_image));
                }
                Err(_) => {
                    self.tile_cache.insert(id, None);
                }
            }
            self.tile_cache.get(&id).unwrap()
        }
    }
}
