use std::{collections::HashMap, env};

use crate::anfield::Anfield;

#[derive(Debug, Clone, Default)]
pub struct Robot {
    pub id: u8,
    pub characters: [char; 2],
    pub starting_point: (u8, u8),
}

impl Robot {
    pub fn new(id: u8, ch: [char; 2]) -> Self {
        Self {
            id,
            characters: ch,
            starting_point: (0, 0),
        }
    }

    pub fn set_starting_point(&mut self, x: u8, y: u8) {
        self.starting_point = (x, y)
    }
}

#[derive(Debug, Clone, Default)]

pub struct Piece {
    pub width: u8,
    pub height: u8,
    pub ceils: Vec<Vec<char>>,
}

impl Piece {
    pub fn new(ceils: Vec<Vec<char>>) -> Self {
        Self {
            width: ceils[0].len() as u8,
            height: ceils.len() as u8,
            ceils,
        }
    }
}

pub struct State {
    pub anfield: Anfield,
    pub robot: Robot,
    pub other_robot: Robot,
    pub current_piece: Piece,
    pub prg_name: String,
    pub started: bool,
}

impl State {
    pub fn prog_name() -> String {
        let args: Vec<String> = env::args().collect();
        args.first()
            .unwrap()
            .split_terminator('/')
            .last()
            .unwrap()
            .to_string()
    }
    pub fn new() -> Self {
        Self {
            anfield: Anfield::default(),
            robot: Robot::default(),
            other_robot: Robot::default(),
            current_piece: Piece::default(),
            prg_name: Self::prog_name(),
            started: false,
        }
    }

    pub fn parse(&mut self, lines: Vec<String>) {
        let mut anfield: Anfield = Anfield::new(0, 0);
        let mut robot: Robot;
        let mut other_robot: Robot;

        let mut pidx = 1;

        let mut players: Vec<Robot> = Vec::new();
        let mut pieces_ceils = Vec::new();
        let mut parsing_pieces = false;
        let mut startidx = 0;

        let mut parsing_anfield = false;
        let mut anfield_strtidx: usize = 0;
        for (idx, line) in lines.iter().enumerate() {
            if line.starts_with("$$$") && line.contains(&Self::prog_name()) {
                if line.contains("p1") {
                    robot = Robot::new(1, ['a', '@'])
                } else {
                    pidx = 2;
                    robot = Robot::new(1, ['s', '$'])
                }
                players.push(robot)
            } else if line.starts_with("$$$") {
                if line.contains("p1") {
                    other_robot = Robot::new(2, ['a', '@'])
                } else {
                    other_robot = Robot::new(2, ['s', '$'])
                }
                players.push(other_robot)
            } else if line.starts_with("Anfield") {
                let part = line
                    .trim_matches(|c: char| !c.is_numeric())
                    .split_once(' ')
                    .unwrap();
                let width: u8 = part.0.parse().unwrap();
                let height: u8 = part.1.parse().unwrap();
                anfield = Anfield::new(width, height)
            } else if line.trim().chars().all(char::is_numeric) {
                parsing_anfield = true;
                anfield_strtidx = idx + 1;
            } else if line.starts_with("Piece") {
                parsing_anfield = false;
                parsing_pieces = true;
                startidx = idx + 1;
            } else if line.starts_with("->") {
                parsing_anfield = false;
                parsing_pieces = false;
            }
            if parsing_anfield {
                let l = line.trim_matches(|c: char| !c.is_ascii_punctuation());
                l.char_indices().for_each(|(i, c)| {
                    if c != '.' {
                        for r in players.iter_mut() {
                            if r.characters.contains(&c) {
                                r.set_starting_point(i as u8, (idx - anfield_strtidx) as u8);
                                anfield
                                    .occupation
                                    .insert((i as u8, (idx - anfield_strtidx) as u8), r.id);
                            }
                        }
                    }
                });
            }

            if parsing_pieces {
                let l = line.trim_matches(|c: char| !c.is_ascii_punctuation());
                let ceils: Vec<char> = l.chars().collect();
                pieces_ceils.push(ceils)
            }
        }
        self.anfield = anfield;
        self.robot = players.iter().find(|p| p.id == pidx).unwrap().clone();
        self.other_robot = players.iter().find(|p| p.id != pidx).unwrap().clone();
        self.current_piece = Piece::new(pieces_ceils);
        self.started = true;
    }
}
