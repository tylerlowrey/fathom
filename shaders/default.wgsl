struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) color: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec4<f32>,
};


@vertex
fn vertex_main(vertex_in: VertexInput) -> VertexOutput{
    var out: VertexOutput;
    out.position = vec4<f32>(vertex_in.position, 0.0, 1.0);
    out.color = vec4<f32>(vertex_in.color, 1.0);
    return out;
}

@fragment
fn fragment_main(vertex_in: VertexOutput) -> @location(0) vec4<f32> {
    return vertex_in.color;
}