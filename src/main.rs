use std::io;
use rayon::prelude::*;
use bit_vec::BitVec;

// 44000^2 = 1 936 000 000 cells, 13.06 seconds for initialization, 2.47 seconds for a generationupdate
const GRID_WIDTH: usize = 100000; // 100000 works fine
const GRID_HEIGHT: usize = 100000; // 100000 works fine

// O(n*m) time complexity, O(n*m) space complexity increase of grid by 10x --> approx 10x time increase

type Grid = Vec<BitVec>;

fn main() {
    let mut current_generation = initialize_grid_with_random_values();
    let mut next_generation = vec![BitVec::from_elem(GRID_WIDTH, false); GRID_HEIGHT];

    loop {
        display_current_generation(&current_generation);

        println!("Press Enter for the next generation, or type 'q' to quit:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if input.trim().eq_ignore_ascii_case("q") {
            break;
        }

        update_generation(&current_generation, &mut next_generation);
        std::mem::swap(&mut current_generation, &mut next_generation);
    }
}

fn initialize_grid_with_random_values() -> Grid {
    let mut grid = vec![BitVec::from_elem(GRID_WIDTH, false); GRID_HEIGHT];

    for row in grid.iter_mut() {
        for col in 0..GRID_WIDTH {
            row.set(col, rand::random::<bool>());
        }
    }

    grid
}

fn display_current_generation(_grid: &Grid) {
    // Due to the size of the grid, the display function has been commented out.
    // Uncomment to display the grid in the terminal, but be aware that this may take a long time.

    // for row in grid.iter() {
    //     for cell in row.iter() {
    //         print!("{}", if *cell { 'X' } else { ' ' });
    //     }
    //     println!();
    // }
}

fn update_generation(current_generation: &Grid, next_generation: &mut Grid) {
    next_generation.par_iter_mut().enumerate().for_each(|(row, next_row)| {
        for col in 0..GRID_WIDTH {
            let live_neighbors = count_live_neighbors(current_generation, row, col);
            let next_state = current_generation[row][col] && (live_neighbors == 2 || live_neighbors == 3) || !current_generation[row][col] && live_neighbors == 3;
            next_row.set(col, next_state);
        }
    });
}

fn count_live_neighbors(current_generation: &Grid, row: usize, col: usize) -> u32 {
    let mut live_neighbors = 0;

    for row_offset in -1..=1 {
        for col_offset in -1..=1 {
            if row_offset == 0 && col_offset == 0 {
                continue;
            }

            let new_row = (row as isize + row_offset).rem_euclid(GRID_HEIGHT as isize) as usize;
            let new_col = (col as isize + col_offset).rem_euclid(GRID_WIDTH as isize) as usize;

            if current_generation[new_row][new_col] {
                live_neighbors += 1;
            }
        }
    }

    live_neighbors
}
