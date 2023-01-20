use crate::grid::Grid;
use crate::stone::Stone;

pub struct Model {
  x: f32,
  y: f32,
  width: usize,
  click: bool,
  pub grid: Grid<Stone>,
}
pub const MARGIN: u32 = 10;

impl Model {
    pub fn new(rows: usize, cols: usize, width: usize) -> Self {
        let grid = Grid::new(rows, cols);
        Model {
          grid,
          x: 0.0,
          y: 0.0,
          width,
          click: false,
        }
    }

    pub fn next_step(&mut self) {
      if !self.click {
        return;
      }

      self.click = false;
      self.grid.walk_mut(&|stone, row, col| {
        let x = (col * self.width) as f32;
        let y = (row * self.width) as f32;
        let w = self.width as f32;

        if self.x >= x && self.x <= x + w && self.y >= y && self.y <= y + w {
          stone.active = !stone.active;
        }
      });
    }

    pub fn set_click(&mut self, x: f32, y: f32) {
      self.click = true;
      // center point is 0, 0, here we need to convert to the top left point
      self.x = x + (self.width * self.grid.num_cols) as f32 * 0.5;
      self.y = (self.width * self.grid.num_rows) as f32 * 0.5 - y;
    }

    pub fn clear(&mut self) {
      self.grid.walk_mut(&|stone, _, _| {
        stone.active = false;
      });
    }
}
