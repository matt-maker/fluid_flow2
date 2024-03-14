use crate::Model;
use nannou::prelude::*;

/*pub fn change_shade(app: &App, model: &mut Model, _update: Update) {
    let time = app.elapsed_frames() as f32 / 100.0;

    for cell in model.cells.iter_mut() {
        cell.shade = time - time.floor();
    }
}*/

fn index(x: i32, y: i32, grid_size: i32) -> usize {
    (x + (y * grid_size)) as usize
}

pub fn fluid_cube_add_density(model: &mut Model, x: i32, y: i32, amount: f32) {
    let n = model.grid_size;
    model.density[index(x, y, model.grid_size)] += amount;
}

pub fn fluid_cube_add_velocity() {
    //
}

pub fn mouse_clicked(app: &App) {
    let mouse_button = app.mouse.buttons.left();
    let mouse_position = app.mouse.position();

    if mouse_button.is_down() {
        println!("{}, {}", mouse_position.x, mouse_position.y);
    }
}
