use std::collections::HashMap;

use crate::common::{Point, PointIterator, PointNeighbourIterator};
use crate::universe::universe::CellUniverse;

pub struct GolStateHash {
    hashmap_main: HashMap<(usize, usize), bool>

}

impl GolStateHash {
    pub fn new() -> GolStateHash {
        return GolStateHash {
            hashmap_main: Default::default()
        };
    }

    // how to restrict return type containing generics, for that generics to be a trait?
    // fn iter_alive_2<I>(& self) -> PointIterator<I> where I:Iterator<Item=Point> {
    //     let wrapper = PointIterator::new(
    //         self.hashmap_main
    //             .iter()
    //             .map(|((x, y), b)| {
    //                 Point { x: *x, y: *y }
    //             })
    //     );
    //     return wrapper;
    // }
}

impl CellUniverse for GolStateHash {
    fn set_cell_alive(&mut self, x: usize, y: usize) {
        self.hashmap_main.insert((x, y), true);
    }

    fn set_cell_dead(&mut self, x: usize, y: usize) {
        self.hashmap_main.remove_entry(&(x, y));
    }

    fn set_cell_state(&mut self, x: usize, y: usize, alive: bool) {
        if alive {
            self.set_cell_alive(x, y)
        } else {
            self.set_cell_dead(x, y)
        }
    }

    fn is_cell_alive(&self, x: usize, y: usize) -> bool {
        match self.hashmap_main.get(&(x, y)) {
            None => false,
            Some(_) => true
        }
    }

    fn is_cell_dead(&self, col: usize, row: usize) -> bool {
        !self.is_cell_alive(col, row)
    }

    fn wipe(&mut self) {
        self.hashmap_main = Default::default();
    }

    // todo: how to enable this? I want to return custom iterator so I could chain some custom ops on it
    // fn iter_alive<'a>(&'a self) -> PointIterator<Item=dyn Iterator> {
    fn iter_alive<'a>(&'a self) -> Box<dyn Iterator<Item=Point> + 'a> {
        let wrapper = PointIterator::new(
            self.hashmap_main
                .iter()
                .map(|((x, y), b)| {
                    Point { x: *x, y: *y }
                })
        );
        return Box::new(wrapper);
    }

    // If I could return custom Iterator from iter_alive, I could extract this out to util function reading point iterator, returning another point iterator
    fn iter_neighbours<'a>(&'a self) -> Box<dyn Iterator<Item=Point> + 'a> {
        Box::new(PointNeighbourIterator::new(self.iter_alive()))
    }

    fn insert<'a>(&'a mut self, x_origin: usize, y_origin: usize, cells: Box<dyn Iterator<Item=Point> + 'a>) {
        for cell_point in cells {
            self.set_cell_alive(x_origin + cell_point.x, y_origin + cell_point.y)
        }
    }
}


#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::slice::Split;

    use super::*;
    use crate::universe::util::{init_from_plaintext, init_from_plaintext_file};

