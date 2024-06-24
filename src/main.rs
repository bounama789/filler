use std::{
    env,
    io::{self, BufRead},
};

use filler::{flag, State};

fn main() {
    let mut input_lines = Vec::new();
    let stdin = io::stdin();
    let mut state = State::new();
    let mut rem_line = i32::MAX;

    let args: Vec<String> = env::args().collect();
    if let Some(flag) = args.get(1) {
        if flag.trim() == "-d" || flag.trim() == "--debug" {
            unsafe { flag::DEBUG = true };
            println!("\nMode: DEBUG")
        }
    }

    loop {
        'read_buffer: for line in stdin.lock().lines() {
            if let Ok(l) = line {
                if l.starts_with("Piece") {
                    let part = l
                        .trim_matches(|c: char| !c.is_numeric())
                        .split_once(' ')
                        .expect("error while spliting");
                    // let width: i32 = part.0.parse().expect("error while parsing");
                    let height: i32 = part.1.parse().expect("error while parsing");
                    rem_line = height;
                    input_lines.push(l);
                    continue;
                }
                input_lines.push(l);
            }
            rem_line -= 1;
            if rem_line < 1 {
                rem_line = i32::MAX;
                break 'read_buffer;
            }
        }

        state.parse(input_lines.clone());
        input_lines.clear();

        state.anfield.update_opp_occupation(&state.robot);

        let mut positions: Vec<_> = state
            .anfield
            .potential_positions(&state.current_piece, &state.robot)
            .into_iter()
            .collect();

        positions.sort_by(|a, b| a.1.total_cmp(&b.1));

        if let Some(p) = positions.last() {
            println!("{} {}", p.0.x, p.0.y);
            // state.robot.update_area(&p.0, &state.anfield)
        } else {
            println!("0 0");
        }
    }
}
