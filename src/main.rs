mod visualizer_stdout;
mod gol;

use std::thread::sleep;
use std::time::Duration;
use std::collections::HashMap;
use crate::gol_2dboard::GolBoard2D;
use crate::board2d::Board2D;

mod board2d;
mod gol_state_hash;
mod gol_2dboard;
// mod gol_state_hash;

fn main() {
    let mut board2d = Board2D::new(40, 40);
    board2d.init_random();

    let mut golState = GolBoard2D::new(board2d);
    let mut iter = 0;
    loop {
        iter += 1;
        println!("------ UPDATE ------ {}", iter);
        golState.transition();
        sleep(Duration::from_millis(150));
        golState.print();
    }
}

