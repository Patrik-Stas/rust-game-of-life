use crate::common::Point;
use crate::universe::universe::CellUniverse;
use crate::universe::universe_board_2d::Board2D;

pub trait UniverseVisualizer {
    fn visualize(&mut self, cell_iterator: impl Iterator<Item=Point>);
}
