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
    veloc_x: &mut [f32],
    veloc_y: &mut [f32],
    p: &mut [f32],
    div: &mut [f32],
    iter: i32,
    n: u32,
) {
    for j in 1..(n - 1) {
        for i in 1..(n - 1) {
            div[index(i, j, n)] = -0.5
                * (veloc_x[index(i + 1, j, n)] - veloc_x[index(i - 1, j, n)]
                    + veloc_y[index(i, j + 1, n)]
                    - veloc_y[index(i, j - 1, n)])
                / n as f32;
            p[index(i, j, n)] = 0.0;
        }
    }
    set_bnd(0, div, n);
    set_bnd(0, p, n);
    lin_solve(0, p, div, 1.0, 6.0, iter, n);

    for j in 1..(n - 1) {
        for i in 1..(n - 1) {
            veloc_x[index(i, j, n)] -=
                0.5 * (p[index(i + 1, j, n)] - p[index(i - 1, j, n)]) * n as f32;
            veloc_y[index(i, j, n)] -=
                0.5 * (p[index(i, j + 1, n)] - p[index(i, j - 1, n)]) * n as f32;
        }
    }
    set_bnd(1, veloc_x, n);
    set_bnd(2, veloc_y, n);
}

pub fn advect(
    b: u32,
    d: &mut [f32],
    d0: &[f32],
    veloc_x: &[f32],
    veloc_y: &[f32],
    dt: f32,
    n: u32,
) {
    let dtx = dt * (n - 2) as f32;
    let dty = dt * (n - 2) as f32;

    let nfloat: f32 = n as f32;
    let (mut ifloat, mut jfloat): (f32, f32) = (0.0, 0.0);

    for j in 1..(n - 1) {
        jfloat += 1.0;
        for i in 1..(n - 1) {
            ifloat += 1.0;
            let tmp1 = dtx * veloc_x[index(i, j, n)];
            let tmp2 = dty * veloc_y[index(i, j, n)];
            let mut x = ifloat - tmp1;
            let mut y = jfloat - tmp2;

            if x < 0.5 {
                x = 0.5
            };
            if x > (nfloat + 0.5) {
                x = nfloat + 0.5
            };
            let i0 = x.floor();
            let i1 = i0 + 1.0;
            if y < 0.5 {
                y = 0.5
            };
            if y > (nfloat + 0.5) {
                y = nfloat + 0.5
            };
            let j0 = y.floor();
            let j1 = j0 + 1.0;

            let s1 = x - i0;
            let s0 = 1.0 - s1;
            let t1 = y - j0;
            let t0 = 1.0 - t1;

            let i0i: u32 = i0 as u32;
            let i1i: u32 = i1 as u32;
            let j0i: u32 = j0 as u32;
            let j1i: u32 = j1 as u32;

            d[index(i, j, n)] = s0
                * ((t0 * d0[index(i0i, j0i, n)]) + (t1 * d0[index(i0i, j1i, n)]))
                + s1 * ((t0 * d0[index(i1i, j0i, n)]) + (t1 * d0[index(i1i, j1i, n)]));
        }
    }
    set_bnd(b, d, n);
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
