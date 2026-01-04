use wgpu::{Extent3d, Origin3d, TexelCopyBufferLayout, TexelCopyTextureInfo};

use crate::graphics::{Graphics, Texture, TextureHandle};
use anyhow::anyhow;

pub type Atlas = Texture;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct UVRect {
    pub min: [f32; 2],
    pub max: [f32; 2],
}

pub struct AtlasImage {
    pub pixels: Vec<u8>,
    pub width: u32,
    pub height: u32,
}

pub struct AtlasBuilder {
    images: Vec<AtlasImage>,
}

impl AtlasBuilder {
    pub fn new() -> Self {
        Self { images: vec![] }
    }

    pub fn add_image(&mut self, image: AtlasImage) -> TextureHandle {
        let id = self.images.len();
        self.images.push(image);
        id as u32
    }

    fn calculate_atlas_height(images: &[AtlasImage], atlas_width: u32) -> u32 {
        let mut x = 0;
        let mut y = 0;
        let mut row_height = 0;

        for img in images {
            if x + img.width > atlas_width {
                x = 0;
                y += row_height;
                row_height = 0;
            }

            x += img.width;
            row_height = row_height.max(img.height);
        }

        y + row_height
    }

    pub fn build(self, graphics: &Graphics) -> anyhow::Result<(Atlas, Vec<UVRect>)> {
        if self.images.len() == 0 {
            return Err(anyhow!("AtlasBuilder cannot build: images are empty!"));
        }

        let atlas_width = 2048;
        let atlas_height = AtlasBuilder::calculate_atlas_height(&self.images, atlas_width);

        let atlas_width_f = atlas_width as f32;
        let atlas_height_f = atlas_height as f32;

        let atlas_texture =
            Texture::from_color(graphics, [255, 255, 255, 255], atlas_width, atlas_height);

        let mut uvs: Vec<UVRect> = vec![];

        let mut x: u32 = 0;
        let mut y: u32 = 0;
        let mut row_height: u32 = 0;

        for image in self.images {
            if x + image.width > atlas_width {
                x = 0;
                y += row_height;
                row_height = 0;
            }

            let mut flipped_pixels = vec![0u8; (image.width * image.height * 4) as usize];
            for row in 0..image.height {
                let src_start = (row * image.width * 4) as usize;
                let src_end = src_start + (image.width * 4) as usize;

                let dest_start = ((image.height - 1 - row) * image.width * 4) as usize;
                let dest_end = dest_start + (image.width * 4) as usize;

                flipped_pixels[dest_start..dest_end]
                    .copy_from_slice(&image.pixels[src_start..src_end]);
            }

            graphics.queue.write_texture(
                TexelCopyTextureInfo {
                    texture: &atlas_texture.texture,
                    aspect: wgpu::TextureAspect::All,
                    mip_level: 0,
                    origin: Origin3d { x, y, z: 0 },
                },
                &flipped_pixels,
                TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(image.width * 4),
                    rows_per_image: Some(image.height),
                },
                Extent3d {
                    width: image.width,
                    height: image.height,
                    depth_or_array_layers: 1,
                },
            );

            let xf = x as f32;
            let yf = y as f32;

            uvs.push(UVRect {
                min: [xf / atlas_width_f, yf / atlas_height_f],
                max: [
                    (xf + image.width as f32) / atlas_width_f,
                    (yf + image.height as f32) / atlas_height_f,
                ],
            });

            x += image.width;
            row_height = row_height.max(image.height);
        }

        Ok((atlas_texture, uvs))
    }
}
