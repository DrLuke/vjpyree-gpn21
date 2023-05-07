struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
};

struct MyUniforms{
    some_val: f32
}

@group(1) @binding(2)
var<uniform> my_uniforms: MyUniforms;
@group(1) @binding(0)
var texture_a: texture_2d<f32>;
@group(1) @binding(1)
var our_sampler_a: sampler;

@fragment
fn fragment(input: VertexOutput) -> @location(0) vec4<f32> {
    let uv = vec2<f32>(input.uv.x, 1.-input.uv.y);

    // Circle
    let dist = length(uv - vec2<f32>(0.5,0.5));
    let circle = (1.-smoothstep(0.2, 0.205, dist))*my_uniforms.some_val;

    let prev_sample = textureSample(texture_a, our_sampler_a, input.uv);

    let fb_sample = textureSample(texture_a, our_sampler_a, input.uv*2.);

    let output_color = vec4<f32>(circle, fb_sample.b*0.8, (circle*dist*1.) + fb_sample.g*0.8 + fb_sample.r, 1.0);
    return output_color;
}
