use std::io::{self, BufRead};

use filler::State;

fn main() {
    let mut input_lines = Vec::new();
    let stdin = io::stdin();
    let mut started = false;
    let mut state = State::new();
    for line in stdin.lock().lines() {
        if let Ok(l) = line {
            if l == "" {
                if !started {
                    state.parse(input_lines.clone());
                    started=true;
                } else {
                    println!("{} {}",state.robot.starting_point.0,state.robot.starting_point.1);
                }
                input_lines.clear();
            }else{
                input_lines.push(l);
            }
        }
    }
}
