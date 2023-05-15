struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
};

@group(1) @binding(0)
var texture_a: texture_2d<f32>;
@group(1) @binding(1)
var our_sampler_a: sampler;
@group(1) @binding(2)
var fractal_t: texture_2d<f32>;
@group(1) @binding(3)
var fractal_s: sampler;


fn rot3(axis: vec3<f32>, angle: f32) -> mat3x3<f32> {
    let an = normalize(axis);
    let s = sin(angle);
    let c = cos(angle);
    let oc = 1.0 - c;

    return mat3x3<f32>(oc * axis.x * axis.x + c, oc * axis.x * axis.y - axis.z * s, oc * axis.z * axis.x + axis.y * s,
    oc * axis.x * axis.y + axis.z * s, oc * axis.y * axis.y + c, oc * axis.y * axis.z - axis.x * s,
    oc * axis.z * axis.x - axis.y * s, oc * axis.y * axis.z + axis.x * s, oc * axis.z * axis.z + c);
}

fn rot2(a: f32) -> mat2x2<f32> {
    return mat2x2<f32>(cos(a), -sin(a), sin(a), cos(a));
}

fn rgb2hsv(c: vec3<f32>) -> vec3<f32> {
    let K = vec4<f32>(0.0, -1.0 / 3.0, 2.0 / 3.0, -1.0);
    let p = mix(vec4<f32>(c.bg, K.wz), vec4<f32>(c.gb, K.xy), step(c.b, c.g));
    let q = mix(vec4<f32>(p.xyw, c.r), vec4<f32>(c.r, p.yzx), step(p.x, c.r));

    let d = q.x - min(q.w, q.y);
    let e = 1.0e-10;
    return vec3<f32>(abs(q.z + (q.w - q.y) / (6.0 * d + e)), d / (q.x + e), q.x);
}

@fragment
fn fragment(input: VertexOutput) -> @location(0) vec4<f32> {
    let uv = vec2<f32>(input.uv.x, input.uv.y);

    // Circle
    let dist = length(uv - vec2<f32>(0.5,0.5));
    let circle = (1.-smoothstep(0.2, 0.205, dist));

    let prev_sample = textureSample(texture_a, our_sampler_a, input.uv);

    let fb_sample = textureSample(texture_a, our_sampler_a, input.uv*2.);

    var output_color = vec4<f32>(circle, fb_sample.b*0.8, (circle*dist*1.) + fb_sample.g*0.8 + fb_sample.r, 1.0);

    output_color = textureSample(fractal_t, fractal_s, uv);

    return output_color;
}
