struct OurStruct {
    color: vec3f,
    scale: vec2f,
    offset: vec2f,
}

@group(0) @binding(0) var<uniform> our_struct: OurStruct;

fn pos(i: u32) -> vec2f {
    if (i == 0u) {
	return vec2f(0.0, 0.5);
    }
    if (i == 1u) {
	return vec2f(-0.5, -0.5);
    }
    return vec2f(0.5, -0.5);
}

@vertex
fn vertex_main(@builtin(vertex_index) vertex_index: u32) -> @builtin(position) vec4f {
    return vec4f(pos(vertex_index) * our_struct.scale + our_struct.offset, 0.0, 1.0);
}

@fragment
fn fragment_main() -> @location(0) vec4f {
    return vec4f(our_struct.color, 1.0);
}
