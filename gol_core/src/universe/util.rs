use crate::universe::universe::CellUniverse;
use rand::Rng;
use std::fs;

pub fn init_random(x_from: usize, x_to: usize, y_from: usize, y_to: usize, universe: &mut impl CellUniverse) {
    let mut rng = rand::thread_rng();
    for col_index in x_from..x_to {
        for row_index in y_from..y_to {
            let n1: u8 = rng.gen();
            if n1 % 10 == 0 {
                universe.set_cell_alive(col_index, row_index)
            }
        }
    }
}

pub fn init_from_plaintext_file(universe: &mut impl CellUniverse, path: &str) {
    let gol_plaintext = fs::read_to_string(path).unwrap();
    init_from_plaintext(universe, &gol_plaintext, Some('O'))
}

pub fn init_from_plaintext(universe: &mut impl CellUniverse, data: &str, aliveChar: Option<char>) {
    let lines = data.split("\n");
    let mut iy = 0;
    for line in lines {
        if line.chars().nth(0).unwrap() == '!' {
            continue;
        }
        let mut ix = 0;
        for token in line.chars() {
            if token == ' ' {
                continue;
            }
            if token == aliveChar.unwrap_or('O') {
                println!("x={} y={}", ix, iy);
                universe.set_cell_alive(ix, iy)
            }
            ix += 1;
        }
        iy += 1;
    }
}