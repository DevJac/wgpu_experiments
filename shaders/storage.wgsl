struct OurStruct {
    color: vec3f,
    scale: vec2f,
    offset: vec2f,
};

struct VSOut {
    @builtin(position) position: vec4f,
    @location(0) color: vec3f,
};

@group(0) @binding(0) var<storage> our_structs: array<OurStruct>;
@group(0) @binding(1) var<storage> vertices: array<vec2f>;

@vertex
fn vertex_main(
    @builtin(vertex_index) vertex_index: u32,
    @builtin(instance_index) instance_index: u32,
) -> VSOut {
    let our_struct = our_structs[instance_index];
    return VSOut(
	vec4f(vertices[vertex_index] * our_struct.scale + our_struct.offset, 0.0, 1.0),
	our_struct.color, 
    );
}

@fragment
fn fragment_main(@location(0) color: vec3f) -> @location(0) vec4f {
    return vec4f(color, 1.0);
}
