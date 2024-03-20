use crate::Model;
use nannou::prelude::*;

/*pub fn change_shade(app: &App, model: &mut Model, _update: Update) {
    let time = app.elapsed_frames() as f32 / 100.0;

    for cell in model.cells.iter_mut() {
        cell.shade = time - time.floor();
    }
}*/

fn index(x: u32, y: u32, n: u32) -> usize {
    (x + (y * n)) as usize
}

pub fn fluid_cube_add_density(n: u32, density: &mut [f32], x: u32, y: u32, amount: f32) {
    density[index(x, y, n)] += amount;
}

pub fn fluid_cube_add_velocity(
    n: u32,
    vx: &mut [f32],
    vy: &mut [f32],
    x: u32,
    y: u32,
    amountx: f32,
    amounty: f32,
) {
    let index = index(x, y, n);
    vx[index] += amountx;
    vy[index] += amounty;
}

pub fn set_bnd(b: u32, x: &mut [f32], n: u32) {
    for i in 1..(n - 1) {
        if b == 2 {
            x[index(i, 0, n)] = x[index(i, 1, n)] * -1.0
        } else {
            x[index(i, 0, n)] = x[index(i, 1, n)]
        }
        if b == 2 {
            x[index(i, n - 1, n)] = x[index(i, n - 2, n)] * -1.0
        } else {
            x[index(i, n - 1, n)] = x[index(i, n - 2, n)]
        }
    }

    for j in 1..(n - 1) {
        if b == 1 {
            x[index(0, j, n)] = x[index(1, j, n)] * -1.0
        } else {
            x[index(0, j, n)] = x[index(1, j, n)]
        }
        if b == 1 {
            x[index(n - 1, j, n)] = x[index(n - 2, j, n)] * -1.0
        } else {
            x[index(n - 1, j, n)] = x[index(n - 2, j, n)]
        }
    }

    x[index(0, 0, n)] = 0.33 * (x[index(1, 0, n)] + x[index(0, 1, n)] + x[index(0, 0, n)]);

    x[index(0, n - 1, n)] =
        0.33 * (x[index(1, n - 1, n)] + x[index(0, n - 2, n)] + x[index(0, n - 1, n)]);

    x[index(n - 1, 0, n)] =
        0.33 * (x[index(n - 2, 0, n)] + x[index(n - 1, 1, n)] + x[index(n - 1, 0, n)]);

    x[index(n - 1, n - 1, n)] =
        0.33 * (x[index(n - 2, n - 1, n)] + x[index(n - 1, n - 2, n)] + x[index(n - 1, n - 1, n)]);
}

pub fn lin_solve(b: u32, x: &mut [f32], x0: &[f32], a: f32, c: f32, iter: i32, n: u32) {
    let c_recip: f32 = 1.0 / c;

    for _ in 0..iter {
        for j in 1..(n - 1) as u32 {
            for i in 1..(n - 1) as u32 {
                x[index(i, j, n)] = (x0[index(i, j, n)]
                    + a * (x[index(i + 1, j, n)]
                        + x[index(i - 1, j, n)]
                        + x[index(i, j + 1, n)]
                        + x[index(i, j - 1, n)]))
                    * c_recip;
            }
        }
    }
    set_bnd(b, x, n)
}

pub fn diffuse(b: u32, x: &mut [f32], x0: &[f32], diff: f32, dt: f32, iter: i32, n: u32) {
    let a: f32 = dt * diff * (n - 2) as f32 * (n - 2) as f32;
    lin_solve(b, x, x0, a, 1.0 + 6.0 * a, iter, n);
}

pub fn project(
    model: &Model,
    veloc_x: Vec<f32>,
    veloc_y: Vec<f32>,
    p: Vec<f32>,
    div: Vec<f32>,
    iter: i32,
    n: u32,
) {
    //
}

pub fn advect(
    model: &Model,
    b: i32,
    d: Vec<f32>,
    d0: Vec<f32>,
    veloc_x: Vec<f32>,
    veloc_y: Vec<f32>,
    dt: f32,
    n: u32,
) {
    //
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
