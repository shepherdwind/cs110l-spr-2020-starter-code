use std::time::SystemTime;

use crate::grid::Grid;
use crate::stone::Stone;

pub struct Model {
  x: f32,
  y: f32,
  width: usize,
  click: bool,
  pub step: usize,
  pub grid: Grid<Stone>,
  run_time: Option<SystemTime>,
}
pub const MARGIN: u32 = 10;
const DURATION: i32 = 500;

impl Model {
    pub fn new(rows: usize, cols: usize, width: usize) -> Self {
        let grid = Grid::<Stone>::new(rows, cols);
        Model {
          grid,
          x: 0.0,
          y: 0.0,
          step: 0,
          run_time: None,
          width,
          click: false,
        }
    }

    pub fn next_step(&mut self) {
      if !self.click {
        self.auto_run();
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

    pub fn auto_run(&mut self) {
      if self.run_time == None {
        return;
      }

      let now = SystemTime::now();
      let diff = now.duration_since(self.run_time.unwrap()).unwrap().as_millis();
      if diff > DURATION.try_into().unwrap() {
        self.next_status();
        self.run_time = Some(now);
      }
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
      self.step = 0;
      self.stop();
    }

    pub fn start(&mut self) {
      self.run_time = Some(SystemTime::now());
    }

    pub fn stop(&mut self) {
      self.run_time = None;
    }

    pub fn next_status(&mut self) {
      let mut grid = Grid::<Stone>::new(self.grid.num_rows, self.grid.num_cols);
      self.step += 1;

      grid.walk_mut(&|stone, row, col| {
        let mut live_num = 0;

        for item in self.grid.get_nearby(row, col) {
          if item.active {
            live_num += 1;
          }
        }

        let mut current = self.grid.get(row, col).unwrap().active;
        if current && live_num > 4 {
          current = false;
        } else if current && live_num < 3 {
          current = false;
        } else if !current && live_num == 3 {
          current = true;
        }

        stone.active = current;
      });

      self.grid = grid;
    }
}
