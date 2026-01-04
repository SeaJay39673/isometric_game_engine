use anyhow::anyhow;
use serde::Deserialize;
use std::{
    fs,
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

#[derive(Deserialize)]
pub struct TileDef {
    pub name: String,
    pub tint: [u8; 4],
    pub animation: Option<AnimationDef>,
}

#[derive(Deserialize)]
pub struct AnimationDef {
    pub frame_time_ms: u32,
    pub looped: bool,
}

pub struct TileAsset {
    pub name: String,
    pub tint: [u8; 4],
    pub frames: Vec<PathBuf>,
    pub animation: Option<AnimationDef>,
}

pub fn load_blocks(path: &Path) -> anyhow::Result<Vec<TileAsset>> {
    let mut assets: Vec<TileAsset> = vec![];

    for result in WalkDir::new(path).min_depth(1).max_depth(1) {
        let entry = match result {
            Ok(entry) => entry,
            Err(e) => return Err(anyhow!("{e}")),
        };

        let path = entry.path();

        if !path.is_dir() {
            continue;
        }

        let def_str = fs::read_to_string(path.join("tile.ron"))?;
        let def: TileDef = ron::from_str(&def_str)?;

        assets.push(load_block(path, def)?);
    }

    Ok(assets)
}

pub fn load_block(dir: &Path, def: TileDef) -> anyhow::Result<TileAsset> {
    let mut frames: Vec<PathBuf> = vec![];

    if !dir.is_dir() {
        return Err(anyhow!(
            "Error in load_block: provided dir is not a directory!"
        ));
    }

    for result in fs::read_dir(&dir)? {
        let entry = match result {
            Ok(entry) => entry,
            Err(e) => return Err(anyhow!("{e}")),
        };
        let path = entry.path();

        let ext = if let Some(ext) = path.extension() {
            ext
        } else {
            return Err(anyhow!("No file extension found for file: {:?}", path));
        };

        if ext == "ron" {
            continue;
        }

        if ext == "png" {
            frames.push(path);
        } else {
            return Err(anyhow!("Extension invalid: {:?}", ext));
        }
    }

    if frames.len() == 0 {
        return Err(anyhow!(
            "No texture images provided for directory: {:?}",
            dir
        ));
    }

    frames.sort();

    Ok(TileAsset {
        name: def.name,
        tint: def.tint,
        frames,
        animation: def.animation,
    })
}
