use ggez::graphics::Image;
use ggez::Context;
use image_convert::image_to_surface;
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use uorustlibs::texmaps::TexMapsReader;

pub struct TexMapCache {
    tex_map_cache: HashMap<u32, Option<Image>>,
    reader: TexMapsReader<File>,
}

impl TexMapCache {
    pub fn new() -> TexMapCache {
        let reader = TexMapsReader::new(
            &Path::new("./assets/texidx.mul"),
            &Path::new("./assets/texmaps.mul"),
        )
        .expect("Could not load texmaps");
        TexMapCache {
            tex_map_cache: HashMap::new(),
            reader,
        }
    }

    pub fn read_texmap(&mut self, ctx: &mut Context, id: u32) -> &Option<Image> {
        if self.tex_map_cache.contains_key(&id) {
            self.tex_map_cache.get(&id).unwrap()
        } else {
            match self.reader.read(id) {
                Ok(tile) => {
                    let image = tile.to_image();
                    let tile_image = image_to_surface(ctx, &image);
                    self.tex_map_cache.insert(id, Some(tile_image));
                }
                Err(_) => {
                    self.tex_map_cache.insert(id, None);
                }
            }
            self.tex_map_cache.get(&id).unwrap()
        }
    }
}
