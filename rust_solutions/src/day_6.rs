use std::collections::HashSet;

#[derive(PartialEq, Hash, Clone, Copy, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[derive(PartialEq, Hash, Clone, Copy, Eq)]
struct Position {
    x: i32, // Column
    y: i32  // Row
}

impl Position {
    fn get_next(&self, direction: &Direction) -> Position {
        match direction {
            Direction::Up => Position { x: self.x, y: self.y - 1 },
            Direction::Down => Position { x: self.x, y: self.y + 1 },
            Direction::Right => Position { x: self.x + 1, y: self.y },
            Direction::Left => Position { x: self.x - 1, y: self.y }
        }
    }
}

#[derive(PartialEq, Hash, Clone, Copy, Eq)]
struct Guard {
    direction: Direction,
    position: Position
}

impl Guard {
    fn new(x: usize, y: usize, direction: Direction) -> Self {
        Self {
            direction,
            position: Position {
                x: x as i32, y: y as i32
            }
        }
    }

    fn get_next(&self) -> Position { self.position.get_next(&self.direction) }
    fn turn(&mut self) {
        self.direction = match self.direction {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up
        }
    }
}

pub fn solve_a(input: &Vec<String>, debug: bool) -> i32 {
    let mut grid: Vec<Vec<bool>> = vec![];
    let mut guard: Guard = Guard::new(0, 0, Direction::Up);
    for (row, line) in input.iter().enumerate() {
        grid.push(line.chars().map(|x| 
            match x {
                '.' => false,
                '#' => true,
                '^' => {
                    guard = Guard::new(line.find('^').unwrap(), row, Direction::Up);
                    false
                },
                '>' => {
                    guard = Guard::new(line.find('>').unwrap(), row, Direction::Right);
                    false
                },
                '<' => {
                    guard = Guard::new(line.find('<').unwrap(), row, Direction::Left);
                    false
                },
                'v' => {
                    guard = Guard::new(line.find('v').unwrap(), row, Direction::Down);
                    false
                },
                _ => false
            }
        ).collect::<Vec<bool>>());
    }

    let max_x: i32 = grid[0].len() as i32 - 1;
    let max_y: i32 = grid.len() as i32 - 1;

    let mut count: usize = 0;

    let mut unique_positions: HashSet<Position> = HashSet::new();

    while guard.position.x >= 0 && guard.position.x <= max_x && guard.position.y >= 0 && guard.position.y <= max_y {

        unique_positions.insert(guard.position);

        let next_pos = guard.get_next();
        if !(next_pos.x < 0 || next_pos.x > max_x || next_pos.y < 0 || next_pos.y > max_y) {
            if grid[next_pos.y as usize][next_pos.x as usize] {
                count += 1;
                guard.turn();
            } else {
                guard.position = guard.get_next();
            }
        } else {
            guard.position = guard.get_next();
        }
    }

    if debug {
        let mut display: Vec<Vec<char>> = input.iter().map(|x| x.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
        for pair in unique_positions.iter() {
            display[pair.y as usize][pair.x as usize] = 'X';
        }
        for line in display {
            println!("{}", line.iter().collect::<String>());
        }
    }
    return unique_positions.len() as i32;
}

fn is_loop(grid: &Vec<Vec<bool>>, start: Guard) -> bool {
    let mut unique_positions: HashSet<Guard> = HashSet::new();
    let max_x: i32 = grid[0].len() as i32 - 1;
    let max_y: i32 = grid.len() as i32 - 1;

    let mut guard = start;

    while guard.position.x >= 0 && guard.position.x <= max_x && guard.position.y >= 0 && guard.position.y <= max_y {
        unique_positions.insert(guard);
        let next_pos = guard.get_next();
        if !(next_pos.x < 0 || next_pos.x > max_x || next_pos.y < 0 || next_pos.y > max_y) {
            if grid[next_pos.y as usize][next_pos.x as usize] {
                guard.turn();
            } else {
                guard.position = guard.get_next();
            }
        } else {
            guard.position = guard.get_next();
        }
    }

    false
}

pub fn solve_b(input: &Vec<String>, debug: bool) -> i32 {
    let mut grid: Vec<Vec<bool>> = vec![];
    let mut guard: Guard = Guard::new(0, 0, Direction::Up);
    for (row, line) in input.iter().enumerate() {
        grid.push(line.chars().map(|x| 
            match x {
                '.' => false,
                '#' => true,
                '^' => {
                    guard = Guard::new(line.find('^').unwrap(), row, Direction::Up);
                    false
                },
                '>' => {
                    guard = Guard::new(line.find('>').unwrap(), row, Direction::Right);
                    false
                },
                '<' => {
                    guard = Guard::new(line.find('<').unwrap(), row, Direction::Left);
                    false
                },
                'v' => {
                    guard = Guard::new(line.find('v').unwrap(), row, Direction::Down);
                    false
                },
                _ => false
            }
        ).collect::<Vec<bool>>());
    }

    let max_x: i32 = grid[0].len() as i32 - 1;
    let max_y: i32 = grid.len() as i32 - 1;

    let mut count: usize = 0;

    let mut unique_positions: HashSet<Position> = HashSet::new();

    while guard.position.x >= 0 && guard.position.x <= max_x && guard.position.y >= 0 && guard.position.y <= max_y {

        unique_positions.insert(guard.position);

        let next_pos = guard.get_next();
        if !(next_pos.x < 0 || next_pos.x > max_x || next_pos.y < 0 || next_pos.y > max_y) {
            if grid[next_pos.y as usize][next_pos.x as usize] {
                count += 1;
                guard.turn();
            } else {
                guard.position = guard.get_next();
            }
        } else {
            guard.position = guard.get_next();
        }
    }

    if debug {
        let mut display: Vec<Vec<char>> = input.iter().map(|x| x.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
        for pair in unique_positions.iter() {
            display[pair.y as usize][pair.x as usize] = 'X';
        }
        for line in display {
            println!("{}", line.iter().collect::<String>());
        }
    }
    return unique_positions.len() as i32;
}
        
