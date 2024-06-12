use std::{
    cmp::{max, min},
    env,
};

use crate::anfield::{Anfield, Ceil};

#[derive(Debug, Clone, Default)]
pub struct Robot {
    pub id: i32,
    pub characters: [char; 2],
    pub area: ((i32, i32), (i32, i32)),
    pub starting_point: (i32, i32),
}

impl Robot {
    pub fn new(id: i32, ch: [char; 2]) -> Self {
        Self {
            id,
            characters: ch,
            area: ((0, 0), (0, 0)),
            starting_point: (0, 0),
        }
    }

    pub fn set_starting_point(&mut self, x: i32, y: i32) {
        self.starting_point = (x, y);
        self.area = ((x, y), (x, y))
    }

    pub fn update_area(&mut self, pos: &Position, anfield: &Anfield) {
        for i in 0..pos.piece.height {
            for j in 0..pos.piece.width {
                if pos.x + j >= anfield.width {
                    self.area.1 .0 = anfield.width;
                } else if pos.x + j > self.area.1 .0 {
                    self.area.1 .0 = pos.x + j;
                }

                if pos.x + j < self.area.0 .0 {
                    self.area.0 .0 = pos.x + j;
                }

                if pos.y + i >= anfield.height {
                    self.area.1 .1 = anfield.height;
                } else if pos.y + i > self.area.1 .1 {
                    self.area.1 .1 = pos.y + i;
                }
                if pos.y + i < self.area.0 .1 {
                    self.area.0 .1 = pos.y + i;
                }
            }
        }
    }
}

#[derive(Debug, Clone, Default)]

pub struct Piece {
    pub width: i32,
    pub height: i32,
    pub ceils: Vec<Vec<char>>,
}

impl Piece {
    pub fn new(ceils: Vec<Vec<char>>) -> Self {
        // let ceils = Self::remove_empty_rows_and_columns(ceils);
        Self {
            width: ceils[0].len() as i32,
            height: ceils.len() as i32,
            ceils,
        }
    }
}

