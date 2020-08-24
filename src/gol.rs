pub trait Gol {
    fn new() -> Self;
    fn load(&mut self, x: u64, y: u64, gol: impl Gol);
    fn make_cell_alive(&mut self, x: u64, y: u64);
    fn make_cell_dead(&mut self, x: u64, y: u64);
    fn update(&mut self);
    fn iter(&mut self) -> Box<dyn Iterator>;
}
