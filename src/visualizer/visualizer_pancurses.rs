use pancurses::{curs_set, endwin, initscr, Window, Input};

use crate::common::Point;
use crate::universe::universe::CellUniverse;
use crate::universe::universe_board_2d::Board2D;
use crate::visualizer::visualizer::UniverseVisualizer;

pub struct VisualizerPancures {
    window: Window,
    baseview_x: i32,
    baseview_y: i32
}

impl VisualizerPancures {
    pub fn new(baseview_x: i32, baseview_y: i32) -> VisualizerPancures {
        let window = initscr();
        curs_set(0);
        window.refresh();
        window.nodelay(true);
        VisualizerPancures { window, baseview_x, baseview_y }
    }
}

impl UniverseVisualizer for VisualizerPancures {
    fn visualize(&mut self, cell_iterator: impl Iterator<Item=Point>) {
        self.window.mv(4,0);
        self.window.clrtobot();
        for cell in cell_iterator {
            let render_x = cell.x as i32 - self.baseview_x;
            let render_y = cell.y as i32 - self.baseview_y;
            if render_x > 0 || render_y > 0 {
                self.window.mvaddch(render_y, render_x,'x');
            }
        }
        self.window.mvaddstr(0, 0, format!("base_x={} base_y={}", self.baseview_x, self.baseview_y));
        match self.window.getch() {
            Some(Input::Character(c)) => {
                match c {
                    'i' => self.baseview_y += 5,
                    'k' => self.baseview_y -= 5,
                    'l' => self.baseview_x -= 10,
                    'j' => self.baseview_x += 10,
                    _ => {
                        self.window.mvaddstr(3, 2, format!("Nomatch"));
                    }
                }
            }
            _ => {
                self.window.mvaddstr(3, 2, format!("Nomatch"));
            }
        }
        self.window.refresh();
    }
}
