#import bevy_pbr::mesh_types
#import bevy_pbr::mesh_view_bindings

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
};

struct MyUniforms{
    glitch_offset: f32,
    glitch_pixelation: f32,
    glitch_abberration: f32,
}

@group(1) @binding(0)
var texture_a: texture_2d<f32>;
@group(1) @binding(1)
var our_sampler_a: sampler;
@group(1) @binding(2)
var<uniform> my_uniforms: f32;
@group(1) @binding(5)
var noise_tex: texture_2d<f32>;
@group(1) @binding(6)
var noise_tex_sampler: sampler;

fn quantize(a: f32, steps: f32) -> f32 {
    return round(a * steps)/steps;
}

@fragment
fn fragment(input: VertexOutput) -> @location(0) vec4<f32> {
    let uv = vec2<f32>(input.uv.x, input.uv.y);
    let uvc = vec2<f32>((uv.x*2. - 1.), (uv.y*2. - 1.));
    let uvca = vec2<f32>(uvc.x * 16./9., uvc.y);

    var noise_coarse = textureSample(noise_tex, noise_tex_sampler, vec2<f32>(globals.time*0.001 % 1., quantize(uv.y, 24.)));
    var noise_fine = textureSample(noise_tex, noise_tex_sampler, vec2<f32>(quantize(globals.time*0.03 % 1., 512.), quantize(uv.y, 128.)));

    var samp_uv = vec2<f32>(
        uv.x,
        uv.y
    );

    // Offset effect
    // TODO: add control for strength
    let offset = noise_coarse.r*0.04 + noise_fine.b*0.02;
    samp_uv.x += offset * 1.;

    // Pixelation
    // TODO: add control in steps of pow^2
    samp_uv = vec2<f32>(
        quantize(samp_uv.x, 128.),
        quantize(samp_uv.y, 256.)
    );

    //var output_color = textureSample(texture_a, our_sampler_a, vec2<f32>(quantize(uv.x + offset, 128.) , quantize(uv.y, 128.)));
    var output_color = textureSample(texture_a, our_sampler_a, samp_uv);

    // Chromatic aberration
    // TODO: add control for strength
    let deflection = 0.02;
    var red_samp = textureSample(texture_a, our_sampler_a, samp_uv + vec2<f32>(-deflection, 0.));
    var blue_samp = textureSample(texture_a, our_sampler_a, samp_uv + vec2<f32>(deflection, 0.));

    output_color.r = red_samp.r;
    output_color.b = blue_samp.b;

    return output_color;
}
