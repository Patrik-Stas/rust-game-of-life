use crate::common::Point;
use crate::universe::universe::CellUniverse;
use crate::universe::universe_board_2d::Board2D;
use crate::visualizer::visualizer::UniverseVisualizer;

pub struct VizualizerStub {}

impl VizualizerStub {
    pub fn new() -> VizualizerStub { VizualizerStub {  } }
}

impl UniverseVisualizer for VizualizerStub {
    fn visualize(&mut self, _cell_iterator: impl Iterator<Item=Point>) { }
}