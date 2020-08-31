#[derive(Debug)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

pub struct CellPoint {
    point: Point,
    is_alive: bool,
}