// Vertex shader

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.color = model.color;
    out.clip_position = vec4<f32>(model.position, 1.0);
    return out;
}

// Fragment shader
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // set the color
    return vec4<f32>(in.color, 1.0);
}

// Fragment shader 2
@fragment
fn fs_main2(in: VertexOutput) -> @location(0) vec4<f32> {
    // set the color
    return vec4<f32>(in.clip_position.x, in.clip_position.y, in.clip_position.z, 1.0);
}