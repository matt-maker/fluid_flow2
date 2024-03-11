use nannou::prelude::*;

mod simulation;

pub use crate::simulation::change_shade;

const NUMBER_CELLS: usize = 100;
const CELL_SIZE: f32 = 3.0;

fn main() {
    nannou::app(model).update(update).run();
}

struct Cell {
    position: Vec2,
    shade: f32,
}

impl Cell {
    pub fn new(position: Vec2, shade: f32) -> Self {
        Cell { position, shade }
    }
}

pub struct Model {
    cells: Vec<Cell>,
}

fn model(app: &App) -> Model {
    let _window = app.new_window().size(512, 512).view(view).build().unwrap();
    let mut cells = Vec::new();

    for y in 0..NUMBER_CELLS {
        for x in 0..NUMBER_CELLS {
            let pos_x: f32 = (x as f32 - (NUMBER_CELLS / 2) as f32) * CELL_SIZE;
            let pos_y: f32 = (y as f32 - (NUMBER_CELLS / 2) as f32) * CELL_SIZE;
            let cell = Cell::new(vec2(pos_x, pos_y), 0.0);
            cells.push(cell);
        }
    }
    Model { cells }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    simulation::change_shade(app, model, _update);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(nannou::color::LIGHTGRAY);

    for cell in model.cells.iter() {
        draw.rect()
            .width(CELL_SIZE)
            .height(CELL_SIZE)
            .color(Hsl::new(0.0, 0.0, cell.shade))
            .xy(cell.position);
    }

    draw.to_frame(app, &frame).unwrap();
}
