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
use crate::universe::util::{init_from_plaintext_file, init_random};
use crate::visualizer::visualizer_pancurses::VisualizerPancures;
use pancurses::{initscr, endwin};
use crate::visualizer::visualizer::UniverseVisualizer;
use crate::visualizer::visualizer_stub::VizualizerStub;

fn run_in(mut universe: impl CellUniverse, mut visualizer: impl UniverseVisualizer) {
    // println!("Starting GOL");
    let mut iter = 0;
    let limit =100;
    loop {
        visualizer.visualize(universe.iter_alive());
        iter += 1;
        // println!("iter = {}", iter);
        // if iter > limit {
        //     println!("finito");
        //     return
        // }
        let delta = UpdateDelta::new(&universe);
        for ((x, y), action) in delta.actions {
            universe.set_cell_state(x, y, action.make_alive);
        }
        sleep(Duration::from_millis(60));


    }
}



fn main() {
    let mut glider = Board2D::new(200, 60);
    // init_from_plaintext_file(&mut glider, "./patterns/plaintext/glider.txt");
    init_random(0, 200, 0, 60, &mut glider);
    // glider.init_random();
    // let mut bigbox = Board2D::new(10, 10);
    let mut bigbox = GolStateHash::new();
    // let mut bigbox = Board2D::new(500, 150);
    bigbox.insert(0,0, glider.iter_alive());

    let visualizer = VisualizerPancures::new(0, 0);
    // let visualizer = VizualizerStub::new();
    run_in(bigbox, visualizer);
}
