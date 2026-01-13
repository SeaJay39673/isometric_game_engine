use anyhow::anyhow;
use glam::{Mat4, Vec3};
use std::{collections::BTreeMap, sync::Arc};
use wgpu::Device;

use crate::{
    graphics::{Renderable, TextureRegistry, Tile},
    map::Chunk,
    mesh::{InstanceData, InstanceMesh, VertexData},
};

pub struct ChunkMesh {
    pub instance_mesh: InstanceMesh,
}

impl ChunkMesh {
    pub fn new(
        device: &Device,
        chunk: Chunk,
        texture_registry: Arc<TextureRegistry>,
        scale: f32,
    ) -> anyhow::Result<Self> {
        let mut vertices: Vec<VertexData> = vec![];
        let mesh_data = Tile::to_mesh_data();
        vertices.extend_from_slice(&mesh_data.vertices);

        let transform_scale = Mat4::from_scale(Vec3::new(scale, scale, 0.0));

        let mut instances: Vec<InstanceData> = vec![];

        let mut sorted_instances: BTreeMap<(i64, i64, i64), InstanceData> = BTreeMap::new();

        for tile in chunk.tiles {
            let pos = tile.pos;
            let (x, y, z) = (pos[0], pos[1], pos[2]);
            let pos = (-x, -y, z);
            let x = x as f32;
            let y = y as f32;
            let z = z as f32;
            let model = Mat4::from_translation(Vec3 {
                x: (x - y) * scale,
                y: (x + y) * 0.5 * scale + (z * scale),
                z: 0.0,
            }) * transform_scale;
            let tile: &Tile = texture_registry
                .handles
                .get("grass")
                .ok_or(anyhow!("Could not find grass texture handle"))?;
            let data = tile.to_instance_data(model);
            sorted_instances.insert(pos, data);
        }

        instances.extend(sorted_instances.values());

        let instance_mesh = InstanceMesh::new(device, &vertices, mesh_data.indices, &instances);

        Ok(Self {
            instance_mesh: instance_mesh,
        })
    }
}

impl Renderable for ChunkMesh {
    fn render(&self, render_pass: &mut wgpu::RenderPass) {
        self.instance_mesh.render(render_pass);
    }
}
