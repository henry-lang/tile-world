use crate::tile::{Tile, TileType, TileTypes};

pub struct World {
    tiles: Vec<Tile>,
    width: usize,
    height: usize,
}

impl World {
    pub fn new(width: usize, height: usize, fill_tile: TileType) -> Self {
        Self {
            tiles: vec![
                Tile {
                    tile_type: fill_tile
                };
                width * height
            ],
            width,
            height,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Tile {
        self.tiles[x + y * self.width]
    }
}
