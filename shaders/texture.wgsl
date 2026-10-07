// ==== VERTEX SHADER ======
struct AffineTransform {
    col0: vec2<f32>,
    col1: vec2<f32>,
    translation: vec2<f32>,
}
struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) uv_coords: vec2<f32>,
    @location(2) model_col0: vec4<f32>,
    @location(3) model_col1: vec4<f32>,
    @location(4) model_col2: vec4<f32>,
    @location(5) model_col3: vec4<f32>
};
struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

@group(0) @binding(0)
var<uniform> camera: mat4x4<f32>;

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    let model_matrix = mat4x4<f32>(model.model_col0, model.model_col1, model.model_col2, model.model_col3);
    var out: VertexOutput;
    let world_pos = model_matrix * vec4<f32>(model.position, 1.0);
    out.clip_position = camera * world_pos;
    out.uv = model.uv_coords;
    return out;
}

// ==== FRAGMENT SHADER ======
// fn srgb_to_linear(c: vec3<f32>) -> vec3<f32> {
//     return pow(c, vec3<f32>(2.2));
// }

@group(1) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(1) @binding(1)
var s_diffuse: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    //no srgb conversion? maybe? who knows
    //let linear = srgb_to_linear(in.color);
    //return vec4(linear, 1.0);
    let texture_color = textureSample(t_diffuse, s_diffuse, in.uv);
    // return vec4(texture_color, 1.0);
    // return texture_color
    //let converted_color = vec4<f32>(srgb_to_linear(texture_color.xyz), texture_color.w);
    return texture_color;
}
