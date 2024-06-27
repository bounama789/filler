use std::collections::HashMap;


use crate::{
    process::{Piece, Robot},
    Position,
};

#[derive(Debug,Clone, Default)]
pub struct Anfield {
    pub width: i32,
    pub height: i32,
    pub occupation: HashMap<(i32, i32), i32>,
    pub opp_occupation: Vec<Cell>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Cell {
    pub x: i32,
    pub y: i32,
    pub occupied_by: i32,
}

impl Anfield {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            width,
            height,
            occupation: HashMap::new(),
            opp_occupation: Vec::new(),
        }
    }

    pub fn update_opp_occupation(&mut self, robot: &Robot) {
        self.opp_occupation = self
            .occupation
            .iter()
            .filter(|&(c, id)| {
                let cell = Cell::new(c.0, c.1, *id);
                let n = cell.get_neightboor(&self);
                let c = n.iter().filter(|c| c.occupied_by == 0).count();
                c > 2 && (*id != robot.id && *id != 0)
            })
            .map(|(&(x, y), id)| Cell::new(x, y, *id))
            .collect()
    }

    pub fn can_place(&self, coord: (i32, i32), robot: &Robot, piece: &Piece) -> bool {
        let mut touch = 0;
        for i in 0..piece.height {
            for j in 0..piece.width {
                if piece.cells[i as usize][j as usize] != '.' {
                    if coord.0 + j >= self.width || &coord.1 + i >= self.height {
                        return false;
                    }
                    if let Some(c) = self.occupation.get(&(&coord.0 + j, &coord.1 + i)) {
                        if *c == robot.id {
                            touch += 1
                        } else if *c != 0 {
                            return false;
                        }
                    }
                }
            }
        }
        touch == 1
    }

    pub fn potential_positions(&self, piece: &Piece, robot: &Robot) -> HashMap<Position, f32> {
        let mut positions = HashMap::new();
        (0..self.height).into_iter().for_each(|i| {
            (0..self.width).into_iter().for_each(|j| {
                if self.can_place((j, i), robot, piece) {
                    let p = Position {
                        x: j,
                        y: i,
                        robot_idx: robot.id,
                        piece: piece.clone(),
                    };
                    positions.insert(p.clone(), p.score(self, robot));
                }
            })
        });
        positions
    }
}

impl Cell {
    pub fn new(x: i32, y: i32, robot_idx: i32) -> Self {
        Self {
            x,
            y,
            occupied_by: robot_idx,
        }
    }

    pub fn blocking_potential(&self, anfield: &Anfield) -> i32 {
        let mut blocking_score = 0;
        for cell in self.get_neightboor(anfield) {
            if cell.occupied_by != self.occupied_by && cell.occupied_by != 0 {
                blocking_score += 20
                    * cell
                        .get_neightboor(anfield)
                        .iter()
                        .filter(|c| c.occupied_by == 0)
                        .count()
                    / 8;
            }
        }
        (blocking_score / 8) as i32
    }

    pub fn get_neightboor(&self, anfield: &Anfield) -> Vec<Cell> {
        let mut neighboors = Vec::new();

        for di in -1..=1 {
            for dj in -1..=1 {
                let ni = self.y as isize + di;
                let nj = self.x as isize + dj;
                if let Some(idx) = anfield.occupation.get(&(nj as i32, ni as i32)) {
                    neighboors.push(Cell::new(nj as i32, ni as i32, *idx));
                }
            }
        }

        neighboors
    }
}
