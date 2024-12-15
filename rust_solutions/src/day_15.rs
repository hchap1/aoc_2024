#[derive(PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Right,
    Left
}

fn push(grid: &mut Vec<Vec<char>>, x: usize, y: usize, direction: &Direction) -> bool {
    let (nx, ny) = match direction {
        Direction::Up => (x, y - 1),
        Direction::Down => (x, y + 1),
        Direction::Right => (x + 1, y),
        Direction::Left => (x - 1, y)
    };

    if grid[ny][nx] == '.' {
        grid[ny][nx] = grid[y][x];
        grid[y][x] = '.';
        return true;
    } else if grid[ny][nx] == 'O' {
        if push(grid, nx, ny, direction) {
            grid[ny][nx] = grid[y][x];
            grid[y][x] = '.';
            return true;
        } else {
            return false;
        }
    } else {
        return false;
    }
}

fn push_large(grid: &mut Vec<Vec<char>>, x: usize, y: usize, direction: &Direction) -> bool {
    let (nx, ny) = match direction {
        Direction::Up => (x, y - 1),
        Direction::Down => (x, y + 1),
        Direction::Right => (x + 1, y),
        Direction::Left => (x - 1, y)
    };
    if *direction == Direction::Up || *direction == Direction::Down {

        // Always target left bracket


        // If first tile, make sure targetting left bracket
        if grid[y][x] == '.' {
            if grid[ny][nx] == '.' {
                return true;
            } else if grid[ny][nx] == '[' {
                return push_large(grid, nx, ny, direction);
            } else if grid[ny][nx] == ']' {
                return push_large(grid, nx - 1, ny, direction);
            } else {
                return false;
            }
        }

        // If open space in front, true
        if grid[ny][nx] == '.' && grid[ny][nx + 1] == '.' {
            grid[ny][nx] = '[';
            grid[ny][nx + 1] = ']';
            grid[y][x] = '.';
            grid[y][x + 1] = '.';
            return true;
        } 

        // If there is another left bracket in front, just push it
        if grid[ny][nx] == '[' {
            if push_large(grid, nx, ny, direction) {
                grid[ny][nx] = '[';
                grid[ny][nx + 1] = ']';
                grid[y][x] = '.';
                grid[y][x + 1] = '.';
                return true;
            } else {
                return false;
            }
        }

        // If right bracket in front and left bracket in front and to the right, then we need to push both
        if grid[ny][nx] == ']' && grid[ny][nx + 1] == '[' {
            let grid_backup = grid.clone();
            if push_large(grid, nx - 1, ny, direction) && push_large(grid, nx + 1, ny, direction) {
                grid[ny][nx] = '[';
                grid[ny][nx + 1] = ']';
                grid[y][x] = '.';
                grid[y][x + 1] = '.';
                return true;
            } else {
                *grid = grid_backup;
                return false;
            }
        }

        // If there is just one to the left
        if grid[ny][nx + 1] == '.' && grid[ny][nx] == ']' {
            if push_large(grid, nx - 1, ny, direction) {
                grid[ny][nx] = '[';
                grid[ny][nx + 1] = ']';
                grid[y][x] = '.';
                grid[y][x + 1] = '.';
                return true;
            } else {
                return false;
            }
        }

        // If there is just one to the right
        if grid[ny][nx] == '.' && grid[ny][nx + 1] == '[' {
            if push_large(grid, nx + 1, ny, direction) {
                grid[ny][nx] = '[';
                grid[ny][nx + 1] = ']';
                grid[y][x] = '.';
                grid[y][x + 1] = '.';
                return true;
            } else {
                return false;
            }
        }

        false


    } else {
        if grid[ny][nx] == '.' {
            grid[ny][nx] = grid[y][x];
            grid[y][x] = '.';
            return true;
        } else if grid[ny][nx] == '[' || grid[ny][nx] == ']' {
            if push_large(grid, nx, ny, direction) {
                grid[ny][nx] = grid[y][x];
                grid[y][x] = '.';
                return true;
            } else {
                return false;
            }
        } else {
            return false;
        }
    }
}

fn char_to_dir(c: char) -> Direction {
    match c {
        '^' => Direction::Up,
        '>' => Direction::Right,
        'v' => Direction::Down,
        '<' => Direction::Left,
         _  => Direction::Up
    }
}

