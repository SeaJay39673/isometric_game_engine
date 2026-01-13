use std::collections::HashMap;

use anyhow::anyhow;

use crate::{
    assets::TileAsset,
    graphics::{Atlas, AtlasBuilder, AtlasImage, Graphics, Tile, TileTextureHandle, UVRect},
};

pub struct TextureRegistry {
    pub atlas: Atlas,
    pub uvs: Vec<UVRect>,
    pub handles: HashMap<String, Tile>,
}

impl TextureRegistry {
    pub fn builder() -> TextureRegistryBuilder {
        TextureRegistryBuilder {
            atlas_builder: AtlasBuilder::new(),
            handles: HashMap::new(),
        }
    }
}
pub struct TextureRegistryBuilder {
    atlas_builder: AtlasBuilder,
    handles: HashMap<String, Tile>,
}

impl TextureRegistryBuilder {
    pub fn build(self, graphics: &Graphics) -> anyhow::Result<TextureRegistry> {
        let (atlas, uvs) = self.atlas_builder.build(graphics)?;
        Ok(TextureRegistry {
            atlas: atlas,
            uvs: uvs,
            handles: self.handles,
        })
    }

    pub fn register_tiles(mut self, tile_assets: Vec<TileAsset>) -> anyhow::Result<Self> {
        for asset in tile_assets {
            let mut frames: Vec<TileTextureHandle> = vec![];
            for frame in asset.frames {
                if !frame.is_file() {
                    return Err(anyhow!(
                        "TileAsset {} frame: {:?} is not a file",
                        asset.name,
                        frame
                    ));
                }
                let img = image::open(frame)?.to_rgba8();
                let (width, height) = img.dimensions();
                let id = self.atlas_builder.add_image(AtlasImage {
                    width,
                    height,
                    pixels: img.into_raw(),
                });
                frames.push(id);
            }
            self.handles.insert(
                asset.name.clone(),
                Tile {
                    name: asset.name,
                    tint: asset.tint,
                    animation: asset.animation,
                    frames,
                },
            );
        }

        Ok(self)
    }
}
