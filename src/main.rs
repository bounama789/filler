use std::io::{self, BufRead};

use filler::State;

fn main() {
    let mut input_lines = Vec::new();
    let stdin = io::stdin();
    let mut started = false;
    let mut state = State::new();
    let mut rem_line = u8::MAX;
    loop {
        'read_buffer: for line in stdin.lock().lines() {
            if let Ok(l) = line {
                if l.starts_with("Piece") {
                    let part = l
                        .trim_matches(|c: char| !c.is_numeric())
                        .split_once(' ')
                        .expect("error while spliting");
                    // let width: u8 = part.0.parse().expect("error while parsing");
                    let height: u8 = part.1.parse().expect("error while parsing");
                    rem_line = height;
                    input_lines.push(l);
                    continue;
                }
                input_lines.push(l);
            }
            rem_line -= 1;
            if rem_line < 1 {
                rem_line = u8::MAX;
                break 'read_buffer;
            }
        }

        let mut placed = false;

        state.parse(input_lines.clone());
        input_lines.clear();

        'search: for i in 0..state.anfield.height {
            for j in 0..state.anfield.width {
                if state
                    .anfield
                    .can_place((j, i), &state.robot, &state.current_piece)
                {
                    println!("{} {}", j, i);
                    placed = true;
                    break 'search;
                }
            }
        }
        if !placed {
            println!("{:#?}", state.current_piece.ceils);
            println!("{:#?}", state.anfield);
        }
        // println!(
        //     "{} {}",
        //     state.robot.starting_point.0,
        //     state.robot.starting_point.1 - state.current_piece.height+1
        // );
    }
}
