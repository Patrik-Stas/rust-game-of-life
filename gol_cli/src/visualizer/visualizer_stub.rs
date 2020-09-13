use gol_core::common::Point;

use crate::visualizer::visualizer::UniverseVisualizer;

pub struct VizualizerStub {}

impl VizualizerStub {
    pub fn new() -> VizualizerStub { VizualizerStub {} }
}

impl UniverseVisualizer for VizualizerStub {
    fn visualize(&mut self, _cell_iterator: impl Iterator<Item=Point>) {}
}