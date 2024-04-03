use nannou::prelude::*;

mod simulation;

pub use crate::simulation::advect_smoke;
pub use crate::simulation::advect_vel;
pub use crate::simulation::extrapolate;
pub use crate::simulation::integrate;
pub use crate::simulation::mouse_clicked;
pub use crate::simulation::setup_scene;
pub use crate::simulation::solve_incompressibility;

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

struct Scene {
    gravity: f32,
    dt: f32,
    num_iters: u32,
    frame_nr: u32,
    over_relaxation: f32,
    obstacle_x: f32,
    obstacle_y: f32,
    obstacle_radius: f32,
    paused: bool,
    scene_nr: u32,
    show_obstacle: bool,
    show_stream_lines: bool,
    show_velocities: bool,
    show_pressure: bool,
    show_smoke: bool,
    initial_run: bool,
}

impl Scene {
    pub fn new(
        gravity: f32,
        dt: f32,
        num_iters: u32,
        frame_nr: u32,
        over_relaxation: f32,
        obstacle_x: f32,
        obstacle_y: f32,
        obstacle_radius: f32,
        paused: bool,
        scene_nr: u32,
        show_obstacle: bool,
        show_stream_lines: bool,
        show_velocities: bool,
        show_pressure: bool,
        show_smoke: bool,
        initial_run: bool,
    ) -> Self {
        Scene {
            gravity,
            dt,
            num_iters,
            frame_nr,
            over_relaxation,
            obstacle_x,
            obstacle_y,
            obstacle_radius,
            paused,
            scene_nr,
            show_obstacle,
            show_stream_lines,
            show_velocities,
            show_pressure,
            show_smoke,
            initial_run,
        }
    }
}

pub struct Model {
    scene: Scene,
    cells: Vec<Cell>,
    cell_size: f32,
    grid_size: u32,

    density: f32,
    num_x: u32,
    num_y: u32,
    num_cells: u32,
    h: f32,
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
    let grid_size: u32 = 50;
    let cell_size: f32 = 4.0;

    let _window = app
        .new_window()
        .resizable(false)
        .size(800, 800)
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

    let scene = Scene::new(
        0.0,
        1.0 / 60.0,
        40,
        0,
        1.9,
        0.0,
        0.0,
        0.15,
        false,
        0,
        false,
        false,
        false,
        false,
        true,
        true,
    );

    Model {
        scene,
        cells,
        cell_size,
        grid_size,

        density: 1000.0,
        num_x: grid_size + 2,
        num_y: grid_size + 2,
        num_cells: (grid_size + 2) * (grid_size + 2),
        h: 1.0 / 100.0,
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
    if model.scene.initial_run {
        simulation::setup_scene(
            model.grid_size,
            model.grid_size,
            model.s.as_mut_slice(),
            model.u.as_mut_slice(),
            model.m.as_mut_slice(),
        );
        model.scene.initial_run = false;
    }

    simulation::integrate(
        model.num_y,
        model.num_x,
        model.s.as_mut_slice(),
        model.v.as_mut_slice(),
        model.scene.dt,
        model.scene.gravity,
    );

    simulation::solve_incompressibility(
        model.num_x,
        model.num_y,
        model.density,
        model.h,
        model.s.as_slice(),
        model.v.as_mut_slice(),
        model.u.as_mut_slice(),
        model.p.as_mut_slice(),
        model.scene.over_relaxation,
        model.scene.num_iters,
        model.scene.dt,
    );

    simulation::extrapolate(
        model.num_x,
        model.num_y,
        model.u.as_mut_slice(),
        model.v.as_mut_slice(),
    );

    simulation::advect_vel(
        model.scene.dt,
        model.new_u.as_mut_slice(),
        model.new_v.as_mut_slice(),
        model.u.as_mut_slice(),
        model.v.as_mut_slice(),
        model.m.as_mut_slice(),
        model.s.as_slice(),
        model.num_x,
        model.num_y,
        model.h,
    );

    simulation::advect_smoke(
        model.scene.dt,
        model.h,
        model.num_x,
        model.num_y,
        model.new_m.as_mut_slice(),
        model.m.as_mut_slice(),
        model.s.as_slice(),
        model.u.as_mut_slice(),
        model.v.as_mut_slice(),
    );

    let mut counter: usize = 0;
    for cell in model.cells.iter_mut() {
        cell.shade = model.m[counter];
        counter += 1;
    }
    //println!("{:?}", model.m);
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
