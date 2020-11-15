use std::collections::HashMap;
use std::fs::File;
use std::io::Result;
use std::path::Path;

use uorustlibs::map::{Block, Cell, MapReader, StaticLocation, StaticReader};

#[derive(Clone, Copy)]
pub struct Altitudes {
    pub x1y1: i8,
    pub x2y1: i8,
    pub x1y2: i8,
    pub x2y2: i8,
}

pub struct FacetCache {
    map_reader: MapReader,
    static_reader: StaticReader<File>,
    block_cache: HashMap<(u32, u32), (Block, Vec<StaticLocation>)>,
    height_cache: HashMap<(u32, u32), Vec<Altitudes>>,
}

impl FacetCache {
    pub fn new(map_reader: MapReader, static_reader: StaticReader<File>) -> FacetCache {
        FacetCache {
            map_reader,
            static_reader,
            block_cache: HashMap::new(),
            height_cache: HashMap::new(),
        }
    }

    fn read_block_cache(&mut self, x: u32, y: u32) -> &(Block, Vec<StaticLocation>) {
        if (!self.block_cache.contains_key(&(x, y))) {
            let block = self.map_reader.read_block_from_coordinates(x, y);
            let statics = self.static_reader.read_block_from_coordinates(x, y);
            self.block_cache.insert(
                (x, y),
                (block.ok().unwrap(), statics.ok().unwrap_or(vec![])),
            ); // FIXME: Out of bounds errors
        }
        self.block_cache.get(&(x, y)).unwrap()
    }

    fn read_altitudes(&mut self, x: u32, y: u32) -> &Vec<Altitudes> {
        if (!self.height_cache.contains_key(&(x, y))) {
            let (block, _) = self.read_block_cache(x, y).clone(); // FIXME: Do this without clones
            let (block_x2, _) = self.read_block_cache(x + 1, y).clone();
            let (block_y2, _) = self.read_block_cache(x, y + 1).clone();
            let (block_x2y2, _) = self.read_block_cache(x + 1, y + 1).clone();
            let mut collector = vec![];
            for y in 0..8 {
                for x in 0..8 {
                    let cell = block.cells[y * 8 + x];
                    let x1y1 = cell.altitude;
                    let x2y1 = if x < 7 {
                        block.cells[y * 8 + x + 1].altitude
                    } else {
                        block_x2.cells[y * 8].altitude
                    };
                    let x1y2 = if y < 7 {
                        block.cells[(y + 1) * 8 + x].altitude
                    } else {
                        block_y2.cells[x].altitude
                    };
                    let x2y2 = if x < 7 && y < 7 {
                        block.cells[(y + 1) * 8 + x + 1].altitude
                    } else {
                        block_x2y2.cells[0].altitude
                    };
                    collector.push(Altitudes {
                        x1y1,
                        x2y1,
                        x1y2,
                        x2y2,
                    });
                }
            }
            self.height_cache.insert((x, y), collector);
        }
        self.height_cache.get(&(x, y)).unwrap()
    }

    pub fn read_block(&mut self, x: u32, y: u32) -> ((Block, Vec<StaticLocation>), Vec<Altitudes>) {
        // FIXME: Sort out mutable borrows
        let block = self.read_block_cache(x, y).clone();
        let altitudes = self.read_altitudes(x, y).to_vec();
        (block, altitudes)
    }
}
