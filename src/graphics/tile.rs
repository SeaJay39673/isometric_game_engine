use glam::Mat4;

use crate::{
    assets::AnimationDef,
    graphics::TileTextureHandle,
    mesh::{InstanceData, MeshData, VertexData},
};

pub struct Tile {
    pub name: String,
    pub tint: [u8; 4],
    pub frames: Vec<TileTextureHandle>,
    pub animation: Option<AnimationDef>,
}

impl Tile {
    pub fn to_mesh_data() -> MeshData {
        let vertices: Vec<VertexData> = vec![
            VertexData {
                position: [-1.0, -1.0, 0.0],
            },
            VertexData {
                position: [-1.0, 1.0, 0.0],
            },
            VertexData {
                position: [1.0, 1.0, 0.0],
            },
            VertexData {
                position: [1.0, -1.0, 0.0],
            },
        ];

        let indices: Vec<u32> = vec![0, 1, 2, 2, 3, 0];

        MeshData {
            vertices: vertices,
            indices: Some(indices),
        }
    }

    pub fn to_instance_data(&self, model: Mat4) -> InstanceData {
        InstanceData {
            model: model.to_cols_array_2d(),
            base_frame: self.frames[0] as u32,
            frame_count: self.frames.len() as u32,
            frame_time_ms: if let Some(animation) = &self.animation {
                animation.frame_time_ms
            } else {
                0
            },
            color: self.tint,
            _padding: [0u32; 3],
        }
    }
}
