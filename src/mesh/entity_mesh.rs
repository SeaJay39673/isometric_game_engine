use glam::{Mat4, Vec3};
use std::sync::Arc;
use wgpu::Device;

use crate::{
    game_logic::Entity,
    graphics::{Renderable, TextureRegistry, Tile},
    mesh::{InstanceData, InstanceMesh, VertexData},
};

pub struct EntityMesh {
    pub entity: ecs_core::Entity,
    pub instance_mesh: InstanceMesh,
}

impl EntityMesh {
    pub fn new(
        device: &Device,
        entity: &Entity,
        texture_registry: Arc<TextureRegistry>,
        scale: f32,
    ) -> anyhow::Result<Self> {
        let mut vertices: Vec<VertexData> = vec![];
        let mesh_data = Tile::to_mesh_data();
        vertices.extend_from_slice(&mesh_data.vertices);

        let transform_scale = Mat4::from_scale(Vec3::new(scale, scale, 0.0));

        let [x, y, z] = entity.pos;
        let model = Mat4::from_translation(Vec3 {
            x: (x - y) * scale,
            y: (x + y) * 0.5 * scale + (z * scale),
            z: 0.0,
        }) * transform_scale;

        let tile: &Tile = texture_registry
            .handles
            .get(&entity.texture_name)
            .ok_or_else(|| {
                anyhow::anyhow!("Could not find texture handle: {}", entity.texture_name)
            })?;

        let instance: InstanceData = tile.to_instance_data(model);
        let instances = [instance];

        let instance_mesh = InstanceMesh::new(device, &vertices, mesh_data.indices, &instances);

        Ok(Self {
            entity: entity.entity.clone(),
            instance_mesh,
        })
    }

    pub fn update_transform(&mut self, queue: &wgpu::Queue, pos: [f32; 3], scale: f32) {
        let [x, y, z] = pos;
        let transform_scale = Mat4::from_scale(Vec3::new(scale, scale, 0.0));

        let model = Mat4::from_translation(Vec3 {
            x: (x - y) * scale,
            y: (x + y) * 0.5 * scale + (z * scale),
            z: 0.0,
        }) * transform_scale;

        self.instance_mesh
            .update_instance_model(0, model.to_cols_array_2d(), queue);
    }
}

impl Renderable for EntityMesh {
    fn render(&self, render_pass: &mut wgpu::RenderPass) {
        self.instance_mesh.render(render_pass);
    }
}
