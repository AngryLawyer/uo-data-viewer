pub mod lens;

use std::fs::File;
use std::io::Result;
use std::path::Path;

use uorustlibs::map::{MapReader, StaticReader};

use map::lens::MapLens;
use uorustlibs::map::map_size::{FELUCCA, ILSHENAR, MALAS, TRAMMEL};

pub fn map_id_to_facet(id: u8) -> Facet {
    let corrected_id = if id as usize >= MAP_DETAILS.len() {
        0
    } else {
        id as usize
    };
    let (map, idx, statics, (width, height)) = MAP_DETAILS[corrected_id];
    Facet::new(
        &Path::new(map),
        &Path::new(idx),
        &Path::new(statics),
        width / 8,
        height / 8,
    )
}

pub const MAP_DETAILS: [(&'static str, &'static str, &'static str, (u32, u32)); 4] = [
    (
        "./assets/map0.mul",
        "./assets/staidx0.mul",
        "./assets/statics0.mul",
        TRAMMEL,
    ),
    (
        "./assets/map0.mul",
        "./assets/staidx0.mul",
        "./assets/statics0.mul",
        FELUCCA,
    ),
    (
        "./assets/map2.mul",
        "./assets/staidx2.mul",
        "./assets/statics2.mul",
        ILSHENAR,
    ),
    (
        "./assets/map3.mul",
        "./assets/staidx3.mul",
        "./assets/statics3.mul",
        MALAS,
    ),
];

pub struct Facet {
    map_reader: Result<MapReader>,
    static_reader: Result<StaticReader<File>>,
    pub x: u32,
    pub y: u32,
    map_lens: Option<MapLens>,
}

impl Facet {
    pub fn new(
        map_path: &Path,
        static_index: &Path,
        static_path: &Path,
        width_blocks: u32,
        height_blocks: u32,
    ) -> Facet {
        Facet {
            map_reader: MapReader::new(map_path, width_blocks, height_blocks),
            static_reader: StaticReader::new(
                static_index,
                static_path,
                width_blocks,
                height_blocks,
            ),
            x: 0,
            y: 0,
            map_lens: None,
        }
    }

    pub fn get_lens(&mut self, blocks_width: u32, blocks_height: u32) -> Option<MapLens> {
        let lens = match (
            &mut self.map_lens,
            &mut self.map_reader,
            &mut self.static_reader,
        ) {
            (&mut Some(ref lens), &mut Ok(ref mut map_reader), &mut Ok(ref mut static_reader)) => {
                Some(lens.update(map_reader, static_reader, self.x, self.y))
            }
            (&mut None, &mut Ok(ref mut map_reader), &mut Ok(ref mut static_reader)) => {
                Some(MapLens::new(
                    map_reader,
                    static_reader,
                    self.x,
                    self.y,
                    blocks_width,
                    blocks_height,
                ))
            }
            _ => None,
        };
        self.map_lens = lens;
        self.map_lens.clone()
    }
}
