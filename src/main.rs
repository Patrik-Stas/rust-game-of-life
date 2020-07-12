use crate::gol_bitmap::GolBitmap;
use std::thread::sleep;
use std::time::Duration;

mod gol_bitmap;

struct GolState {
    pub main: GolBitmap,
    buffer: GolBitmap
    // activeCells:
}.dockerignore
.gitignore

impl GolState {
    pub fn new(main: GolBitmap) -> GolState {
        let mut buffer  = GolBitmap::new(main.cols, main.rows);
        let mut golState = GolState { main, buffer };
        golState._clone_main_to_buffer();
        return golState
    }

    fn _clone_main_to_buffer(&mut self) {
        for col_i in 0..self.main.cols {
            for row_i in 0..self.main.rows {
                self.buffer.set_state(col_i, row_i, self.main.is_alive(col_i, row_i))
            }
        }
    }

    fn _clone_buffer_to_main(&mut self) {
        for col_i in 0..self.main.cols {
            for row_i in 0..self.main.rows {
                self.main.set_state(col_i, row_i, self.buffer.is_alive(col_i, row_i))
            }
        }
    }

    pub fn transition(&mut self) {

        self._transition_begin();
        for col_i in 0..self.main.cols {
            for row_i in 0..self.main.rows {
                // println!("Updaring {} {}", col_i, row_i);
                let neighbours = self.main.count_neighbours(col_i, row_i);
                if self.main.is_alive(col_i, row_i) {
                    if neighbours < 2 || neighbours >= 4{
                       self.buffer.kill(col_i, row_i)
                    }
                } else {
                    if neighbours == 3 {
                        self.buffer.reincarnate(col_i, row_i)
                    }
                }
            }
        }
        self._transition_finish();
    }

    fn _transition_begin(&mut self) {

    }

    fn _transition_finish(&mut self) {
        self._clone_buffer_to_main()
    }

    pub fn transition_set_alive(&mut self, col: usize, row: usize) {
        self.buffer.reincarnate(col, row)
    }

    pub fn transition_set_dead(&mut self, col: usize, row: usize) {
        self.buffer.kill(col, row)
    }

    pub fn print(&self) {
        self.main.print();
    }
}

fn main() {
    let mut golBitmap = GolBitmap::new(1000, 1000);
    golBitmap.init_random();
    // golBitmap.print();

    let mut golState = GolState::new(golBitmap);
    let mut iter = 0;
    loop {
        iter += 1;
        println!("------ UPDATE ------ {}", iter);
        golState.transition();
        // sleep(Duration::from_millis(50));
        // golState.print();
    }
}

