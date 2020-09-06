use crate::common::Point;

pub trait CellUniverse {
    // fn new() -> Self;
    fn set_cell_alive(&mut self, x: usize, y: usize);
    fn set_cell_dead(&mut self, x: usize, y: usize);
    fn set_cell_state(&mut self, x: usize, y: usize, alive: bool);
    fn is_cell_alive(&self, x: usize, y: usize) -> bool;
    fn is_cell_dead(&self, x: usize, y: usize) -> bool;

    /**
    Returns iterator over Points representing alive cells
     **/
    fn iter_alive<'a>(&'a self) -> Box<dyn Iterator<Item=Point> + 'a>;

    /**
    Returns iterator over Points representing neighbours of alive cells
     **/
    fn iter_neighbours<'a>(&'a self) -> Box<dyn Iterator<Item=Point> + 'a>;

    /**
    Inserts alive cells from iterator relatively to coordinates (x, y)
     **/
    fn insert<'a>(&'a mut self, x: usize, y: usize, cells: Box<dyn Iterator<Item=Point> + 'a>);
}
