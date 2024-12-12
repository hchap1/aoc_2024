use std::collections::HashSet;

fn rh(grid: &Vec<Vec<u8>>, position: (i32, i32)) -> u8 {
    grid[position.0 as usize][position.1 as usize]
}

fn ib(grid: &Vec<Vec<u8>>, position: (i32, i32)) -> bool {
    position.0 >= 0 && position.0 < grid.len() as i32 && position.1 >= 0 && position.1 < grid[0].len() as i32
}

fn recursion_march(grid: &Vec<Vec<u8>>, position: (i32, i32), set: &mut HashSet<(i32, i32)>) {
    let current_height: u8 = rh(grid, position);
    if current_height == 9 {
        set.insert(position);
    }
    for row in -1i32..=1i32 {
        for col in -1i32..=1i32 {
            if (row + col).abs() != 1 {
                continue;
            }
            
            let new_position = (position.0 + row, position.1 + col);

            if !ib(grid, new_position) {
                continue;
            }
            
            if rh(grid, new_position) == current_height + 1 {
                recursion_march(grid, new_position, set);
            }
        }
    }
}

fn finite_recursion_march(grid: &Vec<Vec<u8>>, position: (i32, i32), set: &mut HashSet<Vec<(i32, i32)>>, mut path: Vec<(i32, i32)>) {
    path.push(position);
    let current_height: u8 = rh(grid, position);
    if current_height == 9 {
        set.insert(path);
        return;
    }
    for row in -1i32..=1i32 {
        for col in -1i32..=1i32 {
            if (row + col).abs() != 1 {
                continue;
            }
            
            let new_position = (position.0 + row, position.1 + col);

            if !ib(grid, new_position) {
                continue;
            }
            
            if rh(grid, new_position) == current_height + 1 {
                finite_recursion_march(grid, new_position, set, path.clone());
            }
        }
    }
}

pub fn solve_a(input: &Vec<String>, debug: bool) -> i32 {
    let mut grid: Vec<Vec<u8>> = vec![];
    for line in input {
        grid.push(line.chars().map(|x| x.to_digit(10u32).unwrap() as u8).collect::<Vec<u8>>());
    }

    let mut result: i32 = 0;
    
    for row in grid.iter().enumerate() {
        for col in row.1.iter().enumerate() {
            if grid[row.0][col.0] != 0 { continue; }
            let mut reachable_peaks: HashSet<(i32, i32)> = HashSet::new();
            recursion_march(&grid, (row.0 as i32, col.0 as i32), &mut reachable_peaks);
            result += reachable_peaks.len() as i32;
            if debug {
                println!("Identified trailhead with rating of {}", reachable_peaks.len());
            }
        }
    }
    result
}

pub fn solve_b(input: &Vec<String>, debug: bool) -> i32 {
    let mut grid: Vec<Vec<u8>> = vec![];
    for line in input {
        grid.push(line.chars().map(|x| x.to_digit(10u32).unwrap() as u8).collect::<Vec<u8>>());
    }

    let mut result: i32 = 0;
    
    for row in grid.iter().enumerate() {
        for col in row.1.iter().enumerate() {
            if grid[row.0][col.0] != 0 { continue; }
            let mut reachable_peaks: HashSet<Vec<(i32, i32)>> = HashSet::new();
            finite_recursion_march(&grid, (row.0 as i32, col.0 as i32), &mut reachable_peaks, vec![]);
            result += reachable_peaks.len() as i32;
            if debug {
                println!("Identified trailhead with rating of {}", reachable_peaks.len());
            }
        }
    }
    result
}
        
