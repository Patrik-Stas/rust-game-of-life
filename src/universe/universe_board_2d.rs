use rand::Rng;
use std::fs;
use crate::common::Point;
use crate::universe::universe::CellUniverse;

pub struct Board2D {
    data: Vec<Vec<bool>>,
    pub cols: usize,
    pub rows: usize,
}

pub struct IteratorBoard2D<'a> {
    board: &'a Board2D,
    pos_row: usize,
    pos_col: usize
}

impl<'a> Iterator for IteratorBoard2D<'a> {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        // println!("next >> self.pos_row={} self.pos_col={}", self.pos_row, self.pos_col);
        let mut ret: Option<Point> = None;
        let mut terminate = false;
        while self.pos_row < self.board.rows || self.pos_col < self.board.cols {
            if ret.is_some() {
                // println!("Breaking iteration at location x={} y={}", self.pos_col, self.pos_row);
                terminate = true;
                break
            }
            if self.board.is_alive(self.pos_col, self.pos_row) {
                // println!("Found point x={} y={}", self.pos_col, self.pos_row);
                ret = Some(Point{ x: self.pos_col, y: self.pos_row })
                // not breaking yet, want to make sure we update iterator position
                // to wherever we should *start* on calling next(&mut self) the next time.
            }
            if self.pos_col < self.board.cols {
                self.pos_col += 1;
            } else {
                self.pos_col = 0;
                self.pos_row += 1;
            }
        }
        // println!("Iteratorin returning, stopped at self.pos_col={} self.pos_row={}", self.pos_col, self.pos_row);
        return ret;
    }
}


impl CellUniverse for Board2D {
    fn set_cell_alive(&mut self, x: usize, y: usize) {
        self.data[x][y] = true
    }

    fn set_cell_dead(&mut self, x: usize, y: usize) {
        self.data[x][y] = false
    }

    fn set_cell_state(&mut self, x: usize, y: usize, alive: bool) {
        self.data[x][y] = alive
    }

    fn iter_alive<'a>(&'a self) -> Box<dyn Iterator<Item=Point> + 'a> {
        let iterator = IteratorBoard2D {
            board: &self,
            pos_row: 0,
            pos_col: 0
        };
        Box::new(iterator)
    }

    fn insert<'a>(&'a mut self, x_origin: usize, y_origin: usize, cells: Box<dyn Iterator<Item=Point> + 'a>) {
        for cell_point in cells {
            self.set_cell_alive(x_origin + cell_point.x, y_origin + cell_point.y)
        }
    }
}


impl Board2D {
    pub fn new(cols: usize, rows: usize) -> Board2D {
        Board2D {
            cols,
            rows,
            data: vec!(vec!(false; rows); cols),
        }
    }

    pub fn init_random(&mut self) {
        let mut rng = rand::thread_rng();
        for col_index in 1..self.cols {
            for row_index in 1..self.rows {
                let n1: u8 = rng.gen();
                if n1 % 2 == 0 {
                    self.set_cell_alive(col_index, row_index)
                }
            }
        }
    }

    pub fn init_from_plaintext_file(&mut self, path: &str) {
        let gol_plaintext = fs::read_to_string(path).unwrap();
        self.init_from_plaintext(&gol_plaintext, Some('o'))
    }

    pub fn init_from_plaintext(&mut self, data: &str, aliveChar: Option<char>) {
        let lines = data.split("\n");
        let mut row_i = 0;
        for line in lines {
            if line.chars().nth(0).unwrap() == '!' {
                continue
            }
            let mut col_i = 0;
            for token in line.chars() {
                if token == ' ' {
                    continue
                }
                if token == aliveChar.unwrap_or('o') {
                    self.set_cell_alive(col_i, row_i)
                }
                col_i += 1;
            }
            row_i += 1;
        }
    }

    pub fn make_cell_dead_all(&mut self) {
        for col_index in 1..self.cols {
            for row_index in 1..self.rows {
                self.set_cell_dead(col_index, row_index)
            }
        }
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
    use std::collections::HashMap;

    #[test]
    fn should_correct_construct_bitmap_from_string_1() {
        let mut f = Board2D::new(5, 5);
        let fdata = "\
- - - - -
- - x x -
- - - - -
- - - - -";
        f.init_from_plaintext(fdata, Some('x'));
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
        let mut f = Board2D::new(5, 5);
        let fdata = "\
x - - - x
x - x x -
- x - x -
- - - - x";
        f.init_from_plaintext(fdata, Some('x'));
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
        let mut f = Board2D::new(5, 5);
        let fdata = "\
- - - - -
- - x - x
- x - - x
- - x - -";
        f.init_from_plaintext(fdata, Some('x'));
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
        let mut f = Board2D::new(5, 5);
        let fdata = "\
x - - - -
- - - - -
- - - - -
- - - - -";
        f.init_from_plaintext(fdata, Some('x'));
        f.set_cell_dead(0, 0);
        assert_ne!(f.is_dead(0, 0), f.is_alive(0, 0));
    }

    #[test]
    fn should_make_cell_alive() {
        let mut f = Board2D::new(5, 5);
        let fdata = "\
- - - - -
- - - - -
- - - - -
- - - - -";
        f.init_from_plaintext(fdata, Some('x'));
        f.set_cell_alive(0, 0);
        assert!(f.is_alive(0, 0));
    }

    #[test]
    fn should_make_cell_dead() {
        let mut f = Board2D::new(5, 5);
        let fdata = "\
x - - - -
- - - - -
- - - - -
- - - - -";
        f.init_from_plaintext(fdata, Some('x'));
        f.set_cell_dead(0, 0);
        assert!(f.is_dead(0, 0));
    }


    #[test]
    fn should_read_plaintex() {
        let mut f = Board2D::new(5, 5);
        let fdata = "\
!Name: Gosper glider gun
!
........................O...........
......................O.O...........
............OO......OO............OO
...........O...O....OO............OO
OO........O.....O...OO..............
OO........O...O.OO....O.O...........
..........O.....O.......O...........
...........O...O....................
............OO......................";
        f.init_from_plaintext(fdata, Some('x'));
        f.set_cell_dead(0, 0);
        assert!(f.is_dead(0, 0));
    }

    #[test]
    fn should_read_plaintext_file() {
        let mut f = Board2D::new(5, 5);
        f.init_from_plaintext_file("./patterns/plaintext/glider.txt");
    }


    #[test]
    fn should_use_tuple_keyed_map() {
        let mut hashmapMain: HashMap<(usize, usize), bool> = HashMap::new();
        hashmapMain.insert((1, 2), false);
        hashmapMain.insert((2, 1), true);
        hashmapMain.insert((3, 1), true);
        hashmapMain.insert((4, 1), true);
        for ((x, y), val) in hashmapMain {
            println!("({}, {}) = {}", x, y, val);
        }
    }


}

