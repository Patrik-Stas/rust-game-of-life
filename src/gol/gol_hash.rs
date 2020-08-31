// use std::collections::HashMap;
// use crate::gol_bitmap::Board2D;
//
// pub struct GolStateHash {
//     hashmapMain: HashMap<(usize, usize), bool>
// }
//
// pub struct CandidateIterator {
//     hashmapMain: HashMap<(usize, usize), bool>,
//     // TODO: maybe we should have hashmap's iterator here
//     // and for each hasmap iteration, we should iterate over the neighbours
//
// }
//
// impl GolStateHash {
//     pub fn new(bitmap: &Board2D) -> GolState {
//         hashmapMain = HashMap::new();
//         candidates = HashMap::new();
//         let mut gol_state = GolStateHash { hashmapMain };
//         gol_state.insert(0, 0, bitmap);
//         return gol_state
//     }
//
//     pub fn transition(&mut self) {
//         for col_i in 0..self.main.cols {
//             for row_i in 0..self.main.rows {
//                 // println!("Updaring {} {}", col_i, row_i);
//                 let neighbours = self.main.count_neighbours(col_i, row_i);
//                 if self.main.is_alive(col_i, row_i) {
//                     if neighbours < 2 || neighbours >= 4{
//                         self.buffer.set_dead(col_i, row_i)
//                     }
//                 } else {
//                     if neighbours == 3 {
//                         self.buffer.set_alive(col_i, row_i)
//                     }
//                 }
//             }
//         }
//     }
//
//     pub fn alives(&self) -> impl Iterator<bool> {
//         self.hashmapMain.iter()
//     }
//
//     pub fn insert(&mut self, x: usize, y: usize, bitmap: &Board2D) {
//         for xi in 0..bitmap.cols {
//             for yi in 0..bitmap.rows {
//                 self.set_alive(x + xi, y + yi);
//             }
//         }
//     }
//
//     pub fn set_alive(&mut self, x: usize, y: usize) {
//         self.hashmapMain.insert((x, y), true);
//     }
//
//     pub fn set_dead(&mut self, x: usize, y: usize) {
//         self.hashmapMain.remove_entry(&(x, y));
//     }
//
//     pub fn print(&self) {
//         self.main.print();
//     }
// }