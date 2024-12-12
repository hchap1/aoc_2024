use std::{cmp::max, cmp::min, collections::{HashMap, HashSet}};

pub fn solve_a(input: &Vec<String>, debug: bool) -> i32 {
    // Scan each set of frequencies into a hashmap containing frequency then location
    // Iterate over each unique pair of antenna for each given frequency
    // Add locations to a hashset
    let mut position_by_frequency: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    for (row, line) in input.iter().enumerate() {
        for (col, c) in line.char_indices() {
            if c != '.' {
                if let Some(count) = position_by_frequency.get_mut(&c) {
                    count.push((row, col));
                } else { position_by_frequency.insert(c, vec![(row, col)]); }
            }
        }
    }

    let mut antinodes: HashSet<(usize, usize)> = HashSet::new();

    for freq in position_by_frequency.keys() {
        let locations: Vec<(usize, usize)> = position_by_frequency.get(freq).unwrap().to_vec();
        for outer in 0..locations.len() {
            for inner in outer+1..locations.len() {
                if debug { println!("Comparing {outer} to {inner}"); }
                let a = locations[outer];
                let b = locations[inner];
                let dx: i32 = a.0 as i32 - b.0 as i32;
                let dy: i32 = a.1 as i32 - b.1 as i32;
                let an1: (i32, i32) = (a.0 as i32 + dx, a.1 as i32 + dy);
                let an2: (i32, i32) = (b.0 as i32 - dx, b.1 as i32 - dy);
                if an1.0 >= 0 && an1.0 < input.len() as i32 && an1.1 >= 0 && an1.1 < input[0].len() as i32 {
                    antinodes.insert((an1.0 as usize, an1.1 as usize));
                }
                if an2.0 >= 0 && an2.0 < input.len() as i32 && an2.1 >= 0 && an2.1 < input[0].len() as i32 {
                    antinodes.insert((an2.0 as usize, an2.1 as usize));
                }
            }
        }
    }

    if debug {
        let mut grid: Vec<Vec<char>> = vec![vec!['.'; input[0].len()]; input.len()];
        for coord in antinodes.iter() {
            grid[coord.0][coord.1] = '#';
        }
        for a in position_by_frequency {
            for pos in a.1 {
                grid[pos.0][pos.1] = a.0;
            }
        }
        for line in grid {
            println!("{}", line.iter().collect::<String>());
        }
    }

    antinodes.len() as i32
}

fn smallest_step(step: (i32, i32)) -> (i32, i32) {
    let mut x = step.0 as f32;
    let mut y = step.1 as f32;
    while (x / 2f32).round() == x / 2f32 && (y / 2f32).round() == y / 2f32 {
        x /= 2f32;
        y /= 2f32;
    }
    (x as i32, y as i32)
}

fn usizeify_coord(coord: (i32, i32)) -> (usize, usize) {
    (coord.0 as usize, coord.1 as usize)
}

pub fn solve_b(input: &Vec<String>, debug: bool) -> i32 {
    // Scan each set of frequencies into a hashmap containing frequency then location
    // Iterate over each unique pair of antenna for each given frequency
    // Add locations to a hashset
    let mut position_by_frequency: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    for (row, line) in input.iter().enumerate() {
        for (col, c) in line.char_indices() {
            if c != '.' {
                if let Some(count) = position_by_frequency.get_mut(&c) {
                    count.push((row, col));
                } else { position_by_frequency.insert(c, vec![(row, col)]); }
            }
        }
    }

    let mut antinodes: HashSet<(usize, usize)> = HashSet::new();

    for freq in position_by_frequency.keys() {
        let locations: Vec<(usize, usize)> = position_by_frequency.get(freq).unwrap().to_vec();
        for outer in 0..locations.len() {
            for inner in outer+1..locations.len() {
                let max_x = input.len() as i32;
                let max_y = input[0].len() as i32;
                let mut a = (locations[outer].0 as i32, locations[outer].1 as i32);
                let mut b = (locations[inner].0 as i32, locations[inner].1 as i32);
                let dx: i32 = a.0 - b.0;
                let dy: i32 = a.1 - b.1;
                let step = smallest_step((dx, dy));

                while b.0 >= 0 && b.1 >= 0 && b.0 < max_x && b.1 < max_y {
                    antinodes.insert(usizeify_coord(b));
                    b.0 += step.0;
                    b.1 += step.1;
                }

                while a.0 >= 0 && a.1 >= 0 && a.0 < max_x && a.1 < max_y {
                    antinodes.insert(usizeify_coord(a));
                    a.0 -= step.0;
                    a.1 -= step.1;
                }
            }
        }
    }

    if debug {
        let mut grid: Vec<Vec<char>> = vec![vec!['.'; input[0].len()]; input.len()];
        for coord in antinodes.iter() {
            grid[coord.0][coord.1] = '#';
        }
        for a in position_by_frequency {
            for pos in a.1 {
                grid[pos.0][pos.1] = a.0;
            }
        }
        println!("---------------------");
        for line in grid {
            println!("{}", line.iter().collect::<String>());
        }
    }

    antinodes.len() as i32
}
        
