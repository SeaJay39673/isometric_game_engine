use crate::map::{Tile, generate_heightmap};

#[derive(Clone)]
pub struct Chunk {
    pub pos: [i64; 2],
    pub tiles: Vec<Tile>,
}

impl Chunk {
    pub fn new(pos: [i64; 2], size: u8) -> anyhow::Result<Self> {
        let height_map = generate_heightmap(&(pos[0], pos[1]), size)?;

        let mut tiles: Vec<Tile> = Vec::new();

        for ((x, y), z) in height_map.into_iter() {
            for z in 0..=z {
                let tile = Tile {
                    pos: [x, y, z],
                    texture_name: String::from("grass"),
                };
                tiles.push(tile);
            }
        }

        Ok(Self { pos, tiles })
    }
}
