use std::collections::HashMap;

use crate::universe::universe::CellUniverse;

#[derive(Debug)]
pub struct UpdateAction {
    pub make_alive: bool,
    pub reason: Option<String>,
}

#[derive(Debug)]
pub struct UpdateDelta {
    pub actions: Vec<((usize, usize), UpdateAction)>
}

impl UpdateDelta {
    pub fn new(universe: &impl CellUniverse) -> UpdateDelta {
        let mut delta = UpdateDelta { actions: vec![] };
        let mut neighbour_cnt: HashMap<(usize, usize), u8> = Default::default();
        for neighbour in universe.iter_neighbours() {
            match neighbour_cnt.get_mut(&(neighbour.x, neighbour.y)) {
                None => {
                    neighbour_cnt.insert((neighbour.x, neighbour.y), 1);
                }
                Some(pt_count) => {
                    *pt_count += 1
                }
            }
        }
        // println!("neighbour_cnt = {:?}", neighbour_cnt);
        // process any cell which has a neighbour
        for ((x, y), neighbour_count) in &neighbour_cnt {
            let is_alive = universe.is_cell_alive(*x, *y);
            if is_alive {
                if (2..4).contains(neighbour_count) {
                    // no change, keep alive
                } else {
                    delta.add_set_dead(*x, *y);
                }
            } else {
                if *neighbour_count == 3 {
                    delta.add_set_alive(*x, *y);
                } else {
                    // no change, keep dead
                }
            }
        }
        // cells with no neighbour should die
        for alive_cell in universe.iter_alive() {
            if false == neighbour_cnt.contains_key(&(alive_cell.x, alive_cell.y)) {
                // println!("..killing longer {:?}", alive_cell);
                delta.add_set_dead(alive_cell.x, alive_cell.y);
            }
        }
        delta
    }

    pub fn add_set_state_and_reason(&mut self, x: usize, y: usize, reason: String) {
        self.actions.push(((x, y), UpdateAction { make_alive: true, reason: Some(reason) }))
    }

    pub fn add_set_alive(&mut self, x: usize, y: usize) {
        self.actions.push(((x, y), UpdateAction { make_alive: true, reason: None }))
    }

    pub fn add_set_dead(&mut self, x: usize, y: usize) {
        self.actions.push(((x, y), UpdateAction { make_alive: false, reason: None }))
    }
}
