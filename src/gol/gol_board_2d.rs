use crate::universe::universe_board_2d::Board2D;
use crate::universe::universe::CellUniverse;
use crate::common::Point;
use crate::gol::gol::Gol;

pub struct GolBoard2D {
    pub main: Board2D,
    buffer: Board2D,
}

impl GolBoard2D {
    pub fn new() -> GolBoard2D {
        return GolBoard2D {
            main: Board2D::new(100, 100),
            buffer: Board2D::new(100, 100),
        };
    }

    fn _clone_main_to_buffer(&mut self) {
        for col_i in 0..self.main.cols {
            for row_i in 0..self.main.rows {
                self.buffer.set_cell_state(col_i, row_i, self.main.is_alive(col_i, row_i))
            }
        }
    }

    fn _clone_buffer_to_main(&mut self) {
        for col_i in 0..self.main.cols {
            for row_i in 0..self.main.rows {
                self.main.set_cell_state(col_i, row_i, self.buffer.is_alive(col_i, row_i))
            }
        }
    }

    fn _transition_finish(&mut self) {
        self._clone_buffer_to_main()
    }

    pub fn transition_set_alive(&mut self, col: usize, row: usize) {
        self.buffer.set_cell_alive(col, row)
    }

    pub fn transition_set_dead(&mut self, col: usize, row: usize) {
        self.buffer.set_cell_dead(col, row)
    }

    pub fn print(&self) {
        self.main.print();
    }
}

impl Gol for GolBoard2D {
    fn update(&mut self) {
        for col_i in 0..self.main.cols {
            for row_i in 0..self.main.rows {
                let neighbours = self.main.count_neighbours(col_i, row_i);
                if self.main.is_alive(col_i, row_i) {
                    if neighbours < 2 || neighbours >= 4 {
                        self.buffer.set_cell_dead(col_i, row_i)
                    }
                } else {
                    if neighbours == 3 {
                        self.buffer.set_cell_alive(col_i, row_i)
                    }
                }
            }
        }
        self._transition_finish();
    }
}

impl CellUniverse for GolBoard2D {
    fn set_cell_alive(&mut self, x: usize, y: usize) {
        self.main.set_cell_alive(x, y)
    }

    fn set_cell_dead(&mut self, x: usize, y: usize) {
        self.main.set_cell_dead(x, y)
    }

    fn set_cell_state(&mut self, x: usize, y: usize, alive: bool) {
        self.main.set_cell_state(x, y, alive)
    }

    fn iter_alive<'a>(&'a self) -> Box<dyn Iterator<Item=Point> + 'a> {
        self.main.iter_alive()
    }

    fn insert<'a>(&'a mut self, x: usize, y: usize, cells: Box<dyn Iterator<Item=Point> +'a>) {
        self.main.insert(x, y, cells)
    }
}
