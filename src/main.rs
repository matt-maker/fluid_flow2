use nannou::prelude::*;

mod simulation;

//pub use crate::simulation::fluid_cube_add_density;
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
    add_density_x: f32,
    add_density_y: f32,

    grid_size: i32,
    dt: f32,
    diff: f32,
    visc: f32,

    s: Vec<f32>,
    density: Vec<f32>,

    vx: Vec<f32>,
    vy: Vec<f32>,
    vz: Vec<f32>,

    vx0: Vec<f32>,
    vy0: Vec<f32>,
    vz0: Vec<f32>,
}

fn model(app: &App) -> Model {
    let _window = app.new_window().size(512, 512).view(view).build().unwrap();

    let mut cells = Vec::new();
    let grid_size: i32 = 100;
    let cell_size: f32 = 3.0;

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
        add_density_x: -1.0,
        add_density_y: -1.0,

        dt: 1.0 / 60.0,
        diff: 3.0,
        visc: 3.0,

        s: vec![0.0; (grid_size * grid_size) as usize],
        density: vec![0.0; (grid_size * grid_size) as usize],

        vx: vec![0.0; (grid_size * grid_size) as usize],
        vy: vec![0.0; (grid_size * grid_size) as usize],
        vz: vec![0.0; (grid_size * grid_size) as usize],

        vx0: vec![0.0; (grid_size * grid_size) as usize],
        vy0: vec![0.0; (grid_size * grid_size) as usize],
        vz0: vec![0.0; (grid_size * grid_size) as usize],
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    simulation::mouse_clicked(app);
    //simulation::fluid_cube_add_density();
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
