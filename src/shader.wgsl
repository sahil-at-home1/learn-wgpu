// Vertex shader

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
};

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.tex_coords = model.tex_coords;
    out.clip_position = vec4<f32>(model.position, 1.0);
    return out;
}

// Fragment shader
@group(0) @binding(0) // from 1st parameter in set_bind_group()
var t_diffuse: texture_2d<f32>;
@group(0) @binding(1) // from binding specified in BindGroupLayout and BindGroup
var s_diffuse: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // set the color by sampling texture
    return textureSample(t_diffuse, s_diffuse, in.tex_coords);
}

// Fragment shader 2
@fragment
fn fs_main2(in: VertexOutput) -> @location(0) vec4<f32> {
    // set the color by sampling texture
    return textureSample(t_diffuse, s_diffuse, in.tex_coords);
    // set the color
    // return vec4<f32>(in.clip_position.x, in.clip_position.y, in.clip_position.z, 1.0);
}