use std::{collections::BTreeMap, path::Path, sync::Arc};

use wgpu::{BindGroup, Buffer, util::DeviceExt};

use crate::{
    assets::load_blocks,
    graphics::{Graphics, Renderable, TextureRegistry},
    mesh::ChunkMesh,
};

pub struct ChunkMeshes {
    meshes: BTreeMap<(i64, i64), ChunkMesh>,
    pub time_buffer: Buffer,
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

        let tile_assets = load_blocks(Path::new("src/assets/tiles"))?;

        let texture_registry = Arc::new(
            TextureRegistry::builder()
                .register_tiles(tile_assets)?
                .build(graphics)?,
        );

        let uv_buffer = graphics
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("UV Rects Buffer"),
                contents: bytemuck::cast_slice(&texture_registry.uvs.clone()),
                usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            });

        let time_buffer = graphics.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Time Buffer"),
            size: 4,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let bind_group = graphics
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("Animation Bind Group"),
                layout: &graphics.animation_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: uv_buffer.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: time_buffer.as_entire_binding(),
                    },
                ],
            });

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
