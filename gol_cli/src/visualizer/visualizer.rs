use gol_core::common::Point;

pub trait UniverseVisualizer {
    fn visualize(&mut self, cell_iterator: impl Iterator<Item=Point>);
}
