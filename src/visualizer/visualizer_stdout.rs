use crate::universe::universe::CellUniverse;

trait GolVisualizer {
    fn visualize(gol: Box<dyn CellUniverse>);
}

struct ConsoleVisualizer {}

impl GolVisualizer for ConsoleVisualizer {
    fn visualize(gol: Box<dyn CellUniverse>) {
        unimplemented!("todo")
    }
}