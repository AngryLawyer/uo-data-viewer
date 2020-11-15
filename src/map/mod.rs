
use std::fs::File;
use std::io::Result;
use std::path::Path;

use uorustlibs::map::{MapReader, StaticReader, Block, StaticLocation};

use uorustlibs::map::map_size::{FELUCCA, ILSHENAR, MALAS, TRAMMEL};
use crate::caches::facet_cache::{FacetCache, Altitudes};

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
    facet_cache: FacetCache
}

impl Facet {
    pub fn new(
        map_path: &Path,
        static_index: &Path,
        static_path: &Path,
        width_blocks: u32,
        height_blocks: u32,
    ) -> Facet {
        let facet_cache = FacetCache::new(MapReader::new(map_path, width_blocks, height_blocks).unwrap(), StaticReader::new(
            static_index,
            static_path,
            width_blocks,
            height_blocks
        ).unwrap());
        Facet {
            facet_cache
        }
    }

    pub fn read_block(&mut self, x: u32, y: u32) -> ((Block, Vec<StaticLocation>), Vec<Altitudes>) {
        self.facet_cache.read_block(x, y)
    }
}
