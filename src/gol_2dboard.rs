
pub struct GolBoard2D {
    pub main: Board2D,
    buffer: Board2D
}

impl GolBoard2D {
    pub fn new(main: Board2D) -> GolBoard2D {
        let mut buffer  = Board2D::new(main.cols, main.rows);
        let mut golState = GolBoard2D { main, buffer };
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
                        self.buffer.set_dead(col_i, row_i)
                    }
                } else {
                    if neighbours == 3 {
                        self.buffer.set_alive(col_i, row_i)
                    }
                }
            }
        }
        self._transition_finish();
    }

    // pub fn insert(&mut self, x: u32, y: u32, bitmap: &Board2D) {
    //     for col_i in 0..bitmap.cols {
    //         for row_i in 0..self.main.rows {
    //             // println!("Updaring {} {}", col_i, row_i);
    //             let neighbours = self.main.count_neighbours(col_i, row_i);
    //             if self.main.is_alive(col_i, row_i) {
    //                 if neighbours < 2 || neighbours >= 4{
    //                     self.buffer.set_dead(col_i, row_i)
    //                 }
    //             } else {
    //                 if neighbours == 3 {
    //                     self.buffer.set_alive(col_i, row_i)
    //                 }
    //             }
    //         }
    //     }
    // }

    fn _transition_begin(&mut self) {

    }

    fn _transition_finish(&mut self) {
        self._clone_buffer_to_main()
    }

    pub fn transition_set_alive(&mut self, col: usize, row: usize) {
        self.buffer.set_alive(col, row)
    }

    pub fn transition_set_dead(&mut self, col: usize, row: usize) {
        self.buffer.set_dead(col, row)
    }

    pub fn print(&self) {
        self.main.print();
    }
}