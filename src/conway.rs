use crate::grid::Grid;

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

    pub fn clicked_coords(&mut self, x: f32, y: f32) {
        if let Some(cell) = self.grid.get_cell_mut(x as u32, y as u32) {
            cell.alive = !cell.alive;
        };
    }

    pub fn grid_mut(&mut self) -> &mut Grid {
        &mut self.grid
    }

    pub fn update(&mut self, delta_time: f32) {
        self.timer -= delta_time;

        if self.timer > 0.0 {
            return;
        }

        let mut next = self.grid.clone();

        next.iter_mut().for_each(|cell| {
            let alive_neighbors = self.grid.alive_neighbors_len(cell.vector2());

            if cell.alive {
                cell.alive = (2..=3).contains(&alive_neighbors);
            } else if alive_neighbors == 3 {
                cell.alive = true;
            }
        });

        self.grid = next;
        self.timer = self.update_interval;
    }
}
