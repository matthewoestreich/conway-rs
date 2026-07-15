use crate::grid::Grid;
use raylib::ffi::Vector2;

pub struct Conway {
    update_interval: f32,
    timer: f32,
    grid: Grid,
}

impl Conway {
    pub fn new(update_interval: f32, grid: Grid) -> Self {
        Self {
            grid,
            update_interval,
            timer: update_interval,
        }
    }

    pub fn clicked(&mut self, position: Vector2) {
        if let Some(cell) = self.grid.get_cell_mut(position.x as u32, position.y as u32) {
            cell.alive = !cell.alive;
        };
    }

    pub fn grid(&mut self) -> &Grid {
        &self.grid
    }

    pub fn update(&mut self, delta_time: f32) {
        self.timer -= delta_time;

        if self.timer > 0.0 {
            return;
        }

        let mut next_grid = self.grid.clone();

        next_grid.iter_mut().for_each(|cell| {
            let alive_neighbors = self.grid.alive_neighbors_len(cell.vector2());

            if cell.alive {
                cell.alive = (2..=3).contains(&alive_neighbors);
            } else if alive_neighbors == 3 {
                cell.alive = true;
            }
        });

        self.grid = next_grid;
        self.timer = self.update_interval;
    }
}
