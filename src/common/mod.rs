use std::iter::Skip;
use std::panic;

#[derive(Debug)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

pub struct CellPoint {
    point: Point,
    is_alive: bool,
}

pub struct PointIterator<I> {
    iter: I
}

impl<I> Iterator for PointIterator<I> where I: Iterator<Item=Point> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<I> PointIterator<I> {
    pub fn new(iter: I) -> PointIterator<I> {
        PointIterator { iter }
    }

    pub fn neighbours(self) -> PointNeighbourIterator<I> {
        PointNeighbourIterator::new(self.iter)
    }
}

pub struct PointNeighbourIterator<I> {
    iter: I,
    current_pt: Option<Point>,
    current_neighbour: u8,
    started: bool,
    // nbstate signals neighbour position to iterate
    // 0  1  2
    // 3  X  4
    // 5  6  7
}

impl<I> PointNeighbourIterator<I> {
    pub fn new(iter: I) -> PointNeighbourIterator<I> {
        PointNeighbourIterator { iter, current_pt: None, current_neighbour: 0, started: false }
    }
}

impl<I> Iterator for PointNeighbourIterator<I> where I: Iterator<Item=Point> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let prev_hook = panic::take_hook();
        panic::set_hook(Box::new(|_| {}));
        loop {
            if !self.started {
                self.started = true;
                self.current_pt = self.iter.next();
                self.current_neighbour = 0;
            } else {
                self.current_neighbour += 1;
                if (8..).contains(&self.current_neighbour) {
                    self.current_pt = self.iter.next();
                    self.current_neighbour = 0;
                }
            }
            let (iter_x, iter_y)  = match &self.current_pt {
                None => return None,
                Some(pt) => (pt.x, pt.y)
            };
            let neighbour_type = self.current_neighbour;

            let calc_result = panic::catch_unwind(|| {
                match neighbour_type {
                    0 => Point { x: iter_x - 1, y: iter_y - 1 },
                    1 => Point { x: iter_x, y: iter_y - 1 },
                    2 => Point { x: iter_x + 1, y: iter_y - 1 },
                    3 => Point { x: iter_x - 1, y: iter_y },
                    4 => Point { x: iter_x + 1, y: iter_y },
                    5 => Point { x: iter_x - 1, y: iter_y + 1 },
                    6 => Point { x: iter_x, y: iter_y + 1 },
                    7 => Point { x: iter_x + 1, y: iter_y + 1 },
                    _ => panic!("Invalid current_neighbour value.")
                }
            });
            match calc_result {
                Ok(neighbour_pt) => return {
                    panic::set_hook(prev_hook);
                    Some(neighbour_pt)
                },
                Err(_) => {
                    // println!("error, skipping point");
                    // println!("neighbour_type = {}, iter_x={}, iter_y={}", neighbour_type, iter_x, iter_y);
                    // overflow happened, we tried to calculate point beyond usize boundary. Will move onto next neighbour
                    continue
                }
            }
        }
    }
}
