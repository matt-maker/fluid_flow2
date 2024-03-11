use crate::Model;
use nannou::prelude::*;

pub fn change_shade(app: &App, model: &mut Model, _update: Update) {
    let time = app.elapsed_frames() as f32 / 100.0;

    for cell in model.cells.iter_mut() {
        cell.shade = time - time.floor();
    }
}
