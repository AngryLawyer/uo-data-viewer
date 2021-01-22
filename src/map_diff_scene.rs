use caches::art_cache::ArtCache;
use caches::facet_cache::read_altitudes;
use caches::texmap_cache::TexMapCache;
use cgmath::Point2;
use ggez::event::{KeyCode, KeyMods};
use ggez::graphics::{self, DrawParam, Text};
use ggez::{Context, GameResult};
use map::render::draw_block;
use map::{map_id_to_facet, Facet, MAP_DETAILS};
use scene::{BoxedScene, Scene, SceneChangeEvent, SceneName};
use std::collections::HashMap;
use std::io::Result;
use std::path::Path;
use uorustlibs::map::{Block, MapDiffReader, StaticDiffReader, StaticLocation};

pub struct MapDiffScene {
    art_cache: ArtCache,
    texmap_cache: TexMapCache,
    map_patches: HashMap<u32, Result<Block>>,
    static_patches: HashMap<u32, Result<Vec<StaticLocation>>>,
    map_id: u8,
    patch_id: u32,
    exiting: bool,
}

impl<'a> MapDiffScene {
    pub fn new(ctx: &mut Context) -> BoxedScene<'a, SceneName, ()> {
        let mut scene = Box::new(MapDiffScene {
            exiting: false,
            map_id: 0,
            patch_id: 0,
            map_patches: MapDiffReader::new(
                &Path::new("./assets/mapdifl0.mul"),
                &Path::new("./assets/mapdif0.mul"),
            )
            .unwrap()
            .read_all(),
            static_patches: StaticDiffReader::new(
                &Path::new("./assets/stadifl0.mul"),
                &Path::new("./assets/stadifi0.mul"),
                &Path::new("./assets/stadif0.mul"),
            )
            .unwrap()
            .read_all(),
            art_cache: ArtCache::new(),
            texmap_cache: TexMapCache::new(),
        });
        scene.get_next_patch();
        scene
    }

    pub fn get_patch_data(&mut self) {
        self.patch_id = 0;
        self.map_patches = MapDiffReader::new(
            &Path::new(&format!("./assets/mapdifl{}.mul", self.map_id)),
            &Path::new(&format!("./assets/mapdif{}.mul", self.map_id)),
        )
        .unwrap()
        .read_all();
        self.static_patches = StaticDiffReader::new(
            &Path::new(&format!("./assets/stadifl{}.mul", self.map_id)),
            &Path::new(&format!("./assets/stadifi{}.mul", self.map_id)),
            &Path::new(&format!("./assets/stadif{}.mul", self.map_id)),
        )
        .unwrap()
        .read_all();
        self.get_next_patch();
    }

    pub fn get_next_patch(&mut self) {
        let mut keys = self.map_patches.keys().collect::<Vec<_>>();
        let mut more_keys = self.static_patches.keys().collect::<Vec<_>>();
        
        keys.append(&mut more_keys);
        keys.sort();
        keys.dedup();
        let next_key = keys.iter().find(|key| ***key > self.patch_id);
        match next_key {
            Some(key) => self.patch_id = **key,
            _ => (),
        }
    }

    pub fn get_last_patch(&mut self) {
        let mut keys = self.map_patches.keys().collect::<Vec<_>>();
        let mut more_keys = self.static_patches.keys().collect::<Vec<_>>();
        
        keys.append(&mut more_keys);
        keys.sort();
        keys.dedup();
        keys.reverse();
        let next_key = keys.iter().find(|key| ***key < self.patch_id);
        match next_key {
            Some(key) => self.patch_id = **key,
            _ => (),
        }
    }

    pub fn draw_page(&mut self, ctx: &mut Context) -> GameResult<()> {
        let block = match self.map_patches.get(&self.patch_id) {
            Some(Ok(block)) => Some(block),
            _ => None,
        };
        let default = vec![];
        let statics = match self.static_patches.get(&self.patch_id) {
            Some(Ok(statics)) => statics,
            _ => &default,
        };
        let altitudes = if block.is_some() { read_altitudes(block.unwrap(), None, None, None) } else { vec![] };
        let transform = Point2::new(300.0, 200.0);
        draw_block(
            ctx,
            &mut self.art_cache,
            &mut self.texmap_cache,
            block,
            &statics,
            &altitudes,
            transform,
        );
        Ok(())
    }
}

impl Scene<SceneName, ()> for MapDiffScene {
    fn draw(&mut self, ctx: &mut Context, _engine_data: &mut ()) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);
        self.draw_page(ctx);
        let label = Text::new(format!("Map {} patch {}", self.map_id, self.patch_id));
        graphics::draw(ctx, &label, (Point2::new(0.0, 0.0), graphics::WHITE))?;
        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
        _engine_data: &mut (),
    ) {
        match keycode {
            KeyCode::Escape => self.exiting = true,
            KeyCode::Left => {
                self.get_last_patch();
            }
            KeyCode::Right => {
                self.get_next_patch();
            }
            KeyCode::Tab => {
                self.map_id = (self.map_id + 1) % 3 as u8;
                self.get_patch_data();
            }
            _ => (),
        }
    }

    fn update(
        &mut self,
        _ctx: &mut Context,
        _engine_data: &mut (),
    ) -> GameResult<Option<SceneChangeEvent<SceneName>>> {
        if self.exiting {
            Ok(Some(SceneChangeEvent::PopScene))
        } else {
            Ok(None)
        }
    }
}
