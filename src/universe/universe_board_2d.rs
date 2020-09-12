use std::fs;

use rand::Rng;

use crate::common::{Point, PointNeighbourIterator};
use crate::universe::universe::CellUniverse;

pub struct Board2D {
    data: Vec<Vec<bool>>,
    pub cols_x: usize,
    pub rows_y: usize,
}

pub struct IteratorBoard2D<'a> {
    board: &'a Board2D,
    pos_x: usize,
    pos_y: usize,
}

impl<'a> Iterator for IteratorBoard2D<'a> {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        // println!("next >> self.pos_row={} self.pos_col={}", self.pos_row, self.pos_col);
        let mut ret: Option<Point> = None;
        let mut terminate = false;
        while self.pos_x < self.board.cols_x || self.pos_y < self.board.rows_y {
            if ret.is_some() {
                // println!("Breaking iteration at location x={} y={}", self.pos_col, self.pos_row);
                terminate = true;
                break;
            }
            if self.board.is_cell_alive(self.pos_x, self.pos_y) {
                // println!("Found point x={} y={}", self.pos_col, self.pos_row);
                ret = Some(Point { x: self.pos_x, y: self.pos_y })
                // not breaking yet, want to make sure we update iterator position
                // to wherever we should *start* on calling next(&mut self) the next time.
            }
            if self.pos_y < self.board.rows_y {
                self.pos_y += 1;
            } else {
                self.pos_y = 0;
                self.pos_x += 1;
            }
        }
        return ret;
    }
}


impl CellUniverse for Board2D {
    fn set_cell_alive(&mut self, x: usize, y: usize) {
        self.set_cell_state(x, y, true)
    }

    fn set_cell_dead(&mut self, x: usize, y: usize) {
        self.set_cell_state(x, y, false)
    }

    fn set_cell_state(&mut self, x: usize, y: usize, alive: bool) {
        if !self.is_out_of_bounds(x, y) {
            self.data[x][y] = alive
        }
    }

    fn is_cell_alive(&self, x: usize, y: usize) -> bool {
        if self.is_out_of_bounds(x, y) {
            false
        } else {
            self.data[x][y]
        }
    }

    fn is_cell_dead(&self, col: usize, row: usize) -> bool {
        !self.is_cell_alive(col, row)
    }

    fn wipe(&mut self) {
        self.data = Board2D::_create_empty_data(self.cols_x, self.rows_y);
    }

    fn iter_alive<'a>(&'a self) -> Box<dyn Iterator<Item=Point> + 'a> {
        let iterator = IteratorBoard2D {
            board: &self,
            pos_x: 0,
            pos_y: 0,
        };
        Box::new(iterator)
    }

    fn iter_neighbours<'a>(&'a self) -> Box<dyn Iterator<Item=Point> + 'a> {
        Box::new(PointNeighbourIterator::new(self.iter_alive()))
    }

    fn insert<'a>(&'a mut self, x_origin: usize, y_origin: usize, cells: Box<dyn Iterator<Item=Point> + 'a>) {
        for cell_point in cells {
            self.set_cell_alive(x_origin + cell_point.x, y_origin + cell_point.y)
        }
    }
}


impl Board2D {
    fn _create_empty_data(cols: usize, rows: usize) -> Vec<Vec<bool>> {
        vec!(vec!(false; rows); cols)
    }

    pub fn new(cols_x: usize, rows_y: usize) -> Board2D {
        Board2D {
            cols_x,
            rows_y,
            data: Board2D::_create_empty_data(cols_x, rows_y),
        }
    }

    fn is_out_of_bounds(&self, x: usize, y: usize) -> bool {
        let res = x >= self.data.len() || y >= self.data[x].len();
        res
    }


    pub fn make_cell_dead_all(&mut self) {
        for col_index in 1..self.cols_x {
            for row_index in 1..self.rows_y {
                self.set_cell_dead(col_index, row_index)
            }
        }
    }

