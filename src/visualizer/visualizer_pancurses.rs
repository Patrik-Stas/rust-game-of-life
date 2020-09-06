use pancurses::{curs_set, endwin, initscr, Window};

use crate::common::Point;
use crate::universe::universe::CellUniverse;
use crate::universe::universe_board_2d::Board2D;
use crate::visualizer::visualizer::UniverseVisualizer;

pub struct VisualizerPancures {
    window: Window
}

impl VisualizerPancures {
    pub fn new() -> VisualizerPancures {
        let window = initscr();
        curs_set(0);
        window.refresh();
        VisualizerPancures { window: window }
    }
}

impl UniverseVisualizer for VisualizerPancures {
    fn visualize(&mut self, cell_iterator: impl Iterator<Item=Point>) {
        self.window.clear();
        for cell in cell_iterator {
            self.window.mvaddch(cell.y as i32, cell.x as i32, 'x');
        }
        self.window.refresh();
    }
}
