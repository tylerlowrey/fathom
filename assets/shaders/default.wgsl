struct Uniforms {
    modelViewProjectionMat: mat4x4<f32>
};

@group(0) @binding(0) var<uniform> uniforms: Uniforms;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec4<f32>,
}

@vertex
fn vertex_main(vertex_in: VertexInput) -> VertexOutput {
    var vertex_out: VertexOutput;
    vertex_out.position = uniforms.modelViewProjectionMat * vec4<f32>(vertex_in.position, 1.0);
    vertex_out.color = vec4<f32>(vertex_in.color, 1.0);
    return vertex_out;
}

@fragment
fn fragment_main(vertex_in: VertexOutput) -> @location(0) vec4<f32> {
    return vertex_in.color;
}



