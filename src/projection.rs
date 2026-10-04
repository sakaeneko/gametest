pub fn world_to_screen(
    world: (f32, f32, f32),
    m: &[[f32; 4]; 4],
    sw: f32,
    sh: f32,
) -> Option<(f32, f32)> {
    let w = m[0][3] * world.0 + m[1][3] * world.1
          + m[2][3] * world.2 + m[3][3];
    if w < 0.001 { return None; }

    let x = m[0][0] * world.0 + m[1][0] * world.1
          + m[2][0] * world.2 + m[3][0];
    let y = m[0][1] * world.0 + m[1][1] * world.1
          + m[2][1] * world.2 + m[3][1];

    let ndc_x = x / w;
    let ndc_y = y / w;

    Some((
        (sw * 0.5) * (1.0 + ndc_x),
        (sh * 0.5) * (1.0 - ndc_y),
    ))
}
