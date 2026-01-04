struct VertexInput {
    @location(0) position: vec3<f32>,
};

struct InstanceInput {
    @location(1) m0 : vec4<f32>,
    @location(2) m1 : vec4<f32>,
    @location(3) m2 : vec4<f32>,
    @location(4) m3 : vec4<f32>,

    @location(5) base_frame    : u32,
    @location(6) frame_count   : u32,
    @location(7) frame_time_ms : u32,

    @location(8) color : vec4<f32>,
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) color: vec4<f32>,
};

struct UVRect {
    min: vec2<f32>,
    max: vec2<f32>,
}

@group(0) @binding(0)
var tex: texture_2d<f32>;

@group(0) @binding(1)
var tex_sampler: sampler;

@group(1) @binding(0)
var<storage, read> uv_rects : array<UVRect>;

@group(1) @binding(1)
var<uniform> time_ms: u32;

@vertex
fn vs_main(
    vertex: VertexInput,
    instance: InstanceInput,
) -> VertexOutput {
    let model = mat4x4<f32>(
        instance.m0,
        instance.m1,
        instance.m2,
        instance.m3,
    );

    var frame_index : u32 = 0u;
    if (instance.frame_count > 1u && instance.frame_time_ms > 0u) {
        let anim_time = time_ms / instance.frame_time_ms;
        frame_index = anim_time % instance.frame_count;
    }

    let uv_rect = uv_rects[frame_index];
    let vertex_uv = (vertex.position.xy + vec2<f32>(1.0)) * 0.5;
    let atlas_uv = uv_rect.min + vertex_uv * (uv_rect.max - uv_rect.min);

    var out : VertexOutput;
    out.position = model * vec4<f32>(vertex.position, 1.0);
    out.uv = atlas_uv;
    out.color = instance.color;

    return out;
}


@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    let tex_color = textureSample(tex, tex_sampler, input.uv);
    return tex_color;
}