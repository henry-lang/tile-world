mod shaders;
mod tile;
mod world;

use glium::{
    glutin::{self, event_loop::EventLoop, window::WindowBuilder, ContextBuilder},
    Display,
};
use simple_logger::SimpleLogger;
use std::path::PathBuf;

use shaders::Shaders;
use tile::TileTypes;

struct Resources {
    tile_types: TileTypes,
}

fn create_display(event_loop: &EventLoop<()>) -> Display {
    let wb = WindowBuilder::new();
    let cb = ContextBuilder::new().with_vsync(true);

    Display::new(wb, cb, event_loop).expect("create display")
}

fn main() {
    SimpleLogger::new().init().unwrap();

    let event_loop = glutin::event_loop::EventLoop::new();
    let display = create_display(&event_loop);

    let shaders = Shaders::create_all(&display);

    let tile_types = TileTypes::load(["assets", "tile_types.ron"].iter().collect::<PathBuf>());
    log::info!("Found {} tile types from tile_types.ron", tile_types.len());

    tile_types.build_texture(&display);
}
