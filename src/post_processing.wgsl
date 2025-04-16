struct VertexOutput {
    @location(0) uv: vec2<f32>,
    @builtin(position) clip_position: vec4<f32>,
};

@vertex
fn vs(
    @builtin(vertex_index) vi: u32,
) -> VertexOutput {
    var out: VertexOutput;
    // Triangle from (0,0) (2,0) (0,1)
    out.uv = vec2<f32>(
        f32((vi << 1u) & 2u),
        f32(vi & 2u),
    );
    out.clip_position = vec4<f32>((out.uv * 2.0) - 1.0, 0.0, 1.0);
    // Flip Y
    out.uv.y = 1.0 - out.uv.y;
    return out;
}

@group(0)
@binding(0)
var pp_image: texture_2d<f32>;

@group(0)
@binding(1)
var pp_sampler: sampler;

@fragment
fn fs(vs: VertexOutput) -> @location(0) vec4<f32> {
    var uv = fish_eye(vs.uv);

    var colour = crt(uv);
    let intensity = dot(colour.rgb, vec3<f32>(0.299, 0.587, 0.114));

    return vec4<f32>(vec3<f32>(intensity), colour.a);
}

const warp: f32 = 0.75;

fn fish_eye(uv_in: vec2<f32>) -> vec2<f32> {
    var uv = uv_in - 0.5;

    var dc: vec2<f32> = abs(uv);
    dc *= dc;

    uv.x *= 1.0+(dc.y*(0.3*warp));
    uv.y *= 1.0+(dc.x*(0.4*warp));

    return uv + 0.5;
}

const thickness: f32 = 0.5;
const density: f32 = 500.0;
const size: f32 = 2.0;
fn crt(uv: vec2<f32>) -> vec4<f32> {
    if (uv.y > 1.0 || uv.x < 0.0 || uv.x > 1.0 || uv.y < 0.0) {
        return vec4<f32>(0.0, 0.0, 0.0, 1.0);
    }

    var mul = clamp((abs(sin(uv.y * density))-sin(thickness))*size, 0.0, 1.0);

    return mul * textureSample(pp_image, pp_sampler, uv);
}
