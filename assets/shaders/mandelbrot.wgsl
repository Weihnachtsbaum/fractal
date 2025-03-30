#import bevy_sprite::mesh2d_vertex_output::VertexOutput

const offset = vec2(-0.75, -0.5);
const scale = 2.0;
const bound = 2.0;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let c = (mesh.uv + offset) * scale;
    var z = vec2(0.0);
    for (var i = 0; i <= 1000; i++) {
        z = f(z, c);
        if length(z) > bound {
            let speed = f32(i) / 50.0;
            return vec4(speed, speed, speed, 1.0);
        }
    }
    return vec4(0.0, 0.0, 0.0, 1.0);
}

fn f(z: vec2<f32>, c: vec2<f32>) -> vec2<f32> {
    return square_complex(z) + c;
}

fn square_complex(z: vec2<f32>) -> vec2<f32> {
    return vec2(z.x * z.x - z.y * z.y, 2.0 * z.x * z.y);
}
