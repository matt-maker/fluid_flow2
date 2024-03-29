use crate::Model;
use nannou::prelude::*;

pub fn integrate(num_y: u32, num_x: u32, s: &[f32], v: &mut [f32], dt: f32, gravity: f32) {
    let n = num_y;
    for i in 1..num_x {
        for j in 1..num_y - 1 {
            if s[(i * n + j) as usize] != 0.0 && s[(i * n - j - 1) as usize] != 0.0 {
                v[(i * n + j) as usize] += gravity * dt;
            }
        }
    }
}

pub fn solve_incompressibility(
    num_x: u32,
    num_y: u32,
    density: f32,
    h: f32,
    s: &[f32],
    v: &mut [f32],
    u: &mut [f32],
    p: &mut [f32],
    over_relaxation: f32,
    num_iters: u32,
    dt: f32,
) {
    let n = num_y;
    let cp = density * h / dt;

    for _ in 0..num_iters {
        for i in 1..num_x - 1 {
            for j in 1..num_y - 1 {
                if s[(i * n + j) as usize] == 0.0 {
                    continue;
                }
                let mut s_value = s[(i * n + j) as usize];
                let sx0 = s[((i - 1) * n + j) as usize];
                let sx1 = s[((i + 1) * n + j) as usize];
                let sy0 = s[(i * n + j - 1) as usize];
                let sy1 = s[(i * n - j + 1) as usize];
                s_value = sx0 + sx1 + sy0 + sy1;
                if s_value == 0.0 {
                    continue;
                }
                let div = u[((i + 1) * n + j) as usize] - u[(i * n + j) as usize]
                    + v[(i * n + j + 1) as usize]
                    - v[(i * n + j) as usize];
                let mut p_value = -div / s_value;
                p_value *= over_relaxation;
                p[(i * n + j) as usize] += cp * p_value;

                u[(i * n + j) as usize] -= sx0 * p_value;
                u[((i + 1) * n + j) as usize] += sx1 * p_value;
                v[(i * n + j) as usize] -= sy0 * p_value;
                v[(i * n + j + 1) as usize] += sy1 * p_value;
            }
        }
    }
}

pub fn extrapolate(num_x: u32, num_y: u32, u: &mut [f32], v: &mut [f32]) {
    let n = num_y;
    for i in 0..num_x {
        u[(i * n + 0) as usize] = u[(i * n + 1) as usize];
        u[(i * n + num_y - 1) as usize] = u[(i * n + num_y - 2) as usize];
    }
    for j in 0..num_y {
        v[(0 * n + j) as usize] = v[(1 * n + j) as usize];
        v[((num_x - 1) * n + j) as usize] = v[((num_x - 2) * n + j) as usize];
    }
}

fn sample_field(num_y: u32, num_x: u32, h: f32, x: f32, y: f32, field: &str, u: &mut [f32]) {
    let n = num_y;
    let h = h;
    let h1 = 1.0 / h;
    let h2 = 0.5 * h;
    let mut x_value = x.min(num_x as f32 * h).max(h);
    let mut y_value = y.min(num_y as f32 * h).max(h);

    let dx = 0.0;
    let dy = 0.0;

    let f: &mut [f32];

    match field {
        U_FIELD => f = u,
    }
}

pub fn mouse_clicked(app: &App, model: &Model) -> Option<(u32, u32)> {
    let window_size: i32 = model.grid_size as i32 * model.cell_size as i32;
    let mouse_position_x =
        ((app.mouse.position().x as i32 + window_size / 2) / model.cell_size as i32) as u32;
    let mouse_position_y =
        ((app.mouse.position().y as i32 + window_size / 2) / model.cell_size as i32) as u32;

    if app.mouse.buttons.left().is_down()
        && mouse_position_x < (window_size / model.cell_size as i32) as u32
        && mouse_position_y < (window_size / model.cell_size as i32) as u32
    {
        Some((mouse_position_x, mouse_position_y))
    } else {
        None
    }
}
