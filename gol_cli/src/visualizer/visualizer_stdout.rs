use gol_core::common::Point;
use gol_core::universe::universe_board_2d::Board2D;

use crate::gol_core::universe::universe::CellUniverse;

pub struct ConsoleVisualizer {
    board: Board2D,
}

impl ConsoleVisualizer {
    pub fn new(cols_x: usize, rows_y: usize) -> ConsoleVisualizer {
        ConsoleVisualizer { board: Board2D::new(cols_x, rows_y) }
    }

    pub fn visualize(&mut self, cell_iterator: impl Iterator<Item=Point>) {
        self.board.wipe();
        self.board.insert(0, 0, Box::new(cell_iterator));
        self.board.print()
    }
}

pub fn fn_visualize_iterable(cols_x: usize, rows_y: usize, cell_iterator: impl Iterator<Item=Point>) {
    let mut board = Board2D::new(cols_x, rows_y);
    board.insert(0, 0, Box::new(cell_iterator));
    board.print()
}
