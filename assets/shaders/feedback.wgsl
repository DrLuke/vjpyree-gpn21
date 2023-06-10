#import bevy_pbr::mesh_types
#import bevy_pbr::mesh_view_bindings

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
};

struct UniformParams {
    p0: f32,
    p1: f32,
    p2: f32,
    p3: f32,
    p4: f32,
    p5: f32,
    p6: f32,
    p7: f32,
}

struct Beat {
    beat: f32,
    pt1: f32,
    accum: f32,
    accumpt1: f32,
}

struct Settings {
    palette: f32,
    mirror_x: f32,
    uv_scale: f32,
}

@group(1) @binding(0)
var prev_t: texture_2d<f32>;
@group(1) @binding(1)
var prev_s: sampler;
@group(1) @binding(2)
var fractal_t: texture_2d<f32>;
@group(1) @binding(3)
var fractal_s: sampler;
@group(1) @binding(4)
var rd_t: texture_2d<f32>;
@group(1) @binding(5)
var rd_s: sampler;
@group(1) @binding(6)
var<uniform> col_rot: vec4<f32>;
@group(1) @binding(7)
var<uniform> rand: UniformParams;
@group(1) @binding(8)
var<uniform> randpt1: UniformParams;
@group(1) @binding(9)
var<uniform> beat: Beat;
@group(1) @binding(10)
var<uniform> settings: Settings;

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

// Scale UV coordinates around the point (0.5, 0.5)
fn uvcscale(uv: vec2<f32>, scale: f32) -> vec2<f32> {
   return (uv - vec2<f32>(0.5)) * scale + vec2<f32>(0.5);
}

fn uvcrot(uv: vec2<f32>, angle: f32) -> vec2<f32> {
    return (uv - vec2<f32>(0.5)) * rot2(angle) + vec2<f32>(0.5);
}

fn uvcarot(uv: vec2<f32>, angle: f32) -> vec2<f32> {
    let res = vec2<f32>(1920., 610.);
    let aspect = res.x/res.y;
    return (((uv - vec2<f32>(0.5)) * vec2<f32>(aspect, 1.) ) * rot2(angle) + vec2<f32>(0.5)) * vec2<f32>(1./aspect, 1.);
}

// Color palette by Inigo Quilezles https://iquilezles.org/articles/palettes/
fn palette(t: f32, a: vec3<f32>, b: vec3<f32>, c: vec3<f32>, d: vec3<f32>) -> vec3<f32> {
    return a + b*cos( 6.28318*(c*t+d) );
}

fn palette1(t: f32) -> vec3<f32> {
    return palette(t, vec3<f32>(0.5), vec3<f32>(0.5), vec3<f32>(1.), vec3<f32>(0.00, 0.33, 0.67));
}

fn palette2(t: f32) -> vec3<f32> {
    return palette(t, vec3<f32>(0.8, 0.5, 0.4), vec3<f32>(0.2, 0.4, 0.2), vec3<f32>(2.0, 1.0, 1.0), vec3<f32>(0.00, 0.25, 0.25));
}

fn palette3(t: f32) -> vec3<f32> {
    return palette(t, vec3<f32>(0.5), vec3<f32>(0.5), vec3<f32>(2.0, 1.0, 0.0), vec3<f32>(0.50, 0.20, 0.25));
}

fn palette4(t: f32) -> vec3<f32> {
    return palette(t, vec3<f32>(0.5), vec3<f32>(0.5), vec3<f32>(1.), vec3<f32>(0.00, 0.10, 0.20));
}

fn palette5(t: f32) -> vec3<f32> {
    return palette(t, vec3<f32>(0.5), vec3<f32>(0.5), vec3<f32>(1.0, 1.0, 0.5), vec3<f32>(0.80, 0.90, 0.30	));
}

fn palette6(t: f32) -> vec3<f32> {
    // FREESTYLE
    return palette(t, vec3<f32>(0.5), vec3<f32>(0.5), vec3<f32>(2.0, 1.0, 0.0), vec3<f32>(0.50, 0.20, 0.25));
}

fn pal(palette: f32, t: f32) -> vec3<f32> {
    if (palette <= 0.) {
        return palette1(t);
    }
    if (palette <= 1.) {
        return palette2(t);
    }
    if (palette <= 2.) {
        return palette3(t);
    }
    if (palette <= 3.) {
        return palette4(t);
    }
    if (palette <= 4.) {
        return palette5(t);
    }
    if (palette <= 5.) {
        return palette6(t);
    }
    return palette1(t);
}


@fragment
fn fragment(input: VertexOutput) -> @location(0) vec4<f32> {
    let res = vec2<f32>(1920., 610.);
    let aspect = res.x/res.y;
    let uv = vec2<f32>(input.uv.x, input.uv.y);
    let uva = vec2<f32>((input.uv.x - 0.5) * aspect + 0.5, input.uv.y);
    let uv11a = vec2<f32>(input.uv.x - 0.5, input.uv.y - 0.5)*2. * vec2<f32>(aspect, 1.);

    // Prev
    let prev_sample = textureSample(prev_t, prev_s, input.uv); // 1:1 sample
    let prev_hsv = rgb2hsv(prev_sample.rgb);

    // Feedback sampler effects
    var hsv_angle = prev_hsv.x * 3.14159 * 4. + atan2(uv11a.y, uv11a.x)*1.;
    var sample_offset = vec2<f32>(cos(hsv_angle), sin(hsv_angle)) * 0.001 * randpt1.p5;
    var fb_uv = uvcscale(uvcrot(input.uv, rand.p6 * 0.01 / length(uv11a)), settings.uv_scale) - sample_offset;
    var fb_sample = textureSample(prev_t, prev_s, fb_uv);

    // Output
    var output_color = vec4<f32>(0.);

    // RD sample
    var rd_uv = uva;
    if ( settings.mirror_x > 0.) {
        rd_uv.x = abs(rd_uv.x - 0.5) + 0.5;
    }
    rd_uv = uvcscale(rd_uv, beat.pt1*rand.p0 + 1.);
    rd_uv = uvcrot(rd_uv, globals.time*0.1 + randpt1.p4 + beat.accumpt1 * rand.p1);

    var rd_sample = textureSample(rd_t, rd_s, rd_uv);
    var rd_strength = rd_sample.x;
    var mask = 1.-smoothstep(rd_sample.y, 0.3, 0.5);
    output_color = vec4<f32>(pal(settings.palette, rd_strength + globals.time * 0.1 + length(uv11a)), 1.);

    fb_sample = vec4<f32>(fb_sample.rgb*rot3(col_rot.xyz, col_rot.w), fb_sample.a);
    output_color = output_color * mask * 0.9 + fb_sample * (1.-mask) * 0.99;

    return output_color;
}
