use crate::{conway::Conway, grid::Grid};
use raylib::{
    ffi::{CSSPalette as _, Color, Rectangle, Vector2},
    prelude::{RaylibDraw, RaylibDrawHandle},
};

#[derive(Default, Debug)]
pub struct Viewport {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub cols: u32,
    pub rows: u32,
    pub cell_size: Vector2,
}

impl Viewport {
    pub fn new(x: i32, y: i32, width: i32, height: i32, cols: u32, rows: u32) -> Self {
        Self {
            x,
            y,
            width,
            height,
            rows,
            cols,
            cell_size: Vector2::new(width as f32 / cols as f32, height as f32 / rows as f32),
        }
    }

    pub fn grid_to_world(&self, grid_pos: Vector2) -> Vector2 {
        Vector2::new(grid_pos.x * self.cell_size.x, grid_pos.y * self.cell_size.y)
    }

    pub fn world_to_cell(&self, world_pos: Vector2) -> Option<Vector2> {
        let local_x = world_pos.x - self.x as f32;
        let local_y = world_pos.y - self.y as f32;

        if local_x < 0.0
            || local_x >= self.width as f32
            || local_y < 0.0
            || local_y >= self.height as f32
        {
            return None;
        }

        let x = (local_x / self.cell_size.x).floor();
        let y = (local_y / self.cell_size.y).floor();

        Some(Vector2::new(x, y))
    }

    #[allow(dead_code)]
    pub fn is_within_bounds(&self, pos: Vector2) -> bool {
        pos.x >= self.x as f32
            && pos.x <= (self.x + self.width) as f32
            && pos.y >= self.y as f32
            && pos.y <= (self.y + self.height) as f32
    }
}

#[derive(Debug)]
pub struct Renderer {
    pub viewport: Viewport,
}

impl Renderer {
    pub fn new(viewport: Viewport) -> Self {
        Self { viewport }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle, conway: &mut Conway) {
        self.draw_cells(d, conway.grid());
        self.draw_grid_borders(d);
    }

    fn draw_cells(&self, d: &mut RaylibDrawHandle, grid: &Grid) {
        for cell in grid.iter() {
            let color = if cell.alive {
                Color::WHITE
            } else {
                Color::BLACK
            };

            let pos = self.viewport.grid_to_world(cell.vector2());

            d.draw_rectangle(
                pos.x as i32,
                pos.y as i32,
                self.viewport.cell_size.x as i32 + 1,
                self.viewport.cell_size.y as i32 + 1,
                color,
            );
        }
    }

    fn draw_grid_borders(&self, d: &mut RaylibDrawHandle) {
        let thickness = 1.0;
        let line_color = Color::GREY; //Color::new(80, 80, 80, 255);
        let width = self.viewport.cols as f32 * self.viewport.cell_size.x;
        let height = self.viewport.rows as f32 * self.viewport.cell_size.y;

        d.draw_rectangle_lines_ex(
            Rectangle {
                x: 0.0,
                y: 0.0,
                width,
                height,
            },
            thickness,
            line_color,
        );

        for col in 0..=self.viewport.cols {
            let x_pos = col as f32 * self.viewport.cell_size.x;
            let start = Vector2::new(self.viewport.x as f32 + x_pos, self.viewport.y as f32);
            let end = Vector2::new(x_pos, height);
            d.draw_line_ex(start, end, thickness, line_color);
        }
        for row in 0..=self.viewport.rows {
            let y_pos = row as f32 * self.viewport.cell_size.y;
            let start = Vector2::new(0.0, self.viewport.y as f32 + y_pos);
            let end = Vector2::new(width, y_pos);
            d.draw_line_ex(start, end, thickness, line_color);
        }
    }
}
