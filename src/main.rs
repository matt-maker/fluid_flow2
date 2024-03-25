use nannou::prelude::*;

mod simulation;

pub use crate::simulation::advect;
pub use crate::simulation::diffuse;
pub use crate::simulation::fluid_cube_add_density;
pub use crate::simulation::fluid_cube_add_velocity;
pub use crate::simulation::mouse_clicked;
pub use crate::simulation::project;

pub const AMOUNT_DENSITY: f32 = 0.03;
pub const AMOUNT_X: f32 = 3.0;
pub const AMOUNT_Y: f32 = 3.0;

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
    dt: f32,
    diff: f32,
    visc: f32,

    s: Vec<f32>,
    density: Vec<f32>,

    vx: Vec<f32>,
    vy: Vec<f32>,

    vx0: Vec<f32>,
    vy0: Vec<f32>,
}

fn model(app: &App) -> Model {
    let mut cells = Vec::new();
    let grid_size: u32 = 60;
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

        dt: 1.0 / 60.0,
        diff: 3.0,
        visc: 3.0,

        s: vec![0.0; (grid_size * grid_size) as usize],
        density: vec![0.0; (grid_size * grid_size) as usize],

        vx: vec![0.0; (grid_size * grid_size) as usize],
        vy: vec![0.0; (grid_size * grid_size) as usize],

        vx0: vec![0.0; (grid_size * grid_size) as usize],
        vy0: vec![0.0; (grid_size * grid_size) as usize],
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let cell_position_opt: Option<(u32, u32)> = simulation::mouse_clicked(app, model);

    if cell_position_opt.is_some() {
        let cell_position = cell_position_opt.unwrap();
        simulation::fluid_cube_add_density(
            model.grid_size,
            model.density.as_mut_slice(),
            cell_position.0,
            cell_position.1,
            AMOUNT_DENSITY,
        );
        simulation::fluid_cube_add_velocity(
            model.grid_size,
            model.vx.as_mut_slice(),
            model.vy.as_mut_slice(),
            cell_position.0,
            cell_position.1,
            AMOUNT_X,
            AMOUNT_Y,
        );
        simulation::diffuse(
            1,
            &mut model.vx0.as_mut_slice(),
            model.vx.as_slice(),
            model.visc,
            model.dt,
            4,
            model.grid_size,
        );
        simulation::project(
            &mut model.vx0.as_mut_slice(),
            &mut model.vy0.as_mut_slice(),
            &mut model.vx.as_mut_slice(),
            &mut model.vy.as_mut_slice(),
            4,
            model.grid_size,
        );
        simulation::advect(
            1,
            model.vx.as_mut_slice(),
            model.vx0.clone().as_mut_slice(),
            model.vx0.as_mut_slice(),
            model.vy0.as_mut_slice(),
            model.dt,
            model.grid_size,
        )
    }

    // Add all remaining functions here
    // eg Diffuse, Project, Advect, Project

    let mut counter: usize = 0;
    for cell in model.cells.iter_mut() {
        cell.shade = model.density[counter];
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
