use crate::tile::{Tile, TileType, TileTypes};

pub struct World {
    tiles: Vec<Tile>,
}

impl World {
    pub fn new(w: usize, h: usize, fill_tile: TileType) -> Self {
        Self {
            tiles: vec![
                Tile {
                    tile_type: fill_tile
                };
                w * h
            ],
        }
    }

    pub fn get(x: usize, y: usize) {}
}
