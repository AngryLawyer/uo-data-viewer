use ggez::graphics::Image;
use ggez::Context;
use image_convert::image_to_surface;
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use uorustlibs::art::{Art, ArtReader};
use uorustlibs::tiledata::{MapTileData, StaticTileData, TileDataReader};

pub struct ArtCache {
    tile_cache: HashMap<u32, Option<(Image, MapTileData)>>,
    static_cache: HashMap<u32, Option<(Image, StaticTileData)>>,
    reader: ArtReader<File>,
    tiledata_reader: TileDataReader
}

impl ArtCache {
    pub fn new() -> ArtCache {
        let reader = ArtReader::new(
            &Path::new("./assets/artidx.mul"),
            &Path::new("./assets/art.mul"),
        )
        .expect("Could not load art");
        let tiledata_reader = TileDataReader::new(
            &Path::new("./assets/tiledata.mul"),
        )
        .expect("Could not load tiledata");
        ArtCache {
            tile_cache: HashMap::new(),
            static_cache: HashMap::new(),
            reader,
            tiledata_reader
        }
    }

    pub fn read_tile(&mut self, ctx: &mut Context, id: u32) -> &Option<(Image, MapTileData)> {
        if self.tile_cache.contains_key(&id) {
            self.tile_cache.get(&id).unwrap()
        } else {
            match (self.reader.read_tile(id), self.tiledata_reader.read_map_tile_data(id)) {
                (Ok(tile), Ok(tiledata)) => {
                    let image = tile.to_image();
                    let tile_image = image_to_surface(ctx, &image);
                    self.tile_cache.insert(id, Some((tile_image, tiledata)));
                }
                _ => {
                    self.tile_cache.insert(id, None);
                }
            }
            self.tile_cache.get(&id).unwrap()
        }
    }

    pub fn read_static(&mut self, ctx: &mut Context, id: u32) -> &Option<(Image, StaticTileData)> {
        if self.static_cache.contains_key(&id) {
            self.static_cache.get(&id).unwrap()
        } else {
            match (self.reader.read_static(id), self.tiledata_reader.read_static_tile_data(id)) {
                (Ok(tile), Ok(tiledata)) => {
                    let image = tile.to_image();
                    let tile_image = image_to_surface(ctx, &image);
                    self.static_cache.insert(id, Some((tile_image, tiledata)));
                }
                _ => {
                    self.static_cache.insert(id, None);
                }
            }
            self.static_cache.get(&id).unwrap()
        }
    }
}
