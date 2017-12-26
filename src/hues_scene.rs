use scene::SceneName;
use engine::EngineData;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::surface::Surface;
use std::io::Result;
use std::path::Path;
use uorustlibs::hues::{HueReader, HueGroup, Hue};
use uorustlibs::color::Color as ColorTrait;

use std::io::{Error};
use std::fs::{File};
use sdl2::render::{WindowCanvas, Texture, TextureCreator};
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::video::WindowContext;
use sdl2_engine_helpers::scene::{Scene, BoxedScene, SceneChangeEvent};

pub struct HuesScene<'a> {
    texture_creator: &'a TextureCreator<WindowContext>,
    reader: Result<HueReader<File>>,
    index: u32,
    texture: Option<Texture<'a>>,
    exiting: bool,
}

impl<'a> HuesScene<'a> {
    pub fn new<'b>(engine_data: &mut EngineData<'b>, texture_creator: &'a TextureCreator<WindowContext>) -> BoxedScene<'a, Event, SceneName, EngineData<'b>> {
        let mut scene = Box::new(HuesScene {
            reader: HueReader::new(&Path::new("./assets/hues.mul")),
            texture: None,
            index: 0,
            exiting: false,
            texture_creator
        });
        scene.load_group(engine_data);
        scene
    }

    fn load_group(&mut self, engine_data: &mut EngineData) {
        let mut surface = Surface::new(1024, 768, PixelFormatEnum::RGBA8888).unwrap();
        let maybe_group = match self.reader {
            Ok(ref mut hue_reader) => {
                hue_reader.read_hue_group(self.index)
            },
            Err(ref x) => Err(Error::new(x.kind(), "Whoops"))
        };
        match maybe_group {
            Ok(group) => {
                let drawn_group = self.draw_hue_group(self.index, &group, engine_data);
                drawn_group.blit(None, &mut surface, Some(Rect::new(0, 0, drawn_group.width(), drawn_group.height()))).expect("Failed to blit");
            },
            Err(_) => ()
        };
        self.texture = Some(self.texture_creator.create_texture_from_surface(&surface).unwrap());
    }

    fn draw_hue_group(&self, group_idx: u32, group: &HueGroup, engine_data: &mut EngineData) -> Surface {
        let mut surface = Surface::new(256, 64 * 9, PixelFormatEnum::RGBA8888).unwrap();
        for (idx, hue) in group.entries.iter().enumerate() {
            let drawn_hue = self.draw_hue(&hue, engine_data);
            drawn_hue.blit(None, &mut surface, Some(Rect::new(0, idx as i32 * drawn_hue.height() as i32, drawn_hue.width(), drawn_hue.height()))).unwrap();
        }
        let label = engine_data.text_renderer.create_text(&format!("Group {} - {}", group_idx, group.header), Color::RGBA(255, 255, 255, 255));
        label.blit(None, &mut surface, Some(Rect::new(0, 64 * 8 + 32, label.width(), label.height()))).expect("Failed to blit");
        surface
    }

    fn draw_hue(&self, hue: &Hue, engine_data: &mut EngineData) -> Surface {
        let mut surface = Surface::new(256, 64, PixelFormatEnum::RGBA8888).unwrap();
        for (col_idx, &color) in hue.color_table.iter().enumerate() {
            let (r, g, b, _) = color.to_rgba();
            surface.fill_rect(Some(Rect::new((col_idx as i32 * 16), 0, 16, 64)), Color::RGB(r, g, b)).unwrap();
        };
        let label_text = format!("{}: {} - {}", if hue.name.trim().len() > 0 { &hue.name } else { "NONE" }, hue.table_start, hue.table_end);
        let label = engine_data.text_renderer.create_text(&label_text, Color::RGBA(255, 255, 255, 255));
        label.blit(None, &mut surface, Some(Rect::new(0, 48, label.width(), label.height()))).expect("Failed to blit");
        surface
    }
}

impl<'a, 'b> Scene<Event, SceneName, EngineData<'b>> for HuesScene<'a> {
    fn render(&self, renderer: &mut WindowCanvas, _engine_data: &EngineData, _tick: u64) {
        renderer.clear();
        match self.texture {
            Some(ref texture) => {
                renderer.copy(texture, None, None).unwrap();
            },
            None => ()
        };
        renderer.present();
    }

    fn handle_event(&mut self, event: &Event, engine_data: &mut EngineData<'b>, _tick: u64) {
        match *event {
            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                self.exiting = true
            },
            Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                if self.index > 0 {
                    self.index -= 1;
                    self.load_group(engine_data);
                }
            },
            Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                self.index += 1;
                self.load_group(engine_data);
            },
             _ => ()
        }
    }

    fn think(&mut self, _engine_data: &mut EngineData, _tick: u64) -> Option<SceneChangeEvent<SceneName>> {
        if self.exiting {
            Some(SceneChangeEvent::PopScene)
        } else {
            None
        }
    }
}
