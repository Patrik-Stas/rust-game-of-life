use pancurses::{curs_set, endwin, initscr, Input, Window};

use gol_core::common::Point;

use crate::visualizer::visualizer::UniverseVisualizer;

pub struct VisualizerPancures {
    window: Window,
    baseview_x: i32,
    baseview_y: i32,
}

impl VisualizerPancures {
    pub fn new(baseview_x: i32, baseview_y: i32) -> VisualizerPancures {
        let window = initscr();
        pancurses::start_color();
        pancurses::init_pair(1, pancurses::COLOR_RED, pancurses::COLOR_RED);
        pancurses::init_pair(2, pancurses::COLOR_GREEN, pancurses::COLOR_GREEN);
        pancurses::nl();
        pancurses::noecho();
        curs_set(0);
        window.refresh();
        window.nodelay(true);
        VisualizerPancures { window, baseview_x, baseview_y }
    }
}

impl UniverseVisualizer for VisualizerPancures {
    fn visualize(&mut self, cell_iterator: impl Iterator<Item=Point>) {
        self.window.mv(4, 0);
        let color: i16 = 0x1234;
        self.window.color_set(color);
        self.window.clrtobot();
        for cell in cell_iterator {
            let render_x = cell.x as i32 - self.baseview_x;
            let render_y = cell.y as i32 - self.baseview_y;
            if render_x > 0 || render_y > 0 {
                self.window.mv(render_y, render_x);
                self.window.attrset(pancurses::COLOR_PAIR(2));
                // self.window.attrset(pancurses::COLOR_PAIR(2));
                self.window.addch('.');
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
