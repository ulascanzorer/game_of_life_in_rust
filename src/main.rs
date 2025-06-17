#[path = "engine.rs"]
mod engine;

#[path = "renderer.rs"]
mod renderer;

use rand::{rng, Rng};

fn main() -> eframe::Result<()> {
    let mut rng = rng();

    // TODO: Allow the user to set these manually.
    let num_rows = 100;
    let num_columns = 100;

    let mut grid = vec![vec![true; num_columns]; num_rows];
    
    // TODO: Allow the user to initialize the cells in some way.
    
    // Let's fill the grid with random booleans for fun.
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            grid[i][j] = rng.random();
        }
    }

    renderer::run_gui(grid)
}