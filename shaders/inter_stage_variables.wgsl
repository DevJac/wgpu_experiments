struct PositionAndColor {
    @builtin(position) position: vec4f,
    @location(0) color: vec4f,
}

fn pos(i: u32) -> PositionAndColor {
    if (i == 0u) {
	return PositionAndColor(vec4f(0.0, 0.5, 0.0, 1.0), vec4f(1.0, 0.0, 0.0, 1.0));
    }
    if (i == 1u) {
	return PositionAndColor(vec4f(-0.5, -0.5, 0.0, 1.0), vec4f(0.0, 1.0, 0.0, 1.0));
    }
    return PositionAndColor(vec4f(0.5, -0.5, 0.0, 1.0), vec4f(0.0, 0.0, 1.0, 1.0));
}

@vertex
fn vertex_main(@builtin(vertex_index) vertex_index: u32) -> PositionAndColor {
    return pos(vertex_index);
}

@fragment
fn fragment_main(pc: PositionAndColor) -> @location(0) vec4f {
    let grid = vec2u(pc.position.xy) / 8u;
    let checker = (grid.x + grid.y) % 2u == 1u;
    return select(pc.color, vec4f(0.0, 0.0, 0.0, 0.0), checker);
}

// The following also works. Note receiving @location(0) specifically.
// @fragment
// fn fragment_main(@location(0) color: vec4f) -> @location(0) vec4f {
//     return color;
// }