    pub fn has_alive_in_row(&self, row: usize) -> bool {
        for col_index in 1..self.cols_x {
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

    pub fn print(&self) {
        for iy in 1..self.rows_y {
            for ix in 1..self.cols_x {
                if self.is_cell_alive(ix, iy) {
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
    use std::collections::HashMap;
    use std::slice::Split;

    use super::*;
    use crate::universe::util::{init_from_plaintext, init_from_plaintext_file};

    #[test]
    fn should_correct_construct_bitmap_from_string_1() {
        let mut f = Board2D::new(5, 5);
        let fdata = "\
- - - - -
- - x x -
- - - - -
- - - - -";
        init_from_plaintext(&mut f, fdata, Some('x'));
        assert!(f.is_cell_alive(2, 1));
        assert!(f.is_cell_alive(3, 1));

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
        init_from_plaintext(&mut f, fdata, Some('x'));
        assert!(f.is_cell_alive(0, 0));
        assert!(!f.is_cell_alive(1, 0));
        assert!(!f.is_cell_alive(2, 0));
        assert!(!f.is_cell_alive(3, 0));
        assert!(f.is_cell_alive(4, 0));

        assert!(f.is_cell_alive(0, 1));
        assert!(!f.is_cell_alive(1, 1));
        assert!(f.is_cell_alive(2, 1));
        assert!(f.is_cell_alive(3, 1));
        assert!(!f.is_cell_alive(4, 1));

        assert!(!f.is_cell_alive(0, 2));
        assert!(f.is_cell_alive(1, 2));
        assert!(!f.is_cell_alive(2, 2));
        assert!(f.is_cell_alive(3, 2));
        assert!(!f.is_cell_alive(4, 2));

        assert!(!f.is_cell_alive(0, 3));
        assert!(!f.is_cell_alive(1, 3));
        assert!(!f.is_cell_alive(2, 3));
        assert!(!f.is_cell_alive(3, 3));
        assert!(f.is_cell_alive(4, 3));
    }

    #[test]
    fn should_iterate_alive_neihgbours() {
        let mut f = Board2D::new(3, 3);
        let fdata = "\
- - -
- x -
- - -";
        init_from_plaintext(&mut f, fdata, Some('x'));
        let mut points: HashMap<(usize, usize), bool> = Default::default();
        for pt in f.iter_neighbours() {
            let pt_tuple = (pt.x, pt.y);
            assert!(points.get(&pt_tuple) == None);
            points.insert(pt_tuple, true);
        }
        assert_eq!(points.len(), 8);
        assert!(points.get(&(0,0)) != None);
        assert!(points.get(&(1,0)) != None);
        assert!(points.get(&(2,0)) != None);
        assert!(points.get(&(0,1)) != None);
        assert!(points.get(&(2,1)) != None);
        assert!(points.get(&(0,2)) != None);
        assert!(points.get(&(1,2)) != None);
        assert!(points.get(&(2,2)) != None);
    }


    #[test]
    fn is_dead_should_be_opposite_of_is_cell_alive() {
        let mut f = Board2D::new(5, 5);
        let fdata = "\
x - - - -
- - - - -
- - - - -
- - - - -";
        init_from_plaintext(&mut f, fdata, Some('x'));
        f.set_cell_dead(0, 0);
        assert_ne!(f.is_cell_dead(0, 0), f.is_cell_alive(0, 0));
    }

    #[test]
    fn should_make_cell_alive() {
        let mut f = Board2D::new(5, 5);
        let fdata = "\
- - - - -
- - - - -
- - - - -
- - - - -";
        init_from_plaintext(&mut f, fdata, Some('x'));
        f.set_cell_alive(0, 0);
        assert!(f.is_cell_alive(0, 0));
    }

    #[test]
    fn should_make_cell_dead() {
        let mut f = Board2D::new(5, 5);
        let fdata = "\
x - - - -
- - - - -
- - - - -
- - - - -";
        init_from_plaintext(&mut f, fdata, Some('x'));
        f.set_cell_dead(0, 0);
        assert!(f.is_cell_dead(0, 0));
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
        init_from_plaintext(&mut f, fdata, Some('x'));
        f.set_cell_dead(0, 0);
        assert!(f.is_cell_dead(0, 0));
    }

    #[test]
    fn should_read_plaintext_file() {
        let mut f = Board2D::new(5, 5);
        init_from_plaintext_file(&mut f, "./patterns/plaintext/glider.txt");
    }
}
