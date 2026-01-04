use anyhow::anyhow;
use glam::{Mat4, Vec3};
use std::{collections::BTreeMap, sync::Arc};
use wgpu::Device;

use crate::{
    graphics::{Renderable, TextureRegistry},
    map::Tile,
    mesh::{InstanceData, InstanceMesh, VertexData},
};

pub struct ChunkMesh {
    pub instance_mesh: InstanceMesh,
}

impl ChunkMesh {
    pub fn new(
        device: &Device,
        pos: [i64; 2],
        chunk_spacing: u8,
        size: u8,
        scale: f32,
        texture_registry: Arc<TextureRegistry>,
    ) -> anyhow::Result<Self> {
        let mut vertices: Vec<VertexData> = vec![];
        let mesh_data = Tile::to_mesh_data();
        vertices.extend_from_slice(&mesh_data.vertices);

        let transform_scale = Mat4::from_scale(Vec3::new(scale, scale, 0.0));

        let mut instances: Vec<InstanceData> = vec![];

        let mut sorted_instances: BTreeMap<(i64, i64, i64), InstanceData> = BTreeMap::new();

        let height_map = generate_heightmap(
            &(pos[0] * chunk_spacing as i64, pos[1] * chunk_spacing as i64),
            size,
        )?;

        for ((x, y), z) in height_map.into_iter() {
            for z in 0..=z {
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

use std::{
    collections::HashMap,
    sync::{LazyLock, RwLock},
};

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
