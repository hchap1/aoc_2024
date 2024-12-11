use std::collections::{HashSet,HashMap};

fn check_validity(hash: &HashMap<i32,Vec<i32>>, sequence: &Vec<i32>) -> bool {
    let mut disqualified_numbers: HashSet<i32> = HashSet::new();
    for item in sequence {
        if disqualified_numbers.contains(item) {
            return false;
        }
        if let Some(pre) = hash.get(item) {
            for num in pre {
                disqualified_numbers.insert(*num);
            }
        }
    }
    true
}

pub fn solve_a(input: &Vec<String>, debug: bool) -> i32 {
    // Numbers that need to come before a given number
    let mut hash: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut p2_idx: usize = 0;
    for (idx, line) in input.iter().enumerate() {
        if line.is_empty() { p2_idx = idx; break; }
        let parts = line.split('|').map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        if let Some(pre) = hash.get_mut(&parts[1]) {
            pre.push(parts[0]);
        } else {
            hash.insert(parts[1], vec![parts[0]]);
        }
    }

    let mut total: i32 = 0;

    for idx in p2_idx+1..input.len() {
        let sequence: Vec<i32> = input[idx].split(',').map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        if !check_validity(&hash, &sequence) { continue; }
        total += sequence[(sequence.len() as f32 / 2f32).floor() as usize];
    }

    total
}

fn recursion(hash: &HashMap<i32,Vec<i32>>, target: i32, to_ignore: &mut HashSet<i32>) -> Vec<i32> {
    if let Some(pre) = hash.get(&target) {
        let mut prior_numbers: Vec<i32> = vec![];
        for num in pre {
            if to_ignore.contains(&num) { continue; }
            prior_numbers.push(*num);
            to_ignore.insert(*num);
            prior_numbers.append(&mut recursion(hash, *num, to_ignore));
        }
        return prior_numbers;
    }
    vec![]
}

struct Number {
    value: i32,
    appearances: Vec<i32>
}

pub fn solve_b(input: &Vec<String>, debug: bool) -> i32 {
    // Numbers that need to come before a given number
    let mut hash: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut p2_idx: usize = 0;
    for (idx, line) in input.iter().enumerate() {
        if line.is_empty() { p2_idx = idx; break; }
        let parts = line.split('|').map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        if let Some(pre) = hash.get_mut(&parts[1]) {
            pre.push(parts[0]);
        } else {
            hash.insert(parts[1], vec![parts[0]]);
        }
    }

    let mut total: i32 = 0;

    for idx in p2_idx+1..input.len() {
        let mut new_sequence: Vec<Number> = vec![];
        let sequence: Vec<i32> = input[idx].split(',').map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        if check_validity(&hash, &sequence) { continue; }
        for item in sequence {
            let mut to_ignore: HashSet<i32> = HashSet::new();
            let num: Number = Number { value: item, appearances: recursion(&hash, item, &mut to_ignore) };
            for idx in 0..new_sequence.len() {
            }
            
        }
        if debug {
            let numeric: Vec<i32> = new_sequence.iter().map(|x| x.value).collect::<Vec<i32>>();
            println!("\nNEW SEQUENCE [VALID={}]: ", check_validity(&hash, &numeric));
            for num in &new_sequence {
                print!("{} ", num.value);
            }
            println!("\n");
        }
        total += new_sequence[(new_sequence.len() as f32 / 2f32).floor() as usize].value;
    }

    total
}
       
