//! This file implements the game of life engine.
//! step_simulation takes the boolean grid as input and returns a new updated grid, which can then be rendered to update the window.

// TODO: Refactor the isize usize logic, it looks ugly.

#![allow(dead_code)]
pub(crate) fn step_simulation(grid: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut new_grid = vec![vec![true; grid[0].len()]; grid.len()];

    // Go over the old grid, and fill in our new grid with the info.
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            let num_alive_neighbors = get_num_alive_neighbors(&grid, (i as isize, j as isize));
            new_grid[i][j] = will_i_be_alive(num_alive_neighbors, grid[i][j]);
        }
    }

    new_grid
}

fn get_num_alive_neighbors(grid: &Vec<Vec<bool>>, index: (isize, isize)) -> u8 {
    let (row_idx, col_idx) = index;
    if row_idx < 0
        || row_idx > grid.len() as isize
        || col_idx < 0
        || col_idx > grid[row_idx as usize].len() as isize
    {
        panic!(
            "Indices ({}, {}) are not valid for the given grid!",
            row_idx, col_idx
        );
    }

    let mut num_alive_neighbors: u8 = 0;

    let up_left_neighbor = (row_idx - 1, col_idx - 1);
    let up_neighbor = (row_idx - 1, col_idx);
    let up_right_neighbor = (row_idx - 1, col_idx + 1);
    let right_neighbor = (row_idx, col_idx + 1);
    let down_right_neighbor = (row_idx + 1, col_idx + 1);
    let down_neighbor = (row_idx + 1, col_idx);
    let down_left_neighbor = (row_idx + 1, col_idx - 1);
    let left_neighbor = (row_idx, col_idx - 1);

    let neighbors = [
        up_left_neighbor,
        up_neighbor,
        up_right_neighbor,
        right_neighbor,
        down_right_neighbor,
        down_neighbor,
        down_left_neighbor,
        left_neighbor,
    ];

    for neighbor in neighbors {
        let (row_idx, col_idx) = neighbor;

        if !(row_idx < 0
            || row_idx >= grid.len() as isize
            || col_idx < 0
            || col_idx >= grid[row_idx as usize].len() as isize)
        {
            // This means the position of this neighbor is valid in our grid.
            if grid[row_idx as usize][col_idx as usize] {
                num_alive_neighbors += 1;
            }
        }
    }

    num_alive_neighbors
}

fn will_i_be_alive(num_alive_neighbors: u8, alive_right_now: bool) -> bool {
    if alive_right_now {
        if num_alive_neighbors < 2 || num_alive_neighbors > 3 {
            // We die by starvation or overpopulation.
            return false;
        }
        // We live on.
        true
    } else {
        if num_alive_neighbors == 3 {
            // We are resurrected.
            true
        } else {
            // We remain dead.
            false
        }
    }
}
