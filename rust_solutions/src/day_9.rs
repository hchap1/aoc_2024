use std::thread::sleep;
use std::io::{self, Write};
use std::time::Duration;

fn char_to_usize(c: char) -> usize {
    if c.is_digit(10) {
        c as usize - '0' as usize
    } else {
        0
    }
}

fn get_leftmost_free_space(start: usize, disk_map: &Vec<String>) -> usize {
    for c in start..disk_map.len() {
        if disk_map[c] == "." {
            return c;
        }
    }
    0
}

fn get_rightmost_data(start: usize, disk_map: &Vec<String>) -> usize {
    for c in (0..start).rev() {
        if disk_map[c] != "." {
            return c;
        }
    }
    0
}

pub fn solve_a(input: &Vec<String>, debug: bool) -> i64 {
    let input = input[0].clone();
    let mut id: u32 = 0;
    let mut raw_blocks: Vec<String> = vec![];
    let mut is_data = true;
    let mut num_blocks: usize = 0;

    for c in input.chars() {
        if is_data {
            is_data = false;
            raw_blocks.append(&mut vec![id.to_string(); char_to_usize(c)]);
            num_blocks += char_to_usize(c);
            id += 1;
        } else {
            is_data = true;
            raw_blocks.append(&mut vec![String::from("."); char_to_usize(c)]);
        }
    }

    let mut sum: i64 = 0;

    if debug {
        println!("Num blocks: {num_blocks}");
    }

    let mut search_start: usize = 0;
    let mut search_end: usize = raw_blocks.len();

    while search_start < num_blocks {
        let a = get_rightmost_data(search_end, &raw_blocks);
        let b = get_leftmost_free_space(search_start, &raw_blocks);
        search_start = b;
        search_end = a;
        print!("\r{b} | {a}");
        io::stdout().flush().unwrap();
        raw_blocks.swap(a, b);
    }

    raw_blocks.swap(search_end, search_start);

    for idx in 0..search_start {
        sum += idx as i64 * match raw_blocks[idx].parse::<i64>() {
            Ok(val) => val,
            Err(_) => {
                println!("Failed to parse item {} at {idx}.", raw_blocks[idx]);
                0i64
            }
        }
    }

    if debug {
        println!("Fragmented blocks: {}", raw_blocks.iter().map(|x| x.as_str()).collect::<String>());
    }

    sum
}

fn find_first_contiguous_space(start: usize, end: usize, size: usize, raw_blocks: &Vec<String>) -> Option<usize> {
    for idx in start..end {
        if raw_blocks[idx] == "." {
            if idx + size >= raw_blocks.len() - size { return None; }
            let slice = (&raw_blocks[idx..idx+size]).to_vec();
            let mut valid = true;
            for c in slice {
                if c != "." { valid = false; break; }
            }
            if valid { return Some(idx); }
        }
    }
    None
}

fn find_memory_block(start: usize, id: usize, raw_blocks: &Vec<String>) -> Option<(usize, usize)> { // start idx, size
    let mut end: usize = 0;
    let mut found = false;
    for idx in (0..start).rev() {
        if match raw_blocks[idx].parse::<usize>() {
            Ok(val) => val == id,
            Err(_) => false
        } {
            if !found { end = idx; }
            found = true;
        } else if found {
            return Some((idx + 1, end - idx));
        }
    }
    None
}

pub fn solve_b(input: &Vec<String>, debug: bool) -> i64 {
    let input = input[0].clone();
    let mut id: u32 = 0;
    let mut raw_blocks: Vec<String> = vec![];
    let mut is_data = true;
    let mut num_blocks: usize = 0;

    for c in input.chars() {
        if is_data {
            is_data = false;
            raw_blocks.append(&mut vec![id.to_string(); char_to_usize(c)]);
            num_blocks += char_to_usize(c);
            id += 1;
        } else {
            is_data = true;
            raw_blocks.append(&mut vec![String::from("."); char_to_usize(c)]);
        }
    }

    let mut sum: i64 = 0;

    if debug {
        println!("Num blocks: {num_blocks}");
    }

    let mut search_end: usize = raw_blocks.len();
    let mut target: usize = raw_blocks[get_rightmost_data(raw_blocks.len(), &raw_blocks)].parse::<usize>().unwrap() + 1;

    while target > 0 {
        if debug {
            println!("Fragmented blocks: {}", raw_blocks.iter().map(|x| x.as_str()).collect::<String>());
        }
        target -= 1;
        let (idx_a, size) = match find_memory_block(search_end, target, &raw_blocks) {
            Some(val) => val,
            None => continue
        };
        let idx_b = match find_first_contiguous_space(0, idx_a, size, &raw_blocks) {
            Some(val) => val,
            None => continue
        };

        for offset in 0..size {
            raw_blocks.swap(idx_a + offset, idx_b + offset);
        }

        search_end = idx_a;
    }

    for idx in 0..raw_blocks.len() {
        sum += idx as i64 * match raw_blocks[idx].parse::<i64>() {
            Ok(val) => val,
            Err(_) => {
                println!("Failed to parse item {} at {idx}.", raw_blocks[idx]);
                0i64
            }
        }
    }

    if debug {
        println!("Fragmented blocks: {}", raw_blocks.iter().map(|x| x.as_str()).collect::<String>());
    }

    sum
}
        
