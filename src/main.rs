use nannou::prelude::*;

mod simulation;

pub use crate::simulation::mouse_clicked;

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
    cell_size: f32,
    grid_size: u32,

    density: f32,
    num_x: u32,
    num_y: u32,
    num_cells: u32,
    h: u32,
    u: Vec<f32>,
    v: Vec<f32>,
    new_u: Vec<f32>,
    new_v: Vec<f32>,
    p: Vec<f32>,
    s: Vec<f32>,
    m: Vec<f32>,
    new_m: Vec<f32>,
    num: u32,
}

fn model(app: &App) -> Model {
    let mut cells = Vec::new();
    let grid_size: u32 = 25;
    let cell_size: f32 = 4.0;

    let _window = app
        .new_window()
        .resizable(false)
        .size(512, 512)
        .view(view)
        .build()
        .unwrap();

    for y in 0..grid_size {
        for x in 0..grid_size {
            let pos_x: f32 = (x as f32 - (grid_size / 2) as f32) * cell_size;
            let pos_y: f32 = (y as f32 - (grid_size / 2) as f32) * cell_size;
            let cell = Cell::new(vec2(pos_x, pos_y), 0.0);
            cells.push(cell);
        }
    }

    Model {
        cells,
        cell_size,
        grid_size,

        density: 1000.0,
        num_x: grid_size + 2,
        num_y: grid_size + 2,
        num_cells: (grid_size + 2) * (grid_size + 2),
        h: 0,
        u: vec![0.0; ((grid_size + 2) * (grid_size + 2)) as usize],
        v: vec![0.0; ((grid_size + 2) * (grid_size + 2)) as usize],
        new_u: vec![0.0; ((grid_size + 2) * (grid_size + 2)) as usize],
        new_v: vec![0.0; ((grid_size + 2) * (grid_size + 2)) as usize],
        p: vec![0.0; ((grid_size + 2) * (grid_size + 2)) as usize],
        s: vec![0.0; ((grid_size + 2) * (grid_size + 2)) as usize],
        m: vec![0.0; ((grid_size + 2) * (grid_size + 2)) as usize],
        new_m: vec![1.0; ((grid_size + 2) * (grid_size + 2)) as usize],
        num: grid_size * grid_size,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let _cell_position_opt: Option<(u32, u32)> = simulation::mouse_clicked(app, model);

    let mut counter: usize = 0;
    for cell in model.cells.iter_mut() {
        cell.shade = model.m[counter];
        counter += 1;
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(nannou::color::LIGHTGRAY);

    for cell in model.cells.iter() {
        draw.rect()
            .width(model.cell_size)
            .height(model.cell_size)
            .color(Hsl::new(0.0, 0.0, cell.shade))
            .xy(cell.position);
    }

    draw.to_frame(app, &frame).unwrap();
}
