use uorustlibs::map::{MapReader, Block, StaticReader, StaticLocation};

pub struct MapLens {
    pub blocks: Vec<(Option<Block>, Option<Vec<StaticLocation>>)>,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32
}

impl MapLens {
    pub fn new(map_reader: &mut MapReader, static_reader: &mut StaticReader, x: u32, y: u32, width: u32, height: u32) -> MapLens {
        let mut blocks = vec![];
        for yy in 0..height {
            for xx in 0..width {
                let block = map_reader.read_block_from_coordinates(x + xx, y + yy);
                let statics = static_reader.read_block_from_coordinates(x + xx, y + yy);
                blocks.push((block.ok(), statics.ok()));
            }
        }
        MapLens {
            blocks: blocks,
            x: x,
            y: y,
            width: width,
            height: height
        }
    }

    pub fn update(&self, map_reader: &mut MapReader, static_reader: &mut StaticReader, x: u32, y: u32) -> MapLens {
        let difference_x = x as i64 - self.x as i64;
        let difference_y = y as i64 - self.y as i64;
        let mut blocks = vec![];
        for yy in 0..self.height {
            for xx in 0..self.width {
                let offset_x = xx as i64 + difference_x;
                let offset_y = yy as i64 + difference_y;
                if offset_x < 0 || offset_y < 0 || offset_x >= self.width as i64 || offset_y >= self.height as i64 {
                    blocks.push((map_reader.read_block_from_coordinates(x + xx, y + yy).ok(), static_reader.read_block_from_coordinates(x + xx, y + yy).ok()));
                } else {
                    blocks.push(self.blocks[(offset_x + (offset_y * self.width as i64)) as usize].clone());
                }
            }
        }
        
        MapLens {
            blocks: blocks,
            x: x,
            y: y,
            width: self.width,
            height: self.height
        }
    }
}