//     #[test]
//     fn should_correct_construct_bitmap_from_string_1() {
//         let mut f = GolStateHash::new();
//         let fdata = "\
// - - - - -
// - - x x -
// - - - - -
// - - - - -";
//         init_from_plaintext(&f, fdata, Some('x'));
//         assert!(f.is_cell_alive(2, 1));
//         assert!(f.is_cell_alive(3, 1));
//
//         assert_eq!(false, f.has_alive_in_col(0));
//         assert_eq!(false, f.has_alive_in_col(1));
//         assert_eq!(false, f.has_alive_in_col(4));
//
//         assert_eq!(false, f.has_alive_in_row(0));
//         assert_eq!(false, f.has_alive_in_row(2));
//         assert_eq!(false, f.has_alive_in_row(3));
//     }

    #[test]
    fn should_iterate_alive_cells() {
        let mut f = GolStateHash::new();
        let fdata = "\
x - x
- - -
- x -
- - x";
        init_from_plaintext(&mut f, fdata, Some('x'));

        let mut points: HashMap<(usize, usize), bool> = Default::default();
        for pt in f.iter_alive() {
            let pt_tuple = (pt.x, pt.y);
            assert!(points.get(&pt_tuple) == None);
            points.insert(pt_tuple, true);
        }
        assert!(points.get(&(0,0)) != None);
        assert!(points.get(&(2,0)) != None);
        assert!(points.get(&(1,2)) != None);
        assert!(points.get(&(2,3)) != None);
    }

    #[test]
    fn should_iterate_alive_neihgbours() {
        let mut f = GolStateHash::new();
        let fdata = "\
- - -
- x -
- - -";
        init_from_plaintext(&mut f, fdata, Some('x'));
        let mut points: HashMap<(usize, usize), bool> = Default::default();
        for pt in f.iter_neighbours() {
            let pt_tuple = (pt.x, pt.y);
            assert!(points.get(&pt_tuple) == None);
            points.insert(pt_tuple, true);
        }
        assert_eq!(points.len(), 8);
        assert!(points.get(&(0,0)) != None);
        assert!(points.get(&(1,0)) != None);
        assert!(points.get(&(2,0)) != None);
        assert!(points.get(&(0,1)) != None);
        assert!(points.get(&(2,1)) != None);
        assert!(points.get(&(0,2)) != None);
        assert!(points.get(&(1,2)) != None);
        assert!(points.get(&(2,2)) != None);
    }


    #[test]
    fn should_correct_construct_bitmap_from_string_2() {
        let mut f = GolStateHash::new();
        let fdata = "\
x - - - x
x - x x -
- x - x -
- - - - x";
        init_from_plaintext(&mut f, fdata, Some('x'));
        assert!(f.is_cell_alive(0, 0));
        assert!(!f.is_cell_alive(1, 0));
        assert!(!f.is_cell_alive(2, 0));
        assert!(!f.is_cell_alive(3, 0));
        assert!(f.is_cell_alive(4, 0));

        assert!(f.is_cell_alive(0, 1));
        assert!(!f.is_cell_alive(1, 1));
        assert!(f.is_cell_alive(2, 1));
        assert!(f.is_cell_alive(3, 1));
        assert!(!f.is_cell_alive(4, 1));

        assert!(!f.is_cell_alive(0, 2));
        assert!(f.is_cell_alive(1, 2));
        assert!(!f.is_cell_alive(2, 2));
        assert!(f.is_cell_alive(3, 2));
        assert!(!f.is_cell_alive(4, 2));

        assert!(!f.is_cell_alive(0, 3));
        assert!(!f.is_cell_alive(1, 3));
        assert!(!f.is_cell_alive(2, 3));
        assert!(!f.is_cell_alive(3, 3));
        assert!(f.is_cell_alive(4, 3));
    }

    #[test]
    fn is_dead_should_be_opposite_of_is_cell_alive() {
        let mut f = GolStateHash::new();
        let fdata = "\
x - - - -
- - - - -
- - - - -
- - - - -";
        init_from_plaintext(&mut f, fdata, Some('x'));
        f.set_cell_dead(0, 0);
        assert_ne!(f.is_cell_dead(0, 0), f.is_cell_alive(0, 0));
    }

    #[test]
    fn should_make_cell_alive() {
        let mut f = GolStateHash::new();
        let fdata = "\
- - - - -
- - - - -
- - - - -
- - - - -";
        init_from_plaintext(&mut f, fdata, Some('x'));
        f.set_cell_alive(0, 0);
        assert!(f.is_cell_alive(0, 0));
    }

    #[test]
    fn should_make_cell_dead() {
        let mut f = GolStateHash::new();
        let fdata = "\
x - - - -
- - - - -
- - - - -
- - - - -";
        init_from_plaintext(&mut f, fdata, Some('x'));
        f.set_cell_dead(0, 0);
        assert!(f.is_cell_dead(0, 0));
    }


    #[test]
    fn should_read_plaintex() {
        let mut f = GolStateHash::new();
        let fdata = "\
!Name: Gosper glider gun
!
........................O...........
......................O.O...........
............OO......OO............OO
...........O...O....OO............OO
OO........O.....O...OO..............
OO........O...O.OO....O.O...........
..........O.....O.......O...........
...........O...O....................
............OO......................";
        init_from_plaintext(&mut f, fdata, Some('x'));
        f.set_cell_dead(0, 0);
        assert!(f.is_cell_dead(0, 0));
    }

    #[test]
    fn should_read_plaintext_file() {
        let mut f = GolStateHash::new();
        init_from_plaintext_file(&mut f, "./patterns/plaintext/glider.txt");
    }
}

