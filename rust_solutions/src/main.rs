mod load_input;
use std::env::args;

fn main() {
    let args = args();
    // Expect rust_solutions day 1 [--debug]
    let args_vec: Vec<String> = args.skip(0).map(|x| x.to_string()).collect::<Vec<String>>();
    let day: usize = match args_vec.get(1) {
        Some(raw_day) => {
            match raw_day.parse::<usize>() {
                Ok(day) => day,
                Err(_) => {
                    eprintln!("Invalid day. Expected rust_solutions day N [--debug]");
                    return;
                }
            }
        }
        None => {
            eprintln!("Expected rust_solutions day N [--debug]");
            return;
        }
    };
    
}
