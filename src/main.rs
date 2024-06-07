use std::io::{self, BufRead};

use filler::State;

fn main() {
    let mut input_lines = Vec::new();
    let stdin = io::stdin();
    let mut started = false;
    let mut state = State::new();
    loop {
        'read_buffer: for line in stdin.lock().lines() {
            if let Ok(l) = line {
                if l.trim() == "" {
                    break 'read_buffer;
                } else {
                    input_lines.push(l);
                }
            }
        }


        if !started {
            state.parse(input_lines.clone());
            input_lines.clear();
            started = true;
        } else {
            println!(
                "{} {}",
                state.robot.starting_point.0, state.robot.starting_point.1
            );
        }
    }
}
