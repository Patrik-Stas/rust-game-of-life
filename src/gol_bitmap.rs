pub struct GolBitmap {
    data: Vec<Vec<bool>>,
    pub cols: usize,
    pub rows: usize,
}

impl GolBitmap {
    pub fn new(cols: usize, rows: usize) -> GolBitmap {
        GolBitmap {
            cols,
            rows,
            data: vec!(vec!(false; rows); cols),
        }
    }

    pub fn init_random(&mut self) {
        for col_index in 1..self.cols {
            for row_index in 1..self.rows {
                if col_index % 3 == 0 || (row_index + 3) % 7 == 0 {
                    self.reincarnate(col_index, row_index)
                }
            }
        }
    }

    pub fn init_from(&mut self, data: &str, aliveChar: Option<char>) {
        let lines = data.split("\n");
        let mut row_i = 0;
        for line in lines {
            let mut col_i = 0;
            // todo: just remove the whitespace
            for token in line.split(' ') {
                let q = token.as_bytes();
                if q[0] == aliveChar.unwrap_or('x') as u8 {
                    self.reincarnate(col_i, row_i)
                }
                col_i += 1;
            }
            row_i += 1;
        }
    }

    pub fn kill_all(&mut self) {
        for col_index in 1..self.cols {
            for row_index in 1..self.rows {
                self.kill(col_index, row_index)
            }
        }
    }

    pub fn reincarnate(&mut self, col: usize, row: usize) {
        self.data[col][row] = true
    }

    pub fn kill(&mut self, col: usize, row: usize) {
        self.data[col][row] = false
    }

    pub fn set_state(&mut self, col: usize, row: usize, state: bool) {
        self.data[col][row] = state
    }

    pub fn is_alive(&self, col: usize, row: usize) -> bool {
        if col >= self.cols {
            return false;
        }
        if row >= self.rows {
            return false;
        }
        self.data[col][row]
    }

    pub fn is_dead(&self, col: usize, row: usize) -> bool {
        !self.is_alive(col, row)
    }

    pub fn has_alive_in_row(&self, row: usize) -> bool {
        for col_index in 1..self.cols {
            if self.data[col_index][row] {
                return true;
            }
        }
        false
    }

    pub fn has_alive_in_col(&self, col: usize) -> bool {
        for &row_item in &self.data[col] {
            if row_item {
                return true;
            }
        }
        false
    }

    pub fn count_neighbours(&self, col: usize, row: usize) -> u8 {
        let mut count = 0;

        // above
        if col > 0 && row > 0 && self.is_alive(col - 1, row - 1) {
            count += 1;
        }
        if col > 0 && row > 0 && self.is_alive(col, row - 1) {
            count += 1;
        }
        if row > 0 && row > 0 && self.is_alive(col + 1, row - 1) {
            count += 1;
        }

        // on sides
        if col > 0 && self.is_alive(col - 1, row) {
            count += 1;
        }
        if self.is_alive(col + 1, row) {
            count += 1;
        }

        // below
        if col > 0 && self.is_alive(col - 1, row + 1) {
            count += 1;
        }
        if self.is_alive(col, row + 1) {
            count += 1;
        }
        if self.is_alive(col + 1, row + 1) {
            count += 1;
        }

        return count;
    }

    pub fn print(&self) {
        for col_index in 1..self.cols {
            for row_index in 1..self.rows {
                if self.is_alive(col_index, row_index) {
                    print!("x ")
                } else {
                    print!("- ")
                }
            }
            println!()
        }
    }
}

#[cfg(test)]
mod tests {
    use std::slice::Split;

    use super::*;

    #[test]
    fn should_correct_construct_bitmap_from_string_1() {
        let mut f = GolBitmap::new(5, 5);
        let fdata = "\
- - - - -
- - x x -
- - - - -
- - - - -";
        f.init_from(fdata, Some('x'));
        assert!(f.is_alive(2, 1));
        assert!(f.is_alive(3, 1));

        assert_eq!(false, f.has_alive_in_col(0));
        assert_eq!(false, f.has_alive_in_col(1));
        assert_eq!(false, f.has_alive_in_col(4));

        assert_eq!(false, f.has_alive_in_row(0));
        assert_eq!(false, f.has_alive_in_row(2));
        assert_eq!(false, f.has_alive_in_row(3));
    }

    #[test]
    fn should_correct_construct_bitmap_from_string_2() {
        let mut f = GolBitmap::new(5, 5);
        let fdata = "\
x - - - x
x - x x -
- x - x -
- - - - x";
        f.init_from(fdata, Some('x'));
        assert!(f.is_alive(0, 0));
        assert!(!f.is_alive(1, 0));
        assert!(!f.is_alive(2, 0));
        assert!(!f.is_alive(3, 0));
        assert!(f.is_alive(4, 0));

        assert!(f.is_alive(0, 1));
        assert!(!f.is_alive(1, 1));
        assert!(f.is_alive(2, 1));
        assert!(f.is_alive(3, 1));
        assert!(!f.is_alive(4, 1));

        assert!(!f.is_alive(0, 2));
        assert!(f.is_alive(1, 2));
        assert!(!f.is_alive(2, 2));
        assert!(f.is_alive(3, 2));
        assert!(!f.is_alive(4, 2));

        assert!(!f.is_alive(0, 3));
        assert!(!f.is_alive(1, 3));
        assert!(!f.is_alive(2, 3));
        assert!(!f.is_alive(3, 3));
        assert!(f.is_alive(4, 3));
    }

    #[test]
    fn should_count_neighbours() {
        let mut f = GolBitmap::new(5, 5);
        let fdata = "\
- - - - -
- - x - x
- x - - x
- - x - -";
        f.init_from(fdata, Some('x'));
        assert_eq!(0, f.count_neighbours(0, 0));
        assert_eq!(2, f.count_neighbours(1, 1));
        assert_eq!(3, f.count_neighbours(2, 2));
        assert_eq!(4, f.count_neighbours(3, 2));

        assert_eq!(1, f.count_neighbours(2, 1));
        assert_eq!(1, f.count_neighbours(0, 2));
        assert_eq!(2, f.count_neighbours(1, 3));
        assert_eq!(1, f.count_neighbours(2, 3));
    }

    #[test]
    fn is_dead_should_be_opposite_of_is_alive() {
        let mut f = GolBitmap::new(5, 5);
        let fdata = "\
x - - - -
- - - - -
- - - - -
- - - - -";
        f.init_from(fdata, Some('x'));
        f.kill(0,0);
        assert_ne!(f.is_dead(0, 0), f.is_alive(0, 0));
    }

    #[test]
    fn should_make_cell_alive() {
        let mut f = GolBitmap::new(5, 5);
        let fdata = "\
- - - - -
- - - - -
- - - - -
- - - - -";
        f.init_from(fdata, Some('x'));
        f.reincarnate(0,0);
        assert!(f.is_alive(0, 0));
    }

    #[test]
    fn should_make_cell_dead() {
        let mut f = GolBitmap::new(5, 5);
        let fdata = "\
x - - - -
- - - - -
- - - - -
- - - - -";
        f.init_from(fdata, Some('x'));
        f.kill(0,0);
        assert!(f.is_dead(0, 0));
    }
}

