fn _naga_inverse_2x2_f16(m: mat2x2<f16>) -> mat2x2<f16> {
    var adj: mat2x2<f16>;
    adj[0][0] = m[1][1];
    adj[0][1] = -m[0][1];
    adj[1][0] = -m[1][0];
    adj[1][1] = m[0][0];

    let det: f16 = m[0][0] * m[1][1] - m[1][0] * m[0][1];
    return adj * (1 / det);
}