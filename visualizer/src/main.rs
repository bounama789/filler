use std::io::{self, BufRead};

use filler::{Anfield, Robot};
use ggez::conf::WindowMode;
use ggez::event::{self, EventHandler};
use ggez::glam::Vec2;
use ggez::graphics::{
    self, Canvas, Color, DrawParam, Mesh, MeshBuilder, PxScale, Text, TextFragment,
};
use ggez::{Context, ContextBuilder, GameResult};
use visualizer::Grid;

fn main() {
    let (mut ctx, event_loop) = ContextBuilder::new("filler_visualizer", "bcoulibal")
        .window_mode(WindowMode {
            fullscreen_type: ggez::conf::FullscreenType::True,
            maximized: true,
            ..Default::default()
        })
        .build()
        .expect("aieee, could not create ggez context!");

    let my_game = VState::new(&mut ctx);

    event::run(ctx, event_loop, my_game);
}

struct VState {
    pub robot1: Robot,
    pub robot2: Robot,
    pub anfield: Anfield,
    pub grid: Grid,
    pub started: bool,
    pub winner: Option<u8>,
}

impl VState {
    pub fn new(_ctx: &mut Context) -> VState {
        VState {
            robot1: Robot::default(),
            robot2: Robot::default(),
            anfield: Anfield::default(),
            grid: Grid::new(),
            started: false,
            winner: None,
        }
    }

    pub fn draw_scores(&self, canvas: &mut Canvas) {
        let text1 = Text::new(TextFragment {
            text: self.robot1.score.to_string(),
            color: Some(Color::GREEN),
            font: Some("LiberationMono-Regular".into()),
            scale: Some(PxScale::from(30.0)),

            ..Default::default()
        });

        let text2 = Text::new(TextFragment {
            text: self.robot2.score.to_string(),
            color: Some(Color::YELLOW),
            font: Some("LiberationMono-Regular".into()),
            scale: Some(PxScale::from(30.0)),

            ..Default::default()
        });
        let x = self.grid.rect.x + self.grid.rect.w / 2.0;
        let y = self.grid.rect.y + self.grid.rect.h + 20.0;
        canvas.draw(&text1, Vec2::new(x, y));
        let y = y + 20.0;
        canvas.draw(&text2, Vec2::new(x, y));

        if let Some(id) = self.winner {
            let text3 = Text::new(TextFragment {
                text: format!("player{} won!", id),
                color: if id == 2 {
                    Some(Color::YELLOW)
                } else {
                    Some(Color::GREEN)
                },
                font: Some("LiberationMono-Regular".into()),
                scale: Some(PxScale::from(30.0)),

                ..Default::default()
            });
            let y = y + 20.0;
            let x = x - 60.0;
            canvas.draw(&text3, Vec2::new(x, y));
        }
    }

    pub fn fill_grid(&self) -> MeshBuilder {
        let occ = self.anfield.occupation.to_owned();
        let cell_size = self.grid.cell_size;

        let mesh_builder = &mut MeshBuilder::new();

        occ.into_iter().for_each(|((col, row), id)| {
            if id != 0 {
                let x = self.grid.rect.x + col as f32 * cell_size.0;
                let y = self.grid.rect.y + row as f32 * cell_size.1;

                let color = if id == 1 { Color::GREEN } else { Color::YELLOW };

                mesh_builder
                    .rectangle(
                        graphics::DrawMode::fill(),
                        graphics::Rect::new(x, y, cell_size.0, cell_size.1),
                        color,
                    )
                    .unwrap();
            }
        });
        mesh_builder.to_owned()
    }

    pub fn parse(&mut self, lines: Vec<String>) {
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
            } else if line.contains("won") {
                if line.contains("Player1") {
                    self.winner = Some(1);
                } else {
                    self.winner = Some(2);
                }
            }
            if parsing_anfield {
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
        self.started = true;
    }
}

impl EventHandler for VState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        let mut input_lines = Vec::new();
        let stdin = io::stdin();
        let mut rem_line = i32::MAX;

        'read_buffer: for line in stdin.lock().lines() {
            if let Ok(l) = line {
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

        self.parse(input_lines.clone());
        if self.started {
            let c = self.anfield.width as usize | 2;
            let r = self.anfield.height as usize | 2;

            let size = _ctx.gfx.size();

            self.grid.init(r, c, size);
        }
        input_lines.clear();
        self.started = true;
        self.robot1.update_score(&self.anfield);
        self.robot2.update_score(&self.anfield);

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::from_rgb(41, 45, 60));
        if let Some(g) = self.grid.build() {
            let mesh_data = Mesh::from_data(ctx, g.build());
            canvas.draw(&mesh_data, DrawParam::default());
        }
        let mesh_data = Mesh::from_data(ctx, self.fill_grid().build());
        canvas.draw(&mesh_data, DrawParam::default());
        self.draw_scores(&mut canvas);
        canvas.finish(ctx)
    }
}
