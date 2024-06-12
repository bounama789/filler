use std::io::{self, BufRead};

use filler::State;

fn main() {
    let mut input_lines = Vec::new();
    let stdin = io::stdin();
    let mut state = State::new();
    let mut rem_line = i32::MAX;
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

        let mut positions = state
            .anfield
            .potential_positions(&state.current_piece, &state.robot);

        // if state.robot.area.0.1 > state.anfield.height/2 {
        //     positions.sort_by_key(|p|p.y);
        // }


        if let Some(p) = positions.last() {
            println!("{} {}",p.x,p.y);
            state.robot.update_area(p, &state.anfield)
            // println!("{:#?}", state.current_piece.ceils);
            // println!("{:#?}", state.anfield);
        } else {
            println!("0 0");
        }
        // println!(
        //     "{} {}",
        //     state.robot.starting_point.0,
        //     state.robot.starting_point.1 - state.current_piece.height+1
        // );
    }
}
