use std::collections::HashMap;

use crate::{
    process::{Piece, Robot},
    Position,
};

#[derive(Debug, Default)]
pub struct Anfield {
    pub width: i32,
    pub height: i32,
    pub occupation: HashMap<(i32, i32), i32>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Ceil {
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
        }
    }

    pub fn can_place(&self, coord: (i32, i32), robot: &Robot, piece: &Piece) -> bool {
        let mut touch = 0;
        for i in 0..piece.height {
            for j in 0..piece.width {
                if piece.ceils[i as usize][j as usize] != '.' {
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

    pub fn potential_positions(&self, piece: &Piece, robot: &Robot) -> Vec<Position> {
        let mut positions = Vec::new();
        for i in 0..self.height {
            for j in 0..self.width {
                if self.can_place((j, i), robot, piece) {
                    let p = Position {
                        x: j,
                        y: i,
                        robot_idx: robot.id,
                        piece: piece.clone(),
                    };
                    positions.push(p)
                }
            }
        }
        positions.sort_by(|a, b| a.score(self, robot).total_cmp(&b.score(self, robot)));
        positions
    }

    pub fn get_opponent_border(&self, robot: &Robot) -> Vec<Ceil> {
        let mut result = Vec::new();
        let opponent_occupation: Vec<Ceil> = self
            .occupation
            .iter()
            .filter(|&(_, id)| *id != robot.id)
            .map(|c: (&(i32, i32), &i32)| Ceil::new(c.0 .0, c.0 .1, *c.1))
            .collect();

        for oc in opponent_occupation {
            for ceil in oc.get_neightboor(self) {
                if ceil.occupied_by == 0 {
                    result.push(ceil)
                }
            }
        }
        result
    }
}

impl Ceil {
    pub fn new(x: i32, y: i32, robot_idx: i32) -> Self {
        Self {
            x,
            y,
            occupied_by: robot_idx,
        }
    }

    pub fn blocking_potential(&self, anfield: &Anfield) -> i32 {
        let mut blocking_score = 0;
        for ceil in self.get_neightboor(anfield) {
            if ceil.occupied_by != self.occupied_by && ceil.occupied_by != 0 {
                blocking_score += 20 * ceil
                    .get_neightboor(anfield)
                    .iter()
                    .filter(|c| c.occupied_by == 0)
                    .count() / 8;
            }
        }
        (20 * blocking_score / (8*20)) as i32
    }

    pub fn get_neightboor(&self, anfield: &Anfield) -> Vec<Ceil> {
        let mut neighboors = Vec::new();

        if let Some(idx) = anfield.occupation.get(&(self.x - 1, self.y - 1)) {
            neighboors.push(Ceil::new(self.x - 1, self.y - 1, *idx));
        }
        if let Some(idx) = anfield.occupation.get(&(self.x, self.y - 1)) {
            neighboors.push(Ceil::new(self.x, self.y - 1, *idx));
        }
        if let Some(idx) = anfield.occupation.get(&(self.x + 1, self.y - 1)) {
            neighboors.push(Ceil::new(self.x + 1, self.y - 1, *idx));
        }

        if let Some(idx) = anfield.occupation.get(&(self.x - 1, self.y)) {
            neighboors.push(Ceil::new(self.x - 1, self.y, *idx));
        }
        if let Some(idx) = anfield.occupation.get(&(self.x + 1, self.y)) {
            neighboors.push(Ceil::new(self.x + 1, self.y, *idx));
        }

        if let Some(idx) = anfield.occupation.get(&(self.x - 1, self.y + 1)) {
            neighboors.push(Ceil::new(self.x - 1, self.y + 1, *idx));
        }
        if let Some(idx) = anfield.occupation.get(&(self.x, self.y + 1)) {
            neighboors.push(Ceil::new(self.x, self.y + 1, *idx));
        }
        if let Some(idx) = anfield.occupation.get(&(self.x + 1, self.y + 1)) {
            neighboors.push(Ceil::new(self.x + 1, self.y + 1, *idx));
        }

        neighboors
    }
    // pub fn distance_to(other_ceil: &Ceil) -> i32 {}
}
