// Vertex shader

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) vert_pos: vec3<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> VertexOutput {
    var out: VertexOutput;
    let x = f32(1 - i32(in_vertex_index)) * 0.5;
    let y = f32(i32(in_vertex_index & 1u) * 2 - 1) * 0.5;
    out.clip_position = vec4<f32>(x, y, 0.0, 1.0);
    out.vert_pos = out.clip_position.xyz;
    return out;
}

// Fragment shader
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // set the color
    return vec4<f32>(0.5, 0.5, 0.5, 1.0);
}

// Fragment shader 2
@fragment
fn fs_main2(in: VertexOutput) -> @location(0) vec4<f32> {
    // set the color
    return vec4<f32>(in.clip_position.x, in.clip_position.y, in.clip_position.z, 1.0);
}