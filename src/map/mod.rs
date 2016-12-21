pub mod lens;

use std::path::Path;
use std::io::Result;
use std::fs::File;

use uorustlibs::map::{MapReader, Block, RadarColReader, StaticReader, StaticLocation};

use map::lens::MapLens;


//FIXME: Unify
const MAX_BLOCKS_WIDTH: u32 = 1024 / 8;
const MAX_BLOCKS_HEIGHT: u32 = 768 / 8;

pub struct Facet {
    map_reader: Result<MapReader>,
    static_reader: Result<StaticReader<File>>,
    pub x: u32,
    pub y: u32,
    map_lens: Option<MapLens>
}

impl Facet {
    pub fn new(map_path: &Path, static_index: &Path, static_path: &Path, width_blocks: u32, height_blocks: u32) -> Facet {
        Facet {
            map_reader: MapReader::new(map_path, width_blocks, height_blocks),
            static_reader: StaticReader::new(static_index, static_path, width_blocks, height_blocks),
            x: 0,
            y: 0,
            map_lens: None
        }
    }

    pub fn get_lens(&mut self) -> &Option<MapLens> {
        let lens = match (&mut self.map_lens, &mut self.map_reader, &mut self.static_reader) {
            (&mut Some(ref lens), &mut Ok(ref mut map_reader), &mut Ok(ref mut static_reader)) => {
                Some(lens.update(map_reader, static_reader, self.x, self.y))
            },
            (&mut None, &mut Ok(ref mut map_reader), &mut Ok(ref mut static_reader)) => {
                Some(MapLens::new(map_reader, static_reader, self.x, self.y, MAX_BLOCKS_WIDTH, MAX_BLOCKS_HEIGHT))
            },
            _ => None
        };
        self.map_lens = lens;
        &self.map_lens
    }
}
