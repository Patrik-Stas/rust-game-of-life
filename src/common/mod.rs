use std::iter::Skip;

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
    started: bool
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
        match &self.current_pt {
            None => return None,
            Some(pt) => {
                match self.current_neighbour {
                    // todo: find better solution for these +2 to avoid going into negative with usize
                    0 => Some(Point { x: pt.x - 1, y: pt.y - 1 }),
                    1 => Some(Point { x: pt.x,     y: pt.y - 1 }),
                    2 => Some(Point { x: pt.x + 1, y: pt.y - 1 }),
                    3 => Some(Point { x: pt.x - 1, y: pt.y     }),
                    4 => Some(Point { x: pt.x + 1, y: pt.y     }),
                    5 => Some(Point { x: pt.x - 1, y: pt.y + 1 }),
                    6 => Some(Point { x: pt.x,     y: pt.y + 1 }),
                    7 => Some(Point { x: pt.x + 1, y: pt.y + 1 }),
                    _ => panic!("Invalid current_neighbour value.")
                }
            }
        }
    }
}
