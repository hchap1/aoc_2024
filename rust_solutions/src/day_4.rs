fn slice(grid: &Vec<Vec<char>>, start_x: i32, start_y: i32, x: i32, y: i32, len: i32) -> Option<Vec<char>> {
    let mut dump: Vec<char> = vec![];
    for i in 0..len {
        let row = start_y + y * i;
        let col = start_x + x * i;
        if row < 0 || row >= grid.len() as i32 {
            return None;
        }
        if col < 0 || col >= grid.len() as i32 {
            return None;
        }
        dump.push(grid[row as usize][col as usize])
    }
    Some(dump)
}

pub fn solve_a(input: &Vec<String>, debug: bool) -> i32 {
    let mut count: i32 = 0;
    let mut grid: Vec<Vec<char>> = vec![];
    for line in input {
        grid.push(line.chars().collect::<Vec<char>>())
    }

    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            let c = grid[row][col];
            if c == 'X' {
                for x in -1..=1 {
                    for y in -1..=1 {
                        if x == 0 && y == 0 {
                            continue;
                        }
                        let slice: String = match slice(&grid, col as i32, row as i32, x, y, 4) {
                            Some(slice) => slice.into_iter().collect::<String>(),
                            None => continue
                        };
                        if &slice == "XMAS" {
                            count += 1;
                        }

                    }
                }
            }
        }
    }
    return count;
}

pub fn solve_b(input: &Vec<String>, debug: bool) -> i32 {
    let mut count: i32 = 0;
    let mut grid: Vec<Vec<char>> = vec![];
    for line in input {
        grid.push(line.chars().collect::<Vec<char>>())
    }

    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            let c = grid[row][col];
            if c == 'A' {
                if row == 0 || row >= grid.len() - 1 {
                    continue;
                }
                if col == 0 || col >= grid[0].len() - 1 {
                    continue;
                }
                if debug { println!("Non-border A."); }
                let d1 = slice(&grid, col as i32 - 1, row as i32 - 1, 1, 1, 3);
                let d2 = slice(&grid, col as i32 - 1, row as i32 + 1, 1,-1, 3);
                if let (Some(d1), Some(d2)) = (d1, d2) {
                    let s1 = d1.into_iter().collect::<String>();
                    let s2 = d2.into_iter().collect::<String>();
                    let v1 = s1 == "MAS" || s1 == "SAM";
                    let v2 = s2 == "MAS" || s2 == "SAM";
                    if v1 && v2 {
                        count += 1;
                        if debug { println!("{row}, {col}"); }
                    }
                }
            }
        }
    }
    return count;
}
        
