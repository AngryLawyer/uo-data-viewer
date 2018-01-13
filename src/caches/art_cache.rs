use uorustlibs::art::{ArtReader, Art};
use sdl2::render::{Texture, TextureCreator};
use std::collections::HashMap;
use sdl2::video::WindowContext;
use std::fs::File;
use std::path::Path;
use image_convert::image_to_surface;

pub struct ArtCache<'a> {
    tile_cache: HashMap<u32, Option<Texture<'a>>>,
    texture_creator: &'a TextureCreator<WindowContext>,
    reader: ArtReader<File>,
}

impl<'a> ArtCache<'a> {
    pub fn new(texture_creator: &'a TextureCreator<WindowContext>) -> ArtCache<'a> {
        let reader = ArtReader::new(&Path::new("./assets/artidx.mul"), &Path::new("./assets/art.mul")).expect("Could not load art");
        ArtCache {
            tile_cache: HashMap::new(),
            texture_creator,
            reader
        }
    }

    pub fn read_tile(&mut self, id: u32) -> &Option<Texture<'a>> {
        if self.tile_cache.contains_key(&id) {
            self.tile_cache.get(&id).unwrap()
        } else {
            match self.reader.read_tile(id) {
                Ok(tile) => {
                    let image = tile.to_image();
                    let tile_image = image_to_surface(&image);
                    self.tile_cache.insert(id, Some(self.texture_creator.create_texture_from_surface(&tile_image).expect("Could not create tile")));
                },
                Err(_) => {
                    self.tile_cache.insert(id, None);
                }
            }
            self.tile_cache.get(&id).unwrap()
        }
    }
}
