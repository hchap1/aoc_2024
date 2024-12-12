use std::collections::HashMap;
const LOG_2024: f64 = 3.30621; // This is log base 10 of 2024
const MAXITERATIONS: i64 = 75;

fn blink(stones: &mut Vec<i64>) {
    let mut new_stones: Vec<i64> = vec![];
    for stone in &*stones {
        if *stone == 0 {
            new_stones.push(1);
        } else if stone.to_string().len() % 2 == 0 {
            let stone_string: String = stone.to_string();
            let stone_digits: usize  = stone_string.len();
            let a = &stone_string[0..stone_digits / 2];
            let b = &stone_string[stone_digits / 2..stone_digits];
            new_stones.push(a.parse::<i64>().unwrap());
            new_stones.push(b.parse::<i64>().unwrap());
        } else {
            new_stones.push(*stone * 2024);
        }
    }
    *stones = new_stones;
}

fn find_iterations_to_even_number(n: f64, depth: i64) -> i64 {
    let log_n = n.log10();
    let mut x: i64 = 0;
    while x + depth < MAXITERATIONS {
        let value = (log_n + LOG_2024 * x as f64).floor() as i64;
        if value % 2 == 1 {
            return x;
        }
        x += 1;
    }
    MAXITERATIONS
}

fn recursion(stone: i64, depth: i64, precompute: &Vec<i64>, pcs: &mut HashMap<(i64, i64), i64>) -> i64 {
    if depth >= MAXITERATIONS { 
        return 1;
    }
    if let Some(solution) = pcs.get(&(stone, depth)) {
        return *solution;
    }
    if stone == 0 {
        let solution = recursion(1, depth + 1, precompute, pcs);
        pcs.insert((stone, depth), solution);
        solution
    } else {
        let num_digits = (stone as f64).log10().floor() as usize + 1; // Number of digits in base 10

        if num_digits % 2 == 0 {
            let mid = 10i64.pow((num_digits / 2) as u32);
            let a = stone / mid;
            let b = stone % mid;
            let solution = recursion(a, depth + 1, precompute, pcs) + recursion(b, depth + 1, precompute, pcs);
            pcs.insert((stone, depth), solution);
            solution
        } else {
            let solution = recursion(stone * 2024, depth + 1, precompute, pcs);
            pcs.insert((stone, depth), solution);
            return solution;
            /*
            let iterations = find_iterations_to_even_number(stone as f64, depth);
            if stone + iterations >= MAXITERATIONS {
                return 1;
            } else {
                let solution = recursion(precompute[iterations as usize] * stone, depth + iterations, precompute, pcs);
                pcs.insert((stone, depth), solution);
                solution
            }
            */
        }
    }
}

pub fn solve_a(input: &Vec<String>, debug: bool) -> i32 {
    let mut stones: Vec<i64> = input[0].split(' ').map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>();
    for _ in 0..25 {
        blink(&mut stones);
    }
    if debug {
        println!("{}", stones.iter().map(|x| x.to_string()).collect::<String>());
    }
    stones.len() as i32
}

pub fn solve_b(input: &Vec<String>, debug: bool) -> i64 {
    // Precompute powers of 2024
    let mut powers: Vec<i64> = vec![];
    for i in 0..5 {
        powers.push(2024i64.pow(i as u32));
    }

    let mut previously_determined_solutions: HashMap<(i64, i64), i64> = HashMap::new();
    let stones: Vec<i64> = input[0].split(' ').map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>();
    let mut total: i64 = 0;
    for stone in stones {
        total += recursion(stone, 0, &powers, &mut previously_determined_solutions);
    }
    total
}
        