fn print_grid(grid: &Vec<Vec<char>>, x: usize, y: usize, _last_direction: &Direction) {
    let mut print_grid = grid.clone();
    print_grid[y][x] = '^';
    for line in print_grid.iter().enumerate() {
        print!("{}", line.1.iter().collect::<String>());
        if line.0 != print_grid.len() - 1 {
            print!("\n");
        }
    }
}

pub fn solve_a(input: &Vec<String>, debug: bool) -> i32 {
    let mut empty_idx: usize = 0;
    for line in input.iter().enumerate() {
        if line.1.is_empty() {
            empty_idx = line.0;
            break;
        }
    }
    let mut grid: Vec<Vec<char>> = vec![];
    let mut x: usize = 0;
    let mut y: usize = 0;
    for line in input[0..empty_idx].iter().enumerate() {
        grid.push(line.1.char_indices().map(|a| if a.1 == '@' {
            y = line.0;
            x = a.0;
            '.'
        } else { a.1 }).collect());
    }
    let mut instructions: Vec<Direction> = vec![];
    
    for line in input[empty_idx+1..input.len()].iter() {
        instructions.append(&mut line.chars().map(|x| char_to_dir(x)).collect());
    }

    let mut count: f32 = 0f32;

    for instruction in &instructions {
        count += 1f32;
        if push(&mut grid, x, y, instruction) {
            match instruction {
                Direction::Up => y -= 1,
                Direction::Down => y += 1,
                Direction::Right => x += 1,
                Direction::Left => x -= 1
            }
        }

        if debug {
            print_grid(&grid, x, y, instruction);
            print!("          <======== PART 1 ========> {}%", ((count / instructions.len() as f32) * 100f32).round());
            println!("\x1B[{}A", grid.len() + 2);
        }
    }

    let mut gps: i32 = 0;

    for line in grid.iter().enumerate() {
        for thing in line.1.iter().enumerate() {
            if *thing.1 == 'O' {
                gps += (line.0 as i32 * 100i32) + thing.0 as i32;
            }
        }
    }

    gps
}

pub fn solve_b(input: &Vec<String>, debug: bool) -> i32 {
    let mut empty_idx: usize = 0;
    for line in input.iter().enumerate() {
        if line.1.is_empty() {
            empty_idx = line.0;
            break;
        }
    }
    let mut grid: Vec<Vec<char>> = vec![];
    let mut x: usize = 0;
    let mut y: usize = 0;
    for line in input[0..empty_idx].iter().enumerate() {
        let mut temp: Vec<char> = vec![];
        for item in line.1.char_indices() {
            match item.1 {
                'O' => {
                    temp.push('[');
                    temp.push(']');
                }
                '#' => {
                    temp.push('#');
                    temp.push('#');
                }
                '.' => {
                    temp.push('.');
                    temp.push('.');
                }
                 _  => {
                    x = item.0 * 2;
                    y = line.0;
                    temp.push('.');
                    temp.push('.');
                }
            }
        }
        grid.push(temp);
    }

    let mut instructions: Vec<Direction> = vec![];
    
    for line in input[empty_idx+1..input.len()].iter() {
        instructions.append(&mut line.chars().map(|x| char_to_dir(x)).collect());
    }

    let mut count: f32 = 0f32;

    for instruction in &instructions {
        count += 1f32;
        if push_large(&mut grid, x, y, instruction) {
            match instruction {
                Direction::Up => y -= 1,
                Direction::Down => y += 1,
                Direction::Right => x += 1,
                Direction::Left => x -= 1
            }
        }

        if debug {
            print_grid(&grid, x, y, instruction);
            print!("          <======== PART 2 ========> {}%", ((count / instructions.len() as f32) * 100f32).round());
            println!("\x1B[{}A", grid.len() + 2);
        }
    }

    let mut gps: i32 = 0;

    for line in grid.iter().enumerate() {
        for thing in line.1.iter().enumerate() {
            if *thing.1 == '[' {
                gps += (line.0 as i32 * 100i32) + thing.0 as i32;
            }
        }
    }

    if debug { println!("\x1B[{}B", grid.len() + 3); }
    
    gps
}
