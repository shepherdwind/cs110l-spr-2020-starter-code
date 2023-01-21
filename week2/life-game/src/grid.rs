use crate::stone::Stone;

// Grid implemented as flat vector
pub struct Grid<T> {
    // y
    pub num_rows: usize,
    // x
    pub num_cols: usize,
    elms: Vec<T>,
}

impl<T> Grid<T> {
    pub fn size(&self) -> (usize, usize) {
        (self.num_rows, self.num_cols)
    }

    /// Returns the element at the specified location. If the location is out of bounds, returns
    /// None.
    ///
    /// Note to students: this function also could have returned Result. It's a matter of taste in
    /// how you define the semantics; many languages raise exceptions for out-of-bounds exceptions,
    /// but others argue that makes code needlessly complex. Here, we decided to return Option to
    /// give you more practice with Option :) and because this similar library returns Option:
    /// https://docs.rs/array2d/0.2.1/array2d/struct.Array2D.html
    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        if row >= self.num_rows || col >= self.num_cols {
            return None
        }

        match self.elms.get(row * self.num_cols + col) {
            Some(v) => Some(v),
            None => None,
        }
    }

    /// Sets the element at the specified location to the specified value. If the location is out
    /// of bounds, returns Err with an error message.
    pub fn set(&mut self, row: usize, col: usize, val: T) -> Result<(), &'static str> {
        if row >= self.num_rows || col >= self.num_cols {
            return Err("Invalid location")
        }
        let _ = &self.elms.insert(row * self.num_cols + col, val);
        Ok(())
        // Be sure to delete the #[allow(unused)] line above
    }

    pub fn walk_mut(&mut self, func: &dyn Fn(&mut T, usize, usize) -> ()) {
        for row in 0..self.num_rows {
            for col in 0..self.num_cols {
                let elms = self.elms.get_mut(row * self.num_cols + col);
                match elms {
                    Some(v) => func(v, row, col),
                    None => (),
                }
            }
        }
    }

    pub fn  get_nearby(&self, row: usize, col: usize) -> Vec<&T> {
        let mut nearby = Vec::new();
        for y in 0..3 {
            for x in 0..3 {
                if col + x == 0 || row + y == 0 {
                    continue;
                }

                let item = self.get(row + y - 1, col + x - 1);
                match item {
                    None => {},
                    Some(v) => {
                        nearby.push(v);
                    }
                }
            }
        }
        nearby
    }

    pub fn walk(&self, func: Box<dyn Fn(&T, usize, usize) -> ()>) {
        for row in 0..self.num_rows {
            for col in 0..self.num_cols {
                let elms = self.get(row, col);
                match elms {
                    Some(v) => func(v, row, col),
                    None => (),
                }
            }
        }
    }
}

impl Grid<Stone> {
    pub fn new(num_rows: usize, num_cols: usize) -> Grid<Stone> {
        let mut elms = Vec::new();

        for y in 0..num_rows {
            for x in 0..num_cols {
                let val = Stone::new(x as f32, y as f32);
                elms.insert(y * num_cols + x, val);
            }
        }

        Grid {
            num_rows,
            num_cols,
            elms,
        }
    }
}

impl Grid<usize> {
    pub fn new(num_rows: usize, num_cols: usize) -> Grid<usize> {
        Grid {
            num_rows,
            num_cols,
            elms: vec![0, num_rows * num_cols],
        }
    }

    pub fn display(&self) {
        self.walk(Box::new(move |v, row, col|{
            if col == 0 && row != 0 {
                println!("");
            }
            print!("{} ", v);
        }));
        println!("");
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_grid() {
        let n_rows = 4;
        let n_cols = 3;
        let mut grid = Grid::<usize>::new(n_rows, n_cols);

        // Initialize grid
        for r in 0..n_rows {
            for c in 0..n_cols {
                assert!(
                    grid.set(r, c, r * n_cols + c).is_ok(),
                    "Grid::set returned Err even though the provided bounds are valid!"
                );
            }
        }

        // Note: you need to run "cargo test  -- --nocapture" in order to see output printed
        println!("Grid contents:");
        grid.display();

        // Make sure the values are what we expect
        for r in 0..n_rows {
            for c in 0..n_cols {
                assert!(
                    grid.get(r, c).is_some(),
                    "Grid::get returned None even though the provided bounds are valid!"
                );
                assert_eq!(*grid.get(r, c).unwrap(), r * n_cols + c);
            }
        }

        assert_eq!(grid.get_nearby(0, 0), vec![&0, &1, &3, &4]);
        assert_eq!(grid.get_nearby(0, 1), vec![&0, &1, &2, &3, &4, &5]);
        assert_eq!(grid.get_nearby(0, 2), vec![&1, &2, &4, &5]);
        assert_eq!(grid.get_nearby(1, 0), vec![&0, &1, &3, &4, &6, &7]);
    }
}