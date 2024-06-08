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
        let ceils = Self::remove_empty_rows_and_columns(ceils);
        Self {
            width: ceils[0].len() as u8,
            height: ceils.len() as u8,
            ceils,
        }
    }

    fn remove_empty_rows_and_columns(mut grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
        // Remove rows that contain only '.'
        grid.retain(|row| row.iter().any(|&c| c != '.'));

        if grid.is_empty() {
            return grid;
        }

        let col_len = grid[0].len();
        let mut col_to_keep = vec![false; col_len];

        for i in 0..col_len {
            for row in &grid {
                if row[i] != '.' {
                    col_to_keep[i] = true;
                    break;
                }
            }
        }

        for row in &mut grid {
            let mut new_row = Vec::new();
            for (i, &c) in row.iter().enumerate() {
                if col_to_keep[i] {
                    new_row.push(c);
                }
            }
            *row = new_row;
        }
        grid
    }
}

pub struct State {
    pub anfield: Anfield,
    pub robot: Robot,
    pub current_piece: Piece,
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
            current_piece: Piece::default(),
            started: false,
        }
    }

    pub fn parse(&mut self, lines: Vec<String>) {
        let mut anfield: Anfield = Anfield::new(0, 0);
        let mut robot = if self.started {
            Some(self.robot.clone())
        } else {
            None
        };
        // let mut other_robot: Robot;

        let mut pidx = 1;

        // let mut players: Vec<Robot> = Vec::new();
        let mut pieces_ceils = Vec::new();
        let mut parsing_pieces = false;

        let mut parsing_anfield = false;
        let mut anfield_strtidx: usize = 0;
        for (idx, line) in lines.iter().enumerate() {
            if line.starts_with("$$$") {
                if line.contains("p1") {
                    let r = Robot::new(1, ['a', '@']);
                    robot = Some(r);
                } else {
                    pidx = 2;
                    let r = Robot::new(2, ['s', '$']);
                    robot = Some(r)
                }
                // players.push(robot)
            } else if line.starts_with("Anfield") {
                let part = line
                    .trim_matches(|c: char| !c.is_numeric())
                    .split_once(' ')
                    .expect("error while spliting");
                let width: u8 = part.0.parse().expect("error while parsing");
                let height: u8 = part.1.parse().expect("error while parsing");
                anfield = Anfield::new(width, height)
            } else if line.trim().chars().all(char::is_numeric) {
                parsing_anfield = true;
                anfield_strtidx = idx + 1;
                continue;
            } else if line.starts_with("Piece") {
                parsing_anfield = false;
                parsing_pieces = true;
                continue;
            } else if line.starts_with("->") {
                parsing_anfield = false;
                parsing_pieces = false;
                continue;
            }
            if parsing_anfield {
                let l = line.trim_matches(|c: char| !c.is_ascii_punctuation());
                if let Some(mut p) = robot.to_owned() {
                    l.char_indices().for_each(|(i, c)| {
                        if c != '.' {
                            if !self.started {
                                if p.characters.contains(&c) {
                                    p.set_starting_point(i as u8, (idx - anfield_strtidx) as u8);
                                    anfield
                                        .occupation
                                        .insert((i as u8, (idx - anfield_strtidx) as u8), p.id);
                                }
                                self.robot = p.to_owned();
                            } else {
                                if self.robot.characters.contains(&c) {
                                    anfield
                                        .occupation
                                        .insert((i as u8, (idx - anfield_strtidx) as u8), p.id);
                                }
                            }
                        }
                    });
                }
            }

            if parsing_pieces {
                let l = line.trim();
                let ceils: Vec<char> = l.chars().collect();
                pieces_ceils.push(ceils)
            }
        }
        self.anfield = anfield;

        // println!("{:?}",self.anfield);

        // println!("{:?}",self.robot);
        // println!("{:?}",lines);

        // println!("{:?}",pieces_ceils);
        self.current_piece = Piece::new(pieces_ceils);
        // println!("{:?}", self.current_piece);

        self.started = true;
    }
}
