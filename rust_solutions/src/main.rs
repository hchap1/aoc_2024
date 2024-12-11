mod load_input;
use crate::load_input::load_input;
use std::env::args;
use std::time::{Instant, Duration};

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
    
    let input: Vec<String> = match load_input(day) {
        Ok(input) => input,
        Err(e) => {
            eprintln!("Error: {e:?}");
            return;
        }
    };
    
    let debug: bool = args_vec.contains(&String::from("--debug"));
    let a_time = Instant::now();

    let a: i64 = match day {
        1 => rust_solutions::day_1::solve_a(&input, debug) as i64,
        2 => rust_solutions::day_2::solve_a(&input, debug) as i64,
        3 => rust_solutions::day_3::solve_a(&input, debug) as i64,
        4 => rust_solutions::day_4::solve_a(&input, debug) as i64,
        5 => rust_solutions::day_5::solve_a(&input, debug) as i64,
        6 => rust_solutions::day_6::solve_a(&input, debug) as i64,
        7 => rust_solutions::day_7::solve_a(&input, debug) as i64,
        8 => rust_solutions::day_8::solve_a(&input, debug) as i64,
        9 => rust_solutions::day_9::solve_a(&input, debug) as i64,
        10 => rust_solutions::day_10::solve_a(&input, debug) as i64,
        11 => rust_solutions::day_11::solve_a(&input, debug) as i64,
        12 => rust_solutions::day_12::solve_a(&input, debug) as i64,
        13 => rust_solutions::day_13::solve_a(&input, debug) as i64,
        14 => rust_solutions::day_14::solve_a(&input, debug) as i64,
        15 => rust_solutions::day_15::solve_a(&input, debug) as i64,
        16 => rust_solutions::day_16::solve_a(&input, debug) as i64,
        17 => rust_solutions::day_17::solve_a(&input, debug) as i64,
        18 => rust_solutions::day_18::solve_a(&input, debug) as i64,
        19 => rust_solutions::day_19::solve_a(&input, debug) as i64,
        20 => rust_solutions::day_20::solve_a(&input, debug) as i64,
        21 => rust_solutions::day_21::solve_a(&input, debug) as i64,
        22 => rust_solutions::day_22::solve_a(&input, debug) as i64,
        23 => rust_solutions::day_23::solve_a(&input, debug) as i64,
        24 => rust_solutions::day_24::solve_a(&input, debug) as i64,
        25 => rust_solutions::day_25::solve_a(&input, debug) as i64,
        _ => 0i64
    };

    let ms_a = a_time.elapsed().as_micros();

    let b_time = Instant::now();

    let b: i64 = match day {
        1 => rust_solutions::day_1::solve_b(&input, debug) as i64,
        2 => rust_solutions::day_2::solve_b(&input, debug) as i64,
        3 => rust_solutions::day_3::solve_b(&input, debug) as i64,
        4 => rust_solutions::day_4::solve_b(&input, debug) as i64,
        5 => rust_solutions::day_5::solve_b(&input, debug) as i64,
        6 => rust_solutions::day_6::solve_b(&input, debug) as i64,
        7 => rust_solutions::day_7::solve_b(&input, debug) as i64,
        8 => rust_solutions::day_8::solve_b(&input, debug) as i64,
        9 => rust_solutions::day_9::solve_b(&input, debug) as i64,
        10 => rust_solutions::day_10::solve_b(&input, debug) as i64,
        11 => rust_solutions::day_11::solve_b(&input, debug) as i64,
        12 => rust_solutions::day_12::solve_b(&input, debug) as i64,
        13 => rust_solutions::day_13::solve_b(&input, debug) as i64,
        14 => rust_solutions::day_14::solve_b(&input, debug) as i64,
        15 => rust_solutions::day_15::solve_b(&input, debug) as i64,
        16 => rust_solutions::day_16::solve_b(&input, debug) as i64,
        17 => rust_solutions::day_17::solve_b(&input, debug) as i64,
        18 => rust_solutions::day_18::solve_b(&input, debug) as i64,
        19 => rust_solutions::day_19::solve_b(&input, debug) as i64,
        20 => rust_solutions::day_20::solve_b(&input, debug) as i64,
        21 => rust_solutions::day_21::solve_b(&input, debug) as i64,
        22 => rust_solutions::day_22::solve_b(&input, debug) as i64,
        23 => rust_solutions::day_23::solve_b(&input, debug) as i64,
        24 => rust_solutions::day_24::solve_b(&input, debug) as i64,
        25 => rust_solutions::day_25::solve_b(&input, debug) as i64,
        _ => 0i64
    };

    let ms_b = b_time.elapsed().as_micros();

    println!("Part 1 -> {a} | {ms_a}μ");
    println!("Part 2 -> {b} | {ms_b}μ");
    println!("Total Runtime | {}μ", ms_a + ms_b)
}
