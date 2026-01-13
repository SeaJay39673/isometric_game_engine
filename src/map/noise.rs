use std::{
    collections::HashMap,
    sync::{LazyLock, RwLock},
};

use anyhow::anyhow;

use noise::{NoiseFn, Perlin};

static PERLIN: LazyLock<RwLock<Perlin>> = LazyLock::new(|| RwLock::new(Perlin::new(0)));

pub fn generate_heightmap(
    postion: &(i64, i64),
    size: u8,
) -> anyhow::Result<HashMap<(i64, i64), i64>> {
    let scale = 0.025;
    let size = size as i64;

    let mut height_map: HashMap<(i64, i64), i64> = HashMap::new();
    for y in (postion.0 - size)..=(postion.0 + size) {
        for x in (postion.1 - size)..=(postion.1 + size) {
            let pos = [(x), (y)];
            let noise = PERLIN
                .read()
                .map_err(|e| anyhow!("Could not get PERLIN to read: {e}"))?
                .get([pos[0] as f64 * scale, pos[1] as f64 * scale]);
            height_map.insert((pos[0], pos[1]), (((noise + 1.0) * 0.5) * 5.0) as i64);
        }
    }

    Ok(height_map)
}
