struct Vertex {
    @location(0) position: vec2f,
    @location(1) color: vec3f,
    @location(2) scale: vec2f,
    @location(3) offset: vec2f,
};

struct VSOut {
    @builtin(position) position: vec4f,
    @location(0) color: vec3f,
};

@vertex
fn vertex_main(
    vertex: Vertex,
    @builtin(instance_index) instance_index: u32,
) -> VSOut {
    return VSOut(
	vec4f(vertex.position * vertex.scale + vertex.offset, 0.0, 1.0),
	vertex.color,
    );
}

@fragment
fn fragment_main(@location(0) color: vec3f) -> @location(0) vec4f {
    return vec4f(color, 1.0);
}
