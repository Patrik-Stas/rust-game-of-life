mod visualizer;
mod gol;
mod universe;
mod common;

use std::thread::sleep;
use std::time::Duration;
use std::collections::HashMap;
use gol::gol_board_2d::GolBoard2D;
use crate::universe::universe_board_2d::Board2D;
use crate::universe::universe::CellUniverse;

// mod gol_state_hash;

fn main() {
    let mut board2d = Board2D::new(40, 40);
    board2d.init_random();

    let mut gol_board2d = GolBoard2D::new();
    gol_board2d.set_cell_alive(1, 1);
    gol_board2d.set_cell_alive(5, 2);
    gol_board2d.set_cell_alive(10, 20);
    gol_board2d.set_cell_alive(3, 13);

    gol_board2d.print();
    let alive_iterator = gol_board2d.iter_alive();
    for point in alive_iterator {
        println!("Alice point = {:?}", point);
    }

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

