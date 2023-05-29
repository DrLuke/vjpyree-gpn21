#import bevy_pbr::mesh_types
#import bevy_pbr::mesh_view_bindings

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
};

@group(1) @binding(0)
var prev_tex: texture_2d<f32>;
@group(1) @binding(1)
var prev_samp: sampler;

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

fn laplace(uv: vec2<f32>) -> vec4<f32>
{
    var out_val = textureSample(prev_tex, prev_samp, uv) * 9.;
    /*out_val -= textureSample(prev_tex, prev_samp, uv + vec2<f32>(-0.0009765625, -0.0009765625)) * 0.04000000000000001;
    out_val -= textureSample(prev_tex, prev_samp, uv + vec2<f32>(-0.0009765625, -0.00048828125)) * 0.16000000000000003;
    out_val -= textureSample(prev_tex, prev_samp, uv + vec2<f32>(-0.0009765625, 0.0)) * 0.2;
    out_val -= textureSample(prev_tex, prev_samp, uv + vec2<f32>(-0.0009765625, 0.00048828125)) * 0.16000000000000003;
    out_val -= textureSample(prev_tex, prev_samp, uv + vec2<f32>(-0.0009765625, 0.0009765625)) * 0.04000000000000001;
    out_val -= textureSample(prev_tex, prev_samp, uv + vec2<f32>(-0.00048828125, -0.0009765625)) * 0.16000000000000003;
    out_val -= textureSample(prev_tex, prev_samp, uv + vec2<f32>(-0.00048828125, -0.00048828125)) * 0.6400000000000001;
    out_val -= textureSample(prev_tex, prev_samp, uv + vec2<f32>(-0.00048828125, 0.0)) * 0.8;
    out_val -= textureSample(prev_tex, prev_samp, uv + vec2<f32>(-0.00048828125, 0.00048828125)) * 0.6400000000000001;
    out_val -= textureSample(prev_tex, prev_samp, uv + vec2<f32>(-0.00048828125, 0.0009765625)) * 0.16000000000000003;
    out_val -= textureSample(prev_tex, prev_samp, uv + vec2<f32>(0.0, -0.0009765625)) * 0.2;
    out_val -= textureSample(prev_tex, prev_samp, uv + vec2<f32>(0.0, -0.00048828125)) * 0.8;
    out_val -= textureSample(prev_tex, prev_samp, uv + vec2<f32>(0.0, 0.0)) * 1.0;
    out_val -= textureSample(prev_tex, prev_samp, uv + vec2<f32>(0.0, 0.00048828125)) * 0.8;
    out_val -= textureSample(prev_tex, prev_samp, uv + vec2<f32>(0.0, 0.0009765625)) * 0.2;
    out_val -= textureSample(prev_tex, prev_samp, uv + vec2<f32>(0.00048828125, -0.0009765625)) * 0.16000000000000003;
    out_val -= textureSample(prev_tex, prev_samp, uv + vec2<f32>(0.00048828125, -0.00048828125)) * 0.6400000000000001;
    out_val -= textureSample(prev_tex, prev_samp, uv + vec2<f32>(0.00048828125, 0.0)) * 0.8;
    out_val -= textureSample(prev_tex, prev_samp, uv + vec2<f32>(0.00048828125, 0.00048828125)) * 0.6400000000000001;
    out_val -= textureSample(prev_tex, prev_samp, uv + vec2<f32>(0.00048828125, 0.0009765625)) * 0.16000000000000003;
    out_val -= textureSample(prev_tex, prev_samp, uv + vec2<f32>(0.0009765625, -0.0009765625)) * 0.04000000000000001;
    out_val -= textureSample(prev_tex, prev_samp, uv + vec2<f32>(0.0009765625, -0.00048828125)) * 0.16000000000000003;
    out_val -= textureSample(prev_tex, prev_samp, uv + vec2<f32>(0.0009765625, 0.0)) * 0.2;
    out_val -= textureSample(prev_tex, prev_samp, uv + vec2<f32>(0.0009765625, 0.00048828125)) * 0.16000000000000003;
    out_val -= textureSample(prev_tex, prev_samp, uv + vec2<f32>(0.0009765625, 0.0009765625)) * 0.04000000000000001;*/

    out_val -= textureSample(prev_tex, prev_samp, uv + vec2<f32>(-0.001953125, -0.001953125)) * 0.04000000000000001;
    out_val -= textureSample(prev_tex, prev_samp, uv + vec2<f32>(-0.001953125, -0.0009765625)) * 0.16000000000000003;
    out_val -= textureSample(prev_tex, prev_samp, uv + vec2<f32>(-0.001953125, 0.0)) * 0.2;
    out_val -= textureSample(prev_tex, prev_samp, uv + vec2<f32>(-0.001953125, 0.0009765625)) * 0.16000000000000003;
    out_val -= textureSample(prev_tex, prev_samp, uv + vec2<f32>(-0.001953125, 0.001953125)) * 0.04000000000000001;
    out_val -= textureSample(prev_tex, prev_samp, uv + vec2<f32>(-0.0009765625, -0.001953125)) * 0.16000000000000003;
    out_val -= textureSample(prev_tex, prev_samp, uv + vec2<f32>(-0.0009765625, -0.0009765625)) * 0.6400000000000001;
    out_val -= textureSample(prev_tex, prev_samp, uv + vec2<f32>(-0.0009765625, 0.0)) * 0.8;
    out_val -= textureSample(prev_tex, prev_samp, uv + vec2<f32>(-0.0009765625, 0.0009765625)) * 0.6400000000000001;
    out_val -= textureSample(prev_tex, prev_samp, uv + vec2<f32>(-0.0009765625, 0.001953125)) * 0.16000000000000003;
    out_val -= textureSample(prev_tex, prev_samp, uv + vec2<f32>(0.0, -0.001953125)) * 0.2;
    out_val -= textureSample(prev_tex, prev_samp, uv + vec2<f32>(0.0, -0.0009765625)) * 0.8;
    out_val -= textureSample(prev_tex, prev_samp, uv + vec2<f32>(0.0, 0.0)) * 1.0;
    out_val -= textureSample(prev_tex, prev_samp, uv + vec2<f32>(0.0, 0.0009765625)) * 0.8;
    out_val -= textureSample(prev_tex, prev_samp, uv + vec2<f32>(0.0, 0.001953125)) * 0.2;
    out_val -= textureSample(prev_tex, prev_samp, uv + vec2<f32>(0.0009765625, -0.001953125)) * 0.16000000000000003;
    out_val -= textureSample(prev_tex, prev_samp, uv + vec2<f32>(0.0009765625, -0.0009765625)) * 0.6400000000000001;
    out_val -= textureSample(prev_tex, prev_samp, uv + vec2<f32>(0.0009765625, 0.0)) * 0.8;
    out_val -= textureSample(prev_tex, prev_samp, uv + vec2<f32>(0.0009765625, 0.0009765625)) * 0.6400000000000001;
    out_val -= textureSample(prev_tex, prev_samp, uv + vec2<f32>(0.0009765625, 0.001953125)) * 0.16000000000000003;
    out_val -= textureSample(prev_tex, prev_samp, uv + vec2<f32>(0.001953125, -0.001953125)) * 0.04000000000000001;
    out_val -= textureSample(prev_tex, prev_samp, uv + vec2<f32>(0.001953125, -0.0009765625)) * 0.16000000000000003;
    out_val -= textureSample(prev_tex, prev_samp, uv + vec2<f32>(0.001953125, 0.0)) * 0.2;
    out_val -= textureSample(prev_tex, prev_samp, uv + vec2<f32>(0.001953125, 0.0009765625)) * 0.16000000000000003;
    out_val -= textureSample(prev_tex, prev_samp, uv + vec2<f32>(0.001953125, 0.001953125)) * 0.04000000000000001;

    return out_val * 0.25;
}

