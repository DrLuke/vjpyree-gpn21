#import bevy_pbr::mesh_types
#import bevy_pbr::mesh_view_bindings

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
};

struct JuliaC{
    re: f32,
    im: f32,
}


@group(1) @binding(0)
var texture_a: texture_2d<f32>;
@group(1) @binding(1)
var our_sampler_a: sampler;
@group(1) @binding(2)
var texture_b: texture_2d<f32>;
@group(1) @binding(3)
var our_sampler_b: sampler;
@group(1) @binding(4)
var<uniform> julia_c: JuliaC;

struct Complex {
    re: f32,
    im: f32,
}

fn mandelbrot(Zn: Complex, C: Complex) -> Complex  {
    return Complex(Zn.re * Zn.re - Zn.im * Zn.im + C.re, 2. * Zn.re * Zn.im + C.im);
}

fn trap(Z: Complex) -> vec4<f32> {
    let uv = vec2<f32>(Z.re, Z.im);
    let samp = textureSample(texture_b, our_sampler_b, uv);
    return vec4<f32>(samp.r, samp.g, samp.b, length(samp.rgb));
}

fn rot3(axis: vec3<f32>, angle: f32) -> mat3x3<f32> {
    let an = normalize(axis);
    let s = sin(angle);
    let c = cos(angle);
    let oc = 1.0 - c;

    return mat3x3<f32>(oc * axis.x * axis.x + c, oc * axis.x * axis.y - axis.z * s, oc * axis.z * axis.x + axis.y * s,
    oc * axis.x * axis.y + axis.z * s, oc * axis.y * axis.y + c, oc * axis.y * axis.z - axis.x * s,
    oc * axis.z * axis.x - axis.y * s, oc * axis.y * axis.z + axis.x * s, oc * axis.z * axis.z + c);
}

@fragment
fn fragment(input: VertexOutput) -> @location(0) vec4<f32> {
    let uv = vec2<f32>(input.uv.x, input.uv.y);
    let uvc = vec2<f32>((uv.x*2. - 1.), (uv.y*2. - 1.));
    let uvca = vec2<f32>(uvc.x * 16./9., uvc.y);

    //let origin = Complex(4., 2.);
    let origin = Complex(0., 0.);
    //let origin = Complex(-0.25, 0.);
    //let scale = Complex(6., 4.);
    let scale = 1./1.;

    var Z = Complex(uvca.x*scale + origin.re, uvca.y*scale + origin.im);
    var C = Complex(julia_c.re + sin(globals.time)*0.1, julia_c.im + cos(globals.time*0.8902)*0.1);

    var output_color = vec4<f32>(0., 0., 0., 0.0);

    for(var i: i32 = 0; i < 100; i++) {
        Z = mandelbrot(Z, C);

        let t = trap(Z);
        if(t.w > 0.01) {
            let col = vec3<f32>(t.r, t.g, t.b) * rot3(vec3<f32>(1., 0.4, 1.), f32(i)*0. + globals.time);
            output_color = vec4<f32>(col, 1.) / (f32(i)/1. + 1.);

            break;
        }
    }

    return output_color;
}