pub struct State {
    pub anfield: Anfield,
    pub robot: Robot,
    pub opponent: Robot,
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
            opponent: Robot::default(),
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
        let mut opponent = Robot::default();
        // let mut other_robot: Robot;
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
                    opponent = Robot::new(2, ['s', '$']);
                } else {
                    let r = Robot::new(2, ['s', '$']);
                    robot = Some(r);
                    opponent = Robot::new(1, ['a', '@']);
                }
                // players.push(robot)
            } else if line.starts_with("Anfield") {
                let part = line
                    .trim_matches(|c: char| !c.is_numeric())
                    .split_once(' ')
                    .expect("error while spliting");
                let width: i32 = part.0.parse().expect("error while parsing");
                let height: i32 = part.1.parse().expect("error while parsing");
                anfield = Anfield::new(width, height)
            } else if line.trim().chars().all(char::is_numeric) {
                parsing_anfield = true;
                anfield_strtidx = idx + 1;
                continue;
            } else if line.starts_with("Piece") {
                parsing_anfield = false;
                parsing_pieces = true;
                continue;
            }
            if parsing_anfield {
                let l =
                    line.trim_matches(|c: char| !c.is_ascii_punctuation() && c != 'a' && c != 's');
                l.char_indices().for_each(|(i, c)| {
                    if c != '.' {
                        if !self.started {
                            if let Some(p) = robot.to_owned() {
                                if p.characters.contains(&c) {
                                    self.robot = p.to_owned();
                                    self.robot.set_starting_point(
                                        i as i32,
                                        (idx - anfield_strtidx) as i32,
                                    );
                                } else {
                                    self.opponent = opponent.to_owned();
                                    self.opponent.set_starting_point(
                                        i as i32,
                                        (idx - anfield_strtidx) as i32,
                                    );
                                }
                            }
                        }
                        if self.robot.characters.contains(&c) {
                            anfield
                                .occupation
                                .insert((i as i32, (idx - anfield_strtidx) as i32), self.robot.id);
                        } else {
                            let pidx = if self.robot.id == 1 { 2 } else { 1 };
                            anfield
                                .occupation
                                .insert((i as i32, (idx - anfield_strtidx) as i32), pidx);
                        }
                    } else {
                        anfield
                            .occupation
                            .insert((i as i32, (idx - anfield_strtidx) as i32), 0);
                    }
                });
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

pub struct Position {
    pub x: i32,
    pub y: i32,
    pub robot_idx: i32,
    pub piece: Piece,
}

impl Position {
    fn blocking_score(&self, anfield: &Anfield) -> i32 {
        let mut score = 0;
        for i in 0..self.piece.height {
            for j in 0..self.piece.width {
                if self.piece.ceils[i as usize][j as usize] != '.' {
                    let ceil = Ceil::new(self.x + j, self.y + i, self.robot_idx);
                    score += ceil.blocking_potential(anfield)
                }
            }
        }
        // println!("blocking {score}");
        20 * score / 8
    }

    fn expansion_score(&self, anfield: &Anfield) -> i32 {
        let mut score = 0;
        let mut space = 0;
        for i in 0..self.piece.height {
            for j in 0..self.piece.width {
                if self.piece.ceils[i as usize][j as usize] != '.' {
                    space += 1;
                    let ceil = Ceil::new(self.x + j, self.y + i, self.robot_idx);
                    for c in ceil.get_neightboor(anfield) {
                        if c.occupied_by == 0 {
                            score += 1;
                        }
                    }
                }
            }
        }
        // println!("expansion {}",20 * score / (space*8));

        (20 * score / (space * 8)) as i32
    }

    fn edge_proximity(&self, anfield: &Anfield) -> i32 {
        let mut score = 0;
        for i in 0..self.piece.height {
            for j in 0..self.piece.width {
                if self.piece.ceils[i as usize][j as usize] != '.' {
                    let x = self.x + j;
                    let y = self.y + i;
                    let row_dist = min(y, anfield.height - y - 1);
                    let col_dist = min(x, anfield.width - x - 1);
                    score += (row_dist + col_dist) / 2
                }
            }
        }
        // println!(
        //     "edge proximity: {}",
        //     20 * score / max(anfield.height, anfield.width)
        // );

        20 * score / max(anfield.height, anfield.width)
    }

    pub fn surround_score(&self, anfield: &Anfield, robot: &Robot) -> i32 {
        let mut min_distance = f32::MAX;
        let mut score = 0;

        for ((x, y), _) in anfield
            .occupation
            .iter()
            .filter(|&(_, id)| *id != robot.id && *id != 0)
        {
            if let Some(id) = anfield.occupation.get(&(*x, *y)) {
                if *id != 0 && *id != robot.id {
                    let distance = (((self.x as i32 - *x as i32) as f32).powf(2.0)
                        + ((self.y as i32 - *y as i32) as f32).powf(2.0))
                    .sqrt();
                    if distance < min_distance {
                        min_distance = distance;
                    }
                }
                for di in -1..=1 {
                    for dj in -1..=1 {
                        let ni = *y as isize + di;
                        let nj = *x as isize + dj;
                        if ni >= 0
                            && ni < anfield.height as isize
                            && nj >= 0
                            && nj < anfield.width as isize
                        {
                            let ni = ni as i32;
                            let nj = nj as i32;
                            if let Some(id) = anfield.occupation.get(&(nj, ni)) {
                                if *id == 0 {
                                    score += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
        // for i in 0..self.piece.height {
        //     for j in 0..self.piece.width {
        //         for di in -1..=1 {
        //             for dj in -1..=1 {
        //                 let ni = self.y as isize + i as isize + di;
        //                 let nj = self.x as isize + j as isize + dj;
        //                 if ni >= 0
        //                     && ni < anfield.height as isize
        //                     && nj >= 0
        //                     && nj < anfield.width as isize
        //                 {
        //                     let ni = ni as i32;
        //                     let nj = nj as i32;
        //                     if let Some(id) = anfield.occupation.get(&(ni, nj)) {
        //                         if *id == 0 {
        //                             score += 1;
        //                         }
        //                     }
        //                 }
        //             }
        //         }
        //     }
        // }

        // println!("{score}");
        // Normalize encirclement score to be a factor in evaluation
        let normalized_encirclement_score = score as f32 / (anfield.height * anfield.width) as f32;
        // println!("{}",normalized_encirclement_score);
        // Combine min_distance and encirclement_score in the final score
        (min_distance - normalized_encirclement_score) as i32
        // score
    }

    pub fn score(&self, anfield: &Anfield, robot: &Robot) -> f32 {
        let blocking_score = (self.blocking_score(anfield) * 10) as f32;
        let mut score = blocking_score - (self.expansion_score(anfield) as f32)
            + (self.edge_proximity(anfield) as f32);
        // + (self.surround_score(anfield, robot)) as f32;
        // println!("total: {score}");

        if blocking_score > 0.0 {
            score -= self.surround_score(anfield, robot) as f32;
        }

        score
    }
}
