use raylib::ffi::Vector2;

/* ------------------------------------------------------------------------------ */
/* ---------------- Grid -------------------------------------------------------- */
/* ------------------------------------------------------------------------------ */

#[derive(Debug, Clone)]
pub struct Grid {
    pub cols: u32,
    pub rows: u32,
    cells: Vec<Cell>,
}

impl<'a> IntoIterator for &'a mut Grid {
    type Item = &'a mut Cell;
    type IntoIter = std::slice::IterMut<'a, Cell>;

    fn into_iter(self) -> Self::IntoIter {
        self.cells.iter_mut()
    }
}

impl Grid {
    pub fn new(cols: u32, rows: u32) -> Self {
        let size = (cols * rows) as usize;
        let mut cells = Vec::with_capacity(size);

        for y in 0..rows {
            for x in 0..cols {
                cells.push(Cell::new_with_coords(x, y));
            }
        }

        Self { cols, rows, cells }
    }

    pub fn len(&self) -> usize {
        self.cells.len()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Cell> {
        self.cells.iter_mut()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Cell> {
        self.cells.iter()
    }

    pub fn world_to_cell(&self, position: Vector2) -> (u32, u32) {
        let x = (position.x.floor() as i32).clamp(0, self.cols as i32 - 1);
        let y = (position.y.floor() as i32).clamp(0, self.rows as i32 - 1);

        (x as u32, y as u32)
    }

    pub fn get_cell(&self, x: u32, y: u32) -> Option<&Cell> {
        if !self.is_within_grid_bounds(x, y) {
            return None;
        }
        Some(&self.cells[self.index(x, y)])
    }

    pub fn get_cell_mut(&mut self, x: u32, y: u32) -> Option<&mut Cell> {
        if !self.is_within_grid_bounds(x, y) {
            return None;
        }
        let i = self.index(x, y);
        Some(&mut self.cells[i])
    }

    pub fn cell_center(&self, x: u32, y: u32) -> Vector2 {
        Vector2::new(x as f32 + 0.5, y as f32 + 0.5)
    }

    pub fn is_within_grid_bounds(&self, x: u32, y: u32) -> bool {
        x < self.cols && y < self.rows
    }

    pub fn alive_neighbors_len(&self, position: Vector2) -> u32 {
        let pos_x = position.x as isize;
        let pos_y = position.y as isize;

        let mut alive_neighbors = 0;

        for y in (pos_y - 1)..=(pos_y + 1) {
            for x in (pos_x - 1)..=(pos_x + 1) {
                if y == pos_y && x == pos_x {
                    continue; // Don't count ourself
                }

                if let Some(cell) = self.get_cell(x as u32, y as u32)
                    && cell.alive
                {
                    alive_neighbors += 1;
                }
            }
        }

        alive_neighbors
    }

    fn index(&self, x: u32, y: u32) -> usize {
        (y * self.cols + x) as usize
    }
}

/* ------------------------------------------------------------------------------ */
/* ---------------- Cell -------------------------------------------------------- */
/* ------------------------------------------------------------------------------ */

#[derive(Default, Debug, Clone, Copy)]
pub struct Cell {
    pub alive: bool,
    pub x: u32,
    pub y: u32,
}

impl Cell {
    pub fn new() -> Self {
        Self { ..Self::default() }
    }

    pub fn new_with_coords(x: u32, y: u32) -> Self {
        Self { x, y, alive: false }
    }

    pub fn vector2(&self) -> Vector2 {
        Vector2::new(self.x as f32, self.y as f32)
    }
}
