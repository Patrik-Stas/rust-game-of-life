mod visualizer;
mod gol;
mod universe;
mod common;

use std::thread::sleep;
use std::time::Duration;
use std::collections::HashMap;
use crate::universe::universe_board_2d::Board2D;
use crate::universe::universe::CellUniverse;
use crate::visualizer::visualizer_stdout::ConsoleVisualizer;
use universe::universe_hash::{GolStateHash};
use crate::gol::delta::UpdateDelta;
use crate::universe::util::init_from_plaintext_file;
use crate::visualizer::visualizer_pancurses::VisualizerPancures;
use pancurses::{initscr, endwin};
use crate::visualizer::visualizer::UniverseVisualizer;

fn run_in(mut universe: impl CellUniverse, mut visualizer: impl UniverseVisualizer) {
    let mut iter = 0;
    loop {
        visualizer.visualize(universe.iter_alive());
        iter += 1;
        let delta = UpdateDelta::new(&universe);
        for ((x, y), action) in delta.actions {
            universe.set_cell_state(x, y, action.make_alive);
        }
        sleep(Duration::from_millis(100));
    }
}


fn run_in_stdout(mut universe: impl CellUniverse) {
    let mut iter = 0;
    let mut visualizer = ConsoleVisualizer::new(10, 10);
    loop {
        visualizer.visualize(universe.iter_alive());
        iter += 1;
        let delta = UpdateDelta::new(&universe);
        for ((x, y), action) in delta.actions {
            universe.set_cell_state(x, y, action.make_alive);
        }
        sleep(Duration::from_millis(100));
    }
}

fn main() {
    let mut glider = Board2D::new(50, 50);
    init_from_plaintext_file(&mut glider, "./patterns/plaintext/glider.txt");
    // glider.init_random();
    // let mut bigbox = Board2D::new(10, 10);
    let mut bigbox = GolStateHash::new();
    bigbox.insert(20,15, glider.iter_alive());

    let visualizer = VisualizerPancures::new();
    run_in(bigbox, visualizer);

}
