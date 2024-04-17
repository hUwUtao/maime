//#import bevy_sprite::mesh2d_vertex_output::VertexOutput

//we can import items from shader modules in the assets folder with a quoted path
//#import __string__0__endstring__::COLOR_MULTIPLIER

//import is hostile to formatter, so we temporary copy it here
struct VertexOutput {
    //this is `clip position` when the struct is used as a vertex stage output
    //and `frag coord` when used as a fragment stage input
    @builtin(position) position: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
    #ifdef VERTEX_TANGENTS
        @location(3) world_tangent: vec4<f32>,
    #endif
    #ifdef VERTEX_COLORS
        @location(4) color: vec4<f32>,
    #endif
}

@group(2) @binding(0) var<uniform> material_color : vec4<f32>;
@group(2) @binding(1) var material_color_texture : texture_2d<f32>;
@group(2) @binding(2) var material_color_sampler : sampler;

fn alpha_over(bot: vec4<f32>, top: vec4<f32>) -> vec4<f32> {
    let atop = top.a;
    let abot = bot.a * (1.0 - atop);
    return vec4<f32>(
        (top.rgb * atop + bot.rgb * abot) / (atop + abot),
        atop + abot
    );
}

// centerize as it should be
const w = 2.5;
fn decartes_uv(uv: vec2<f32>) -> vec2<f32> {
    return vec2((uv[0] * w * 2.0) - w, (uv[1] * w * 2.0) - w);
}

// a mask for eye, using parabola range. fascinating
const o = 4.0;
const a = 1 / o;
fn eye_mask(uv: vec2<f32>) -> f32 {
    return f32(uv[1] < (-1.0 * a) * (uv[0] * uv[0]) + 1.0 && uv[1] > a * (uv[0] * uv[0]) - 1.0);
}
const sm = (1.0 / 128.0);
fn eye_feather(uv: vec2<f32>) -> f32 {
    return smoothstep(sm, 0.0, min(
        abs((-1.0 * a) * (uv[0] * uv[0]) + 1.0 - uv[1]),
        abs(a * (uv[0] * uv[0]) - 1.0 - uv[1])
    )) * f32(uv[0] * uv[0] < o + sm * 0.5);
}

fn eye(uv: vec2<f32>) -> f32 {
    return clamp(eye_feather(uv) + eye_mask(uv), 0.0, 1.0);
}

// assume center, its good
const r = 1.0;
fn circle(uv: vec2<f32>) -> f32 {
    return smoothstep(r + sm, r, length(uv));
}

@fragment
fn fragment(
    in: VertexOutput,
) -> @location(0) vec4<f32> {
    let uv = decartes_uv(in.uv);

    // return material_color * vec4f(1.0, 1.0, 1.0, textureSample(material_color_texture, material_color_sampler, mesh.uv)[0]);
    // return material_color * (y > (((-1.0 * a) * (x * x)) + 1.0) && y < ((a * (y * y)) - 1.0));
    return alpha_over(vec4(0.0, 0.0, 0.0, eye(uv)), vec4(1.0, 1.0, 1.0, circle(uv)));
    // return vec4(uv[0], uv[1], 0.0, 1.0);
}
