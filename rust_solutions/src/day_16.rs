use std::collections::{BinaryHeap, HashSet};
use std::mem::replace;
use std::cmp::{Ordering, Reverse};
use std::vec;

pub fn solve_a(input: &Vec<String>, debug: bool) -> i32 {
    let mut grid: Vec<Vec<char>> = vec![];
    let mut start: Position = Position::new(0, 0);
    let mut end:   Position = Position::new(0, 0);
    let mut temp: Vec<char> = vec![];
    for line in input.iter().enumerate() {
        for c in line.1.char_indices() {
            temp.push(match c.1 {
                'S' => {
                    start.x = c.0;
                    start.y = line.0;
                    '.'
                }
                'E' => {
                    end.x = c.0;
                    end.y = line.0;
                    '.'
                }
                 _ => c.1
            });
        }
        grid.push(replace(&mut temp, vec![]))
    }

    let (path, cost) = match astar(&grid, start, end) {
        Some(path) => path,
        None => return 0i32
    };

    let mut pdx: i32 = 1;
    let mut pdy: i32 = 0;
    let mut turns: i32 = 0;
    let mut moves: i32 = 0;
    
    for n in 1..path.len() {
        let dx: i32 = path[n].x as i32 - path[n - 1].x as i32;
        let dy: i32 = path[n].y as i32 - path[n - 1].y as i32;

        if dx != pdx || dy != pdy {
            turns += 1;
        }

        moves += 1;
    }

    turns * 1000 + moves
}

pub fn solve_b(input: &Vec<String>, debug: bool) -> i32 {
    return 0i32;
}

#[derive(PartialEq, Eq, Hash, Clone)]
enum Direction {
    N,
    E,
    S,
    W,
}

impl Direction {
    fn from_movement(dx: i32, dy: i32) -> Self {
        match (dx, dy) {
            (0, -1) => Direction::N,
            (1, 0) => Direction::E,
            (0, 1) => Direction::S,
            (-1, 0) => Direction::W,
            _ => panic!("Invalid movement"),
        }
    }
}

#[derive(Clone, Eq)]
struct Node {
    parent: Option<usize>,
    position: Position,
    g: usize,
    h: usize,
    f: usize,
    direction: Option<Direction>,
}

#[derive(Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Position { x, y }
    }

    fn get_adjacent(&self, tilemap: &Vec<Vec<char>>) -> Vec<Position> {
        let max_x = tilemap[0].len();
        let max_y = tilemap.len();
        let mut adjacent = vec![];

        if self.x > 0 {
            adjacent.push(Position { x: self.x - 1, y: self.y });
        }
        if self.y > 0 {
            adjacent.push(Position { x: self.x, y: self.y - 1 });
        }
        if self.x < max_x - 1 {
            adjacent.push(Position { x: self.x + 1, y: self.y });
        }
        if self.y < max_y - 1 {
            adjacent.push(Position { x: self.x, y: self.y + 1 });
        }
        adjacent
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position && self.direction == other.direction
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.f.cmp(&self.f)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn walkable(tilemap: &Vec<Vec<char>>, position: &Position) -> bool {
    tilemap[position.y][position.x] != '#'
}

pub fn astar(tilemap: &Vec<Vec<char>>, start: Position, end: Position) -> Option<(Vec<Position>, usize)> {
    let start_node = Node {
        parent: None,
        position: start.clone(),
        g: 0,
        h: 0,
        f: 0,
        direction: None,
    };
    let mut open_list: BinaryHeap<Node> = BinaryHeap::new();
    let mut closed_set: HashSet<(Position, Option<Direction>)> = HashSet::new();
    let mut closed_list: Vec<Node> = vec![];

    open_list.push(start_node);

    while let Some(current_node) = open_list.pop() {
        if current_node.position == end {
            // Reconstruct path
            let mut path = vec![];
            let mut current_path_node: Option<&Node> = Some(&current_node);

            while let Some(current) = current_path_node {
                path.push(current.position.clone());
                match current.parent {
                    Some(index) => {
                        current_path_node = Some(&closed_list[index]);
                    }
                    None => break,
                }
            }
            path.reverse();
            return Some((path, current_node.g));
        }

        closed_set.insert((current_node.position.clone(), current_node.direction.clone()));
        closed_list.push(current_node.clone());

        for neighbor in current_node.position.get_adjacent(tilemap) {
            if !walkable(tilemap, &neighbor) {
                continue;
            }

            let direction = if let Some(dir) = &current_node.direction {
                Direction::from_movement(
                    neighbor.x as i32 - current_node.position.x as i32,
                    neighbor.y as i32 - current_node.position.y as i32,
                )
            } else {
                Direction::from_movement(
                    neighbor.x as i32 - current_node.position.x as i32,
                    neighbor.y as i32 - current_node.position.y as i32,
                )
            };

            let is_turn = match &current_node.direction {
                Some(dir) if dir != &direction => true,
                _ => false,
            };

            let g = current_node.g + if is_turn { 1000 } else { 1 };
            let h = (neighbor.x as isize - end.x as isize).abs() as usize
                + (neighbor.y as isize - end.y as isize).abs() as usize;

            let new_node = Node {
                parent: Some(closed_list.len() - 1),
                position: neighbor.clone(),
                g,
                h,
                f: g + h,
                direction: Some(direction),
            };

            if closed_set.contains(&(neighbor.clone(), new_node.direction.clone())) {
                continue;
            }

            if let Some(existing) = open_list.iter().find(|n| n.position == neighbor && n.direction == new_node.direction) {
                if existing.g > new_node.g {
                    open_list.push(new_node);
                }
            } else {
                open_list.push(new_node);
            }
        }
    }
    None
}
