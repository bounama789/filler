use std::collections::HashMap;

use crate::process::{Piece, Robot};

#[derive(Debug,Default)]
pub struct Anfield {
    pub width: u8,
    pub height: u8,
    pub occupation: HashMap<(u8, u8), u8>,
}

impl Anfield {
    pub fn new(width: u8, height: u8) -> Self {
        Self {
            width,
            height,
            occupation: HashMap::new(),
        }
    }

    pub fn can_place(&self, coord: (u8, u8), robot: &Robot, piece: &Piece) -> bool {
        let mut touch = 1;
        if let Some(c) = self.occupation.get(&coord) {
            if *c == robot.id {
                touch += 1;
            } else {
                return false;
            }
        }
        for i in 0..piece.height {
            for j in 0..piece.width {
                if let Some(c) = self.occupation.get(&(&coord.0 + i, &coord.1 + j)) {
                    if *c == robot.id && touch < 1 {
                        touch += 1
                    } else {
                        return false;
                    }
                }
            }
        }
        touch == 1
    }
}
