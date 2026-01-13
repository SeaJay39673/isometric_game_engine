use std::{collections::BTreeMap, path::Path, sync::Arc};

use wgpu::{BindGroup, Buffer, util::DeviceExt};

use crate::{
    assets::load_blocks,
    graphics::{Graphics, Renderable, TextureRegistry},
    mesh::ChunkMesh,
};

pub struct ChunkMeshes {
    meshes: BTreeMap<(i64, i64), ChunkMesh>,
    bind_group: BindGroup,
    texture_registry: Arc<TextureRegistry>,
}

impl ChunkMeshes {
    pub fn new(
        graphics: &Graphics,
        chunks_radius: u8,
        chunk_size: u8,
        scale: f32,
    ) -> anyhow::Result<Self> {
        let mut meshes: BTreeMap<(i64, i64), ChunkMesh> = BTreeMap::new();

        let size_i64 = chunks_radius as i64;

        for x in -(size_i64)..=size_i64 {
            for y in -(size_i64)..=size_i64 {
                meshes.insert(
                    (-x, -y),
                    ChunkMesh::new(
                        &graphics.device,
                        [x, y],
                        chunk_size * 2 + 1,
                        chunk_size,
                        scale,
                        texture_registry.clone(),
                    )?,
                );
            }
        }

        Ok(Self {
            meshes,
            time_buffer,
            bind_group,
            texture_registry: texture_registry.clone(),
        })
    }
}

impl Renderable for ChunkMeshes {
    fn render(&self, render_pass: &mut wgpu::RenderPass) {
        render_pass.set_bind_group(0, &self.texture_registry.atlas.bind_group, &[]);
        render_pass.set_bind_group(1, &self.bind_group, &[]);
        self.meshes
            .values()
            .for_each(|chunk_mesh| chunk_mesh.render(render_pass));
    }
}
