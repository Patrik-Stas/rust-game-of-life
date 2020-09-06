use crate::common::Point;
use crate::universe::universe::CellUniverse;
use crate::universe::universe_board_2d::Board2D;

// What shall be guidelines for deciding whether have something covered by Trait? Being restrained
// by implementing trait has some downsides - for example can't use impl Trait in function signatures.
// I suppose 1. traits make less sense the more outer layer of code we are dealing with
// and 2. make the less sense, the less chance of needing polymorphism for the given interface

pub struct ConsoleVisualizer {
    board: Board2D
}

impl ConsoleVisualizer {
    pub fn new(rows: usize, cols: usize) -> ConsoleVisualizer {
        let board = Board2D::new(cols, rows);
        ConsoleVisualizer { board }
    }

    pub fn fn_visualize(rows: usize, cols: usize, universe: impl CellUniverse) {
        let mut visualizer = ConsoleVisualizer::new(cols, rows);
        visualizer.visualize(universe);
    }

    pub fn fn_visualize_iterable(rows: usize, cols: usize, cell_iterator: impl Iterator<Item=Point>) {
        let mut board = Board2D::new(cols, rows);
        board.insert(0, 0, Box::new(cell_iterator));
        board.print()
    }

    pub fn visualize(&mut self, universe: impl CellUniverse) {
        self.board.insert(0, 0, Box::new(universe.iter_alive()));
        self.board.print()
    }
}