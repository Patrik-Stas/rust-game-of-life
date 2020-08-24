use crate::gol::Gol;

trait GolVisualizer {
    fn visualize(impl Gol);
}

struct ConsoleVisualizer {}

impl GolVisualizer for ConsoleVisualizer {
    fn visualize(gol: impl Gol) {

    }
}