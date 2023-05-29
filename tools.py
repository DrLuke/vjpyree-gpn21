def unroll_laplace():
    resolution = [1024, 1024]
    weights = [0.2, 0.8, 1.0, 0.8, 0.2]
    for i in range(5):
        for j in range(5):
            weight = weights[i] * weights[j]
            offset = (float(i-2)/resolution[0], float(j-2)/resolution[1])
            print(f"out_val -= textureSample(prev_tex, prev_samp, uv + vec2<f32>({offset[0]}, {offset[1]})) * {weight};")

unroll_laplace()
