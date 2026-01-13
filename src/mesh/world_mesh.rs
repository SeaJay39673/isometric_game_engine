use std::{
    collections::{BTreeMap, HashMap},
    path::Path,
    sync::Arc,
};

use wgpu::{BindGroup, Buffer, Device, util::DeviceExt};

use crate::{
    assets::load_blocks,
    game_logic::Entity,
    graphics::{Graphics, Renderable, TextureRegistry},
    map::Chunk,
    mesh::{ChunkMesh, EntityMesh},
};

pub struct WorldMesh {
    chunks_to_render: BTreeMap<(i64, i64), ChunkMesh>,
    chunks_to_update: HashMap<(i64, i64), Chunk>,
    entities_to_render: HashMap<ecs_core::Entity, EntityMesh>,
    entities_to_update: HashMap<ecs_core::Entity, Entity>,
    texture_registry: Arc<TextureRegistry>,
    scale: f32,
    pub time_buffer: Buffer,
    tile_animation_bind_group: BindGroup,
}

impl WorldMesh {
    pub fn new(graphics: &Graphics, scale: f32) -> anyhow::Result<Self> {
        let tile_assets = load_blocks(Path::new("src/assets/tiles"))?;

        let texture_registry = Arc::new(
            TextureRegistry::builder()
                .register_tiles(tile_assets)?
                .build(graphics)?,
        );

        let time_buffer = graphics.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Time Buffer"),
            size: 4,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let uv_buffer = graphics
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("UV Rects Buffer"),
                contents: bytemuck::cast_slice(&texture_registry.uvs.clone()),
                usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            });

        let tile_animation_bind_group =
            graphics
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

        Ok(Self {
            chunks_to_render: BTreeMap::new(),
            chunks_to_update: HashMap::new(),
            entities_to_render: HashMap::new(),
            entities_to_update: HashMap::new(),
            scale,
            time_buffer,
            texture_registry,
            tile_animation_bind_group,
        })
    }

    pub fn update_entity(&mut self, entity: Entity) {
        self.entities_to_update
            .insert(entity.entity.clone(), entity);
    }

    pub fn update_chunk(&mut self, chunk: Chunk) {
        let pos = chunk.pos;
        let pos = (pos[0], pos[1]);
        self.chunks_to_update.insert(pos, chunk);
    }

    pub fn update(&mut self, device: &Device) -> anyhow::Result<()> {
        for (pos, chunk) in &self.chunks_to_update {
            self.chunks_to_render.insert(
                (-pos.0, -pos.1),
                ChunkMesh::new(
                    device,
                    chunk.clone(),
                    self.texture_registry.clone(),
                    self.scale,
                )?,
            );
        }
        self.chunks_to_update = HashMap::new();
        for entity in self.entities_to_update.values() {
            self.entities_to_render.insert(
                entity.entity.clone(),
                EntityMesh::new(device, entity, self.texture_registry.clone(), self.scale)?,
            );
        }
        self.entities_to_update = HashMap::new();
        Ok(())
    }
}

impl Renderable for WorldMesh {
    fn render(&self, render_pass: &mut wgpu::RenderPass) {
        render_pass.set_bind_group(0, &self.texture_registry.atlas.bind_group, &[]);
        render_pass.set_bind_group(1, &self.tile_animation_bind_group, &[]);
        for chunk in self.chunks_to_render.values() {
            chunk.render(render_pass);
        }
        for entity in self.entities_to_render.values() {
            entity.render(render_pass);
        }
    }
}
