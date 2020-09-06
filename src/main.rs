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

fn run_loop(mut universe: impl CellUniverse) {
    let mut iter = 0;
    loop {
        ConsoleVisualizer::fn_visualize_iterable(50, 50, universe.iter_alive());
        iter += 1;
        println!("------ UPDATE ------ {}", iter);
        let delta = UpdateDelta::new(&universe);
        println!("delta = {:?}", delta);
        for ((x, y), alive) in delta.actions {
            universe.set_cell_state(x, y, alive);
        }
        sleep(Duration::from_millis(300));
    }

}


fn main() {
    let mut glider = Board2D::new(50, 50);
    init_from_plaintext_file(&mut glider, "./patterns/plaintext/glider.txt");
    // glider.init_random();
    let mut bigbox = Board2D::new(80, 80);
    // let mut bigbox = GolStateHash::new();
    bigbox.insert(15,15, glider.iter_alive());

    let mut gol_hash = GolStateHash::new();
    // gol_hash.insert(10, 10, glider.iter_alive());

    run_loop(bigbox);

    // glider.print();
    // ConsoleVisualizer::fn_visualize(50, 50, glider);
    // ConsoleVisualizer::fn_visualize_iterable(50, 50, glider.iter_alive());
    // ConsoleVisualizer::fn_visualize_iterable(50, 50, glider.iter_neighbours());
    // let delta = UpdateDelta::new(&glider);
    // println!("delta = {:?}", delta);

    // golhash.insert(10, 10, glider.iter_alive());
    //
    // let mut visualizer = ConsoleVisualizer::new(40, 40);
    // visualizer.visualize(golhash);

    // gol_board2d.print();
    // let alive_iterator = gol_board2d.iter_alive();
    // for point in alive_iterator {
    //     println!("Alice point = {:?}", point);
    // }

    // for point in gol_2dboard.iter_alive().unbox() {
    //     println!("Alive point = {}", point);
    // }

    // let mut iter = 0;
    // loop {
    //     iter += 1;
    //     println!("------ UPDATE ------ {}", iter);
    //     golState.transition();
    //     sleep(Duration::from_millis(150));
    //     golState.print();
    // }
}

