use glam::Vec3;
use glium::{
    texture::{texture2d_array::Texture2dArray, RawImage2d},
    Display,
};
use serde::Deserialize;

use std::fs::{self, File};
use std::io::BufReader;
use std::path::{Path, PathBuf};

pub type TileType = usize;

#[derive(Copy, Clone, Debug)]
pub struct Tile {
    pub tile_type: TileType,
}

#[derive(Deserialize, Debug)]
pub struct TileTypes {
    types: Vec<TileProperties>,
}

impl TileTypes {
    pub fn load(file: impl AsRef<Path>) -> Self {
        ron::from_str::<Self>(&fs::read_to_string(file.as_ref()).unwrap()).unwrap()
    }

    pub fn get(&self, tile_type: TileType) -> &TileProperties {
        &self.types[tile_type]
    }

    pub fn len(&self) -> usize {
        self.types.len()
    }

    pub fn build_texture(&self, display: &Display) -> Texture2dArray {
        let start = std::time::Instant::now();

        let textures = self
            .types
            .iter()
            .map(|tile_type| {
                let path = [
                    "assets",
                    "textures",
                    &format!("{}.png", tile_type.system_name),
                ]
                .iter()
                .collect::<PathBuf>();

                let image = image::load(
                    BufReader::new(File::open(&path).expect("open image")),
                    image::ImageFormat::Png,
                )
                .expect("load image")
                .to_rgba8();

                log::info!("Loaded image {}", path.to_string_lossy());

                let dimensions = image.dimensions();

                RawImage2d::from_raw_rgba(image.into_raw(), dimensions)
            })
            .collect::<Vec<_>>();

        let texture = Texture2dArray::new(display, textures).expect("create texture array");

        let end = std::time::Instant::now();
        log::info!("Texture creation took {}ms", (end - start).as_millis());

        texture
    }
}

#[derive(Deserialize, Debug)]
pub struct TileProperties {
    pub system_name: String,
    pub display_name: String,

    #[serde(default)]
    pub lighting: TileLighting,
}

#[derive(Deserialize, Debug)]
pub enum TileLighting {
    None,
    Colored { intensity: usize, color: Vec3 },
    White { intensity: usize },
}

impl TileLighting {
    pub fn emits_light(&self) -> bool {
        matches!(self, Self::Colored { .. } | Self::White { .. })
    }
}

impl Default for TileLighting {
    fn default() -> Self {
        Self::None
    }
}
