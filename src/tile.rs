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
        log::info!("Loading textures...");
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
                log::info!("Loading texture at {}", path.to_string_lossy());

                let image = image::load(
                    BufReader::new(File::open(path).unwrap()),
                    image::ImageFormat::Png,
                )
                .unwrap()
                .to_rgba8();
                let dimensions = image.dimensions();

                RawImage2d::from_raw_rgba(image.into_raw(), dimensions)
            })
            .collect::<Vec<_>>();

        Texture2dArray::new(display, textures).expect("create texture array")
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