const resolution = vec2<f32>(2048., 2048.);

fn rd1(uv: vec2<f32>) -> vec4<f32>
{
    let prev = textureSample(prev_tex, prev_samp, uv);
    let lap = -laplace(uv);

    let uvf = ((uv - vec2<f32>(0.5))*2.);
    let Da = 1.;
    let Db = 0.2 + sin(length(uvf)*10.) * 0.15;
    let f = 0.0287;
    let k = .078;

    let powfac = 2.;

    let new_stuff = vec2<f32>(
            Da * lap.r - prev.r * prev.g*prev.g + f * clamp(1.0 - prev.r, 0., 1.),
            Db * lap.g + prev.r * prev.g*prev.g - clamp(k, 0., 1.) * prev.g
        ) * 0.7;

    return vec4<f32>(
    prev.r + new_stuff.r,
    prev.g + new_stuff.g,
    lap.r * 30.,
    lap.g * 30.);
}

/*
vec4 rd1(vec2 uv)
{
    vec2 prev = texture(prevtex, uv).rg;
    vec4 lap = -laplace(gl_FragCoord.xy);

    vec2 uvf = ((uv - vec2(0.5))*2);
    uvf *= vec2(res.x/res.y, 1.);
    float Da = 1.;
    float Db = 0.3;
    float f = 0.04 + sin(length(uvf)*10.)*0.015;
    float k = .103 + length(uvf)*0.006 + sin(length(uvf)*10.)*0.015;
    float powfac = 2.0;

    vec2 newCon = clamp(vec2(prev.r, prev.g) + vec2(
        Da * lap.r - prev.r * pow(prev.g, powfac) + f * clamp(1.0 - prev.r, 0., 1.),
        Db * lap.g + prev.r * pow(prev.g, powfac) - clamp(k, 0., 1.) * prev.g
        ) * 0.7,
    0, 1);


    return vec4(newCon, lap.rg*30.);;
}
*/



@fragment
fn fragment(input: VertexOutput) -> @location(0) vec4<f32> {
    let uv = vec2<f32>(input.uv.x, input.uv.y);

    var output_color = rd1(uv);
    //output_color.r = sin(uv.x*100.) * sin(globals.time) * step(uv.y, 0.5);
    //output_color.g += sin(uv.x*100.)*0.01;
    return output_color;
}
