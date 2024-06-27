use std::io::{self, BufRead};
use std::sync::Mutex;

use filler::{Anfield, Robot};
use ggez::conf::WindowMode;
use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color, DrawParam, Mesh, MeshBuilder};
use ggez::{Context, ContextBuilder, GameResult};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use visualizer::Grid;

fn main() {
    // Make a Context.
    let (mut ctx, event_loop) = ContextBuilder::new("my_game", "Cool Game Author")
        .window_mode(WindowMode {
            fullscreen_type: ggez::conf::FullscreenType::True,
            ..Default::default()
        })
        .build()
        .expect("aieee, could not create ggez context!");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let my_game = VState::new(&mut ctx);

    // Run!
    event::run(ctx, event_loop, my_game);
}

struct VState {
    pub robot1: Robot,
    pub robot2: Robot,
    pub anfield: Anfield,
    pub grid: Grid,
    pub started: bool,
}

impl VState {
    pub fn new(_ctx: &mut Context) -> VState {
        // Load/create resources such as images here.
        VState {
            robot1: Robot::default(),
            robot2: Robot::default(),
            anfield: Anfield::default(),
            grid: Grid::new(),
            started: false,
        }
    }

    pub fn fill_grid(&self) -> MeshBuilder {
        let occ = self.anfield.occupation.to_owned();
        let cell_size = self.grid.cell_size;
        let hx = cell_size.0 / 2.0;
        let hy = cell_size.1 / 2.0;

        let mesh_builder = Mutex::new(MeshBuilder::new());

        occ.into_iter().for_each(|((col, row), id)| {
            let mut mb = mesh_builder.lock().unwrap();
            if id != 0 {
                let x = col as f32 * cell_size.0 - cell_size.0;
                let y = row as f32 * cell_size.1 - cell_size.1;

                let color = if id == 1 { Color::GREEN } else { Color::YELLOW };

                mb.rectangle(
                    graphics::DrawMode::fill(),
                    graphics::Rect::new(x, y, cell_size.0, cell_size.1),
                    color,
                )
                .unwrap();
            }
        });
        let mb = mesh_builder.lock().unwrap();

        mb.to_owned()
    }

    pub fn parse(&mut self, lines: Vec<String>) {
        // let mut logger = Logger::new("log.txt").unwrap();
        // logger.write("parsing...\n");
        let mut anfield: Anfield = Anfield::new(0, 0);
        let robot1 = if self.started {
            self.robot1.clone()
        } else {
            Robot::new(1, ['a', '@'])
        };
        let robot2 = if self.started {
            self.robot2.clone()
        } else {
            Robot::new(2, ['s', '$'])
        };
        let mut pieces_cells = Vec::new();
        let mut parsing_pieces = false;

        let mut parsing_anfield = false;
        let mut anfield_strtidx: usize = 0;
        for (idx, line) in lines.iter().enumerate() {
            if line.starts_with("Anfield") {
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
                        if self.robot1.characters.contains(&c) {
                            anfield
                                .occupation
                                .insert((i as i32, (idx - anfield_strtidx) as i32), self.robot1.id);
                        } else {
                            anfield
                                .occupation
                                .insert((i as i32, (idx - anfield_strtidx) as i32), self.robot2.id);
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
            self.robot1 = robot1;
            self.robot2 = robot2
        }

        // logger.write(&format!("current piece\n{:#?}",pieces_cells));

        // self.current_piece = Piece::new(pieces_cells);

        self.started = true;
        // logger.write("end parsing");
    }
}

impl EventHandler for VState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        let mut input_lines = Vec::new();
        let stdin = io::stdin();
        let mut rem_line = i32::MAX;

        let mut n =0;

        'read_buffer: for line in stdin.lock().lines() {
            if let Ok(l) = line {
                n+=1;
                if l.starts_with("Piece") {
                    let part = l
                        .trim_matches(|c: char| !c.is_numeric())
                        .split_once(' ')
                        .expect("error while spliting");
                    let height: i32 = part.1.parse().expect("error while parsing");
                    rem_line = height;
                    input_lines.push(l);
                    continue;
                }
                input_lines.push(l);
            }
            rem_line -= 1;
            if rem_line < 1 {
                break 'read_buffer;
            }
        }

        if n < input_lines.len() - 1 {
            for i in n..input_lines.len() {
                println!("{}", input_lines[i]);
            }
            panic!()
        }

        self.parse(input_lines.clone());
        if self.started {
            let c = self.anfield.width as usize | 2;
            let r = self.anfield.height as usize | 2;

            let w = (800 / c) as f32;
            let h = (600 / r) as f32;

            self.grid.init((w, h), r, c);
        }
        input_lines.clear();
        self.started = true;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::WHITE);
        if let Some(g) = self.grid.build() {
            let mesh_data = Mesh::from_data(ctx, g.build());
            canvas.draw(&mesh_data, DrawParam::default());
        }
        let mesh_data = Mesh::from_data(ctx, self.fill_grid().build());
        canvas.draw(&mesh_data, DrawParam::default());
        canvas.finish(ctx)
    }
}
