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

fn sample_field(
    num_y: u32,
    num_x: u32,
    h: f32,
    x: f32,
    y: f32,
    field: &str,
    u: &mut [f32],
    v: &mut [f32],
    m: &mut [f32],
) -> Option<f32> {
    let n = num_y;
    let h = h;
    let h1 = 1.0 / h;
    let h2 = 0.5 * h;
    let x = x.min(num_x as f32 * h).max(h);
    let y = y.min(num_y as f32 * h).max(h);

    let mut dx = 0.0;
    let mut dy = 0.0;
    let f: &mut [f32];
    let field_char = field.chars().nth(0).unwrap();

    match field_char {
        'U' => {
            f = u;
            dy = h2;
        }
        'V' => {
            f = v;
            dx = h2;
        }
        'S' => {
            f = m;
            dx = h2;
            dy = h2;
        }
        _ => {
            panic!();
        }
    }

    let x0 = ((x - dx) * h1).floor().min(num_x as f32 - 1.0);
    let tx = ((x - dx) - x0 * h) * h1;
    let x1 = (x0 + 1.0).min(num_x as f32 - 1.0);

    let y0 = ((y - dy) * h1).floor().min(num_y as f32 - 1.0);
    let ty = ((y - dy) - y0 * h) * h1;
    let y1 = (y0 + 1.0).min(num_y as f32 - 1.0);

    let sx = 1.0 - tx;
    let sy = 1.0 - ty;

    let val: f32 = sx * sy * f[(x0 * n as f32 + y0) as usize]
        + tx * sy * f[(x1 * n as f32 + y1) as usize]
        + tx * ty * f[(x1 * n as f32 + y1) as usize]
        + sx * ty * f[(x0 * n as f32 + y1) as usize];
    return Some(val);
}

fn avg_u(num_y: u32, u: &[f32], i: u32, j: u32) -> Option<f32> {
    let n = num_y;
    let u_value = (u[(i * n + j - 1) as usize]
        + u[(i * n + j) as usize]
        + u[((i + 1) * n + j - 1) as usize]
        + u[((i + 1) * n + j) as usize])
        * 0.25;
    return Some(u_value);
}

fn avg_v(num_y: u32, v: &[f32], i: u32, j: u32) -> Option<f32> {
    let n = num_y;
    let v_value = (v[((i - 1) * n + j) as usize]
        + v[(i * n + j) as usize]
        + v[((i - 1) * n + j + 1) as usize]
        + v[(i * n + j + 1) as usize])
        * 0.25;
    return Some(v_value);
}

pub fn advect_vel(
    dt: f32,
    new_u: &mut [f32],
    new_v: &mut [f32],
    u: &mut [f32],
    v: &mut [f32],
    m: &mut [f32],
    s: &[f32],
    num_x: u32,
    num_y: u32,
    h: f32,
) {
    new_u.copy_from_slice(u);
    new_v.copy_from_slice(v);

    let n = num_y;
    let h = h;
    let h2 = 0.5 * h;

    for i in 1..num_x {
        for j in 1..num_y {
            //cnt += 1;

            // u
            if s[(i * n + j) as usize] != 0.0
                && s[((i - 1) * n + j) as usize] != 0.0
                && j < num_y - 1
            {
                let mut x = i as f32 * h;
                let mut y = j as f32 * h + h2;
                let mut u_value = u[(i * n + j) as usize];
                let v_value = avg_v(num_y, v, i, j).unwrap();
                x = x - dt * u_value;
                y = y - dt * v_value;
                u_value = sample_field(num_y, num_x, h, x, y, "U_FIELD", u, v, m).unwrap();
                new_u[(i * n + j) as usize] = u_value;
            }

            //v
            if s[(i * n + j) as usize] != 0.0 && s[(i * n + j - 1) as usize] != 0.0 && i < num_x - 1
            {
                let mut x = i as f32 * h + h2;
                let mut y = j as f32 * h;
                let u_value = avg_u(num_y, u, i, j).unwrap();
                let mut v_value = v[(i * n + j) as usize];
                x = x - dt * u_value;
                y = y - dt * v_value;
                v_value = sample_field(num_y, num_x, h, x, y, "V_FIELD", u, v, m).unwrap();
                new_v[(i * n + j) as usize] = v_value;
            }
        }
    }
    u.copy_from_slice(new_u);
    v.copy_from_slice(new_v);
}

pub fn advect_smoke(
    dt: f32,
    h: f32,
    num_x: u32,
    num_y: u32,
    new_m: &mut [f32],
    m: &mut [f32],
    s: &[f32],
    u: &mut [f32],
    v: &mut [f32],
) {
    new_m.copy_from_slice(m);

    let n = num_y;
    let h = h;
    let h2 = 0.5 * h;

    for i in 1..(num_x - 1) {
        for j in 1..(num_y - 1) {
            if s[(i * n + j) as usize] != 0.0 {
                let u_value = u[(i * n + j) as usize] + u[((i + 1) * n + j) as usize] * 0.5;
                let v_value = v[(i * n + j) as usize] + v[(i * n + (j + 1)) as usize] * 0.5;
                let x = i as f32 * h + h2 - dt * u_value;
                let y = j as f32 * h + h2 - dt * v_value;

                new_m[(i * n + j) as usize] =
                    sample_field(num_y, num_x, h, x, y, "S_FIELD", u, v, m).unwrap();
            }
        }
    }
    m.copy_from_slice(new_m);
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
