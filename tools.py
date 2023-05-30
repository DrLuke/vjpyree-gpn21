def unroll_laplace_baked_offset():
    resolution = [1024, 1024]
    weights = [0.2, 0.8, 1.0, 0.8, 0.2]
    for i in range(5):
        for j in range(5):
            weight = weights[i] * weights[j]
            offset = (float(i-2)/resolution[0], float(j-2)/resolution[1])
            print(f"out_val -= textureSample(prev_tex, prev_samp, uv + vec2<f32>({offset[0]}, {offset[1]})) * {weight};")

def unroll_laplace_calculated_offset():
    resolution = 1024
    weights = [0.2, 0.8, 1.0, 0.8, 0.2]
    print(f"\tvar resolution = {resolution:.1f};")
    for i in range(5):
        for j in range(5):
            weight = weights[i] * weights[j]
            offset = (i-2, j-2)
            print(f"\tout_val -= textureSample(prev_tex, prev_samp, uv + vec2<f32>({offset[0]:.1f}/resolution, {offset[1]:.1f}/resolution)) * {weight:.2f};")

unroll_laplace_calculated_offset()
