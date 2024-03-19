use crate::Model;
use nannou::prelude::*;

/*pub fn change_shade(app: &App, model: &mut Model, _update: Update) {
    let time = app.elapsed_frames() as f32 / 100.0;

    for cell in model.cells.iter_mut() {
        cell.shade = time - time.floor();
    }
}*/

fn index(x: u32, y: u32, grid_size: u32) -> usize {
    (x + (y * grid_size)) as usize
}

pub fn fluid_cube_add_density(model: &mut Model, x: u32, y: u32, amount: f32) {
    model.density[index(x, y, model.grid_size)] += amount;
}

pub fn fluid_cube_add_velocity(model: &mut Model, x: u32, y: u32, amountx: f32, amounty: f32) {
    let index = index(x, y, model.grid_size);
    model.vx[index] += amountx;
    model.vy[index] += amounty;
}

pub fn set_bnd(model: &Model, b: u32, x: &mut Vec<f32>, n: u32) {
    for i in 1..(n - 1) {
        if b == 2 {
            x[index(i, 0, model.grid_size)] = x[index(i, 1, model.grid_size)] * -1.0
        } else {
            x[index(i, 0, model.grid_size)] = x[index(i, 1, model.grid_size)]
        }
        if b == 2 {
            x[index(i, n - 1, model.grid_size)] = x[index(i, n - 2, model.grid_size)] * -1.0
        } else {
            x[index(i, n - 1, model.grid_size)] = x[index(i, n - 2, model.grid_size)]
        }
    }

    for j in 1..(n - 1) {
        if b == 1 {
            x[index(0, j, model.grid_size)] = x[index(1, j, model.grid_size)] * -1.0
        } else {
            x[index(0, j, model.grid_size)] = x[index(1, j, model.grid_size)]
        }
        if b == 1 {
            x[index(n - 1, j, model.grid_size)] = x[index(n - 2, j, model.grid_size)] * -1.0
        } else {
            x[index(n - 1, j, model.grid_size)] = x[index(n - 2, j, model.grid_size)]
        }
    }

    x[index(0, 0, model.grid_size)] = 0.33
        * (x[index(1, 0, model.grid_size)]
            + x[index(0, 1, model.grid_size)]
            + x[index(0, 0, model.grid_size)]);

    x[index(0, n - 1, model.grid_size)] = 0.33
        * (x[index(1, n - 1, model.grid_size)]
            + x[index(0, n - 2, model.grid_size)]
            + x[index(0, n - 1, model.grid_size)]);

    x[index(n - 1, 0, model.grid_size)] = 0.33
        * (x[index(n - 2, 0, model.grid_size)]
            + x[index(n - 1, 1, model.grid_size)]
            + x[index(n - 1, 0, model.grid_size)]);

    x[index(n - 1, n - 1, model.grid_size)] = 0.33
        * (x[index(n - 2, n - 1, model.grid_size)]
            + x[index(n - 1, n - 2, model.grid_size)]
            + x[index(n - 1, n - 1, model.grid_size)]);
}

pub fn lin_solve(
    model: &Model,
    b: u32,
    x: &mut Vec<f32>,
    x0: Vec<f32>,
    a: f32,
    c: f32,
    iter: i32,
    n: u32,
) {
    let c_recip: f32 = 1.0 / c;

    for _ in 0..iter {
        for j in 1..(n - 1) as u32 {
            for i in 1..(n - 1) as u32 {
                x[index(i, j, model.grid_size)] = (x0[index(i, j, model.grid_size)]
                    + a * (x[index(i + 1, j, model.grid_size)]
                        + x[index(i - 1, j, model.grid_size)]
                        + x[index(i, j + 1, model.grid_size)]
                        + x[index(i, j - 1, model.grid_size)]))
                    * c_recip;
            }
        }
    }
    set_bnd(model, b, x, n)
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
