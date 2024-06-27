use std::{
    cmp::{max, min},
    env,
    sync::Mutex,
};

use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::{
    anfield::{Anfield, Cell},
    logger::console_log,
};

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
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]

pub struct Piece {
    pub width: i32,
    pub height: i32,
    pub cells: Vec<Vec<char>>,
}

impl Piece {
    pub fn new(cells: Vec<Vec<char>>) -> Self {
        let mut w = 0;
        if !cells.is_empty() {
            w = cells[0].len();
        }
        Self {
            width: w as i32,
            height: cells.len() as i32,
            cells,
        }
    }
}

#[derive(Debug, Clone)]
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
        // let mut logger = Logger::new("log.txt").unwrap();
        // logger.write("parsing...\n");
        let mut anfield: Anfield = Anfield::new(0, 0);
        let mut robot = if self.started {
            Some(self.robot.clone())
        } else {
            None
        };
        let mut opponent = Robot::default();
        let mut pieces_cells = Vec::new();
        let mut parsing_pieces = false;

        let mut parsing_anfield = false;
        let mut anfield_strtidx: usize = 0;
        for (idx, line) in lines.iter().enumerate() {
            if line.starts_with("$$$") {
                if line.contains("p1") && line.contains(&Self::prog_name()) {
                    let r = Robot::new(1, ['a', '@']);
                    robot = Some(r);
                    opponent = Robot::new(2, ['s', '$']);
                } else {
                    let r = Robot::new(2, ['s', '$']);
                    robot = Some(r);
                    opponent = Robot::new(1, ['a', '@']);
                }
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
                // logger.write("parsing anfield...\n");

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
                // logger.write("parsing current piece...\n");

                let l = line.trim();
                let cells: Vec<char> = l.chars().collect();
                pieces_cells.push(cells)
            }
        }

        if anfield.width != 0 {
            self.anfield = anfield;
        }

        // logger.write(&format!("current piece\n{:#?}",pieces_cells));

        self.current_piece = Piece::new(pieces_cells);
        let ((x, y), (x1, y1)) = self.robot.area;
        self.robot.area = (
            (x - self.current_piece.width, y - self.current_piece.height),
            (
                x1 + self.current_piece.width,
                y1 + self.current_piece.height,
            ),
        );
        self.started = true;
        // logger.write("end parsing");
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Position {
    pub x: i32,
    pub y: i32,
    pub robot_idx: i32,
    pub piece: Piece,
}

impl Position {
    fn blocking_score(&self, anfield: &Anfield, coord: (i32, i32)) -> i32 {
        let x = coord.0;
        let y = coord.1;
        let cell = Cell::new(self.x + x, self.y + y, self.robot_idx);
        cell.blocking_potential(anfield)
    }

    fn edge_proximity(&self, anfield: &Anfield, coord: (i32, i32)) -> i32 {
        let x = self.x + coord.0;
        let y = self.y + coord.1;
        let row_dist = min(y, anfield.height - y - 1);
        let col_dist = min(x, anfield.width - x - 1);
        (row_dist + col_dist) / 2
        // console_log(format!(
        //     "edge proximity: {}",
        //     20 * score / max(anfield.height, anfield.width)
        // ));
    }

    pub fn surround_score(&self, anfield: &Anfield, robot: &Robot) -> i32 {
        // let mut logger = Logger::new("log.txt").unwrap();
        // logger.write("calculating surround score");
        let mut min_distance = Mutex::new(f32::MAX);
        let mut score = Mutex::new(0);

        anfield
            .opp_occupation
            .clone()
            .into_par_iter()
            .for_each(|Cell { x, y, .. }| {
                let mut m = min_distance.lock().unwrap();
                let mut s = score.lock().unwrap();

                if let Some(id) = anfield.occupation.get(&(x, y)) {
                    if *id != 0 && *id != robot.id {
                        let distance = (((self.x as i32 - x as i32) as f32).powf(2.0)
                            + ((self.y as i32 - y as i32) as f32).powf(2.0))
                        .abs()
                        .sqrt();
                        if distance < *m {
                            *m = distance.into();
                        }
                    }
                    for di in -1..=1 {
                        for dj in -1..=1 {
                            let ni = y as isize + di;
                            let nj = x as isize + dj;
                            if ni >= 0
                                && ni < anfield.height as isize
                                && nj >= 0
                                && nj < anfield.width as isize
                            {
                                let ni = ni as i32;
                                let nj = nj as i32;
                                if let Some(id) = anfield.occupation.get(&(nj, ni)) {
                                    if *id == 0 {
                                        *s += 1;
                                    }
                                }
                            }
                        }
                    }
                }
            });
        let m = min_distance.get_mut().unwrap();
        let s = score.get_mut().unwrap();

        console_log(format!("surround: {}", (*m - *s as f32).abs()));
        (*m - (*s) as f32).abs() as i32
        // score
    }

    pub fn score(&self, anfield: &Anfield, robot: &Robot) -> f32 {
        // let mut logger = Logger::new("log.txt").unwrap();

        let mut blocking_score = 0;
        let mut edge_proximity = 0;
        // logger.write(&format!("calculating score\n"));

        for i in 0..self.piece.height {
            for j in 0..self.piece.width {
                if self.piece.cells[i as usize][j as usize] != '.' {
                    blocking_score += self.blocking_score(anfield, (j, i));
                    edge_proximity += self.edge_proximity(anfield, (j, i));
                }
            }
        }
        let blocking_score = (blocking_score * 10) as f32; // *10 because it's more important
        let edge_proximity = 20 * edge_proximity / max(anfield.height, anfield.width);

        let mut score = blocking_score + edge_proximity as f32;
        score += (self.surround_score(anfield, robot)) as f32 * 2.0;
        console_log(format!("blocking_score: {blocking_score}"));
        console_log(format!("edge_proximity: {edge_proximity}"));

        console_log(format!("total: {score}"));

        console_log(format!("position: {:?}", (self.x, self.y)));
        score
    }
}
