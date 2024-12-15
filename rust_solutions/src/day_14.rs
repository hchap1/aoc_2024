use std::io::{self, Write};
use std::thread::sleep;
use std::time::Duration;

const WIDTH: i32 = 101;
const HEIGHT: i32 = 103;

struct Robot {
    x: i32,
    y: i32,
    xv: i32,
    yv: i32
}

impl Robot {
    fn debug_print(&self) {
        println!("Robot at {}, {} with velocity {} {}", self.x, self.y, self.xv, self.yv);
    }

    fn step(&mut self) {
        self.x += self.xv;
        self.y += self.yv;
        while self.x >= WIDTH { self.x -= WIDTH; }
        while self.y >= HEIGHT { self.y -= HEIGHT; }
        while self.x < 0 { self.x += WIDTH; }
        while self.y < 0 { self.y += HEIGHT; }
    }
}

pub fn solve_a(input: &Vec<String>, debug: bool) -> i32 {
    let mut robots: Vec<Robot> = vec![];
    for line in input {
        let cleaned_line: Vec<i32> = line.strip_prefix("p=").unwrap().split(" v=").map(|x| x.split(",").map(|y| y.parse::<i32>().unwrap()).collect::<Vec<i32>>()).flatten().collect();
        robots.push(Robot {
            x: cleaned_line[0],
            y: cleaned_line[1],
            xv: cleaned_line[2],
            yv: cleaned_line[3],
        });
    }

    for _ in 0..100{
        for robot in &mut robots {
            robot.step();
        }
    }

    let mid_x = (WIDTH - 1) / 2;
    let mid_y = (HEIGHT - 1) / 2;

    let mut a: i32 = 0;
    let mut b: i32 = 0;
    let mut c: i32 = 0;
    let mut d: i32 = 0;

    // a b
    // c d

    for robot in &robots {
        if robot.x < mid_x && robot.y < mid_y { a += 1; continue; }
        if robot.x > mid_x && robot.y < mid_y { b += 1; continue; }
        if robot.x < mid_x && robot.y > mid_y { c += 1; continue; }
        if robot.x > mid_x && robot.y > mid_y { d += 1; continue; }
    }
    a * b * c * d
}

fn calculate_deviation(robots: &Vec<Robot>) -> f32 {
    let mut tx: f32 = 0f32;
    let mut ty: f32 = 0f32;

    for robot in robots {
        tx += robot.x as f32;
        ty += robot.y as f32;
    }

    let ax: f32 = tx / robots.len() as f32;
    let ay: f32 = ty / robots.len() as f32;

    let mut dxt: f32 = 0f32;
    let mut dyt: f32 = 0f32;

    for robot in robots {
        dxt += (robot.x as f32 - ax).abs();
        dyt += (robot.y as f32 - ay).abs();
    }

    let axd = dxt / robots.len() as f32;
    let ayd = dyt / robots.len() as f32;

    (axd * axd + ayd * ayd).sqrt()
}

pub fn solve_b(input: &Vec<String>, debug: bool) -> i32 {
    let mut robots: Vec<Robot> = vec![];
    for line in input {
        let cleaned_line: Vec<i32> = line.strip_prefix("p=").unwrap().split(" v=").map(|x| x.split(",").map(|y| y.parse::<i32>().unwrap()).collect::<Vec<i32>>()).flatten().collect();
        robots.push(Robot {
            x: cleaned_line[0],
            y: cleaned_line[1],
            xv: cleaned_line[2],
            yv: cleaned_line[3],
        });

        if debug {
            robots.last().unwrap().debug_print();
        }
    }

    let mut min_deviation: Option<f32> = None;

    for i in 0..100000 {
        for robot in &mut robots {
            robot.step();
        }

        let new_deviation = calculate_deviation(&robots);

        if min_deviation == None {
            min_deviation = Some(new_deviation);
        } else if let Some(num) = min_deviation {
            if new_deviation <= num {
                min_deviation = Some(new_deviation);
            } else {
                continue;
            }
        } else {
            continue;
        }

        let mut robots_per_line: Vec<usize> = vec![0; HEIGHT as usize];

        if debug {
            let mut print_grid: Vec<Vec<usize>> = vec![vec![0; WIDTH as usize]; HEIGHT as usize];

            for robot in &robots {
                print_grid[robot.y as usize][robot.x as usize] += 1;
                robots_per_line[robot.y as usize] += 1;
            }

            for line in print_grid.iter().enumerate() {
                if robots_per_line[line.0] < 3 {
                    continue;
                }
                for c in line.1.iter().enumerate() {
                    print!("{}", if *c.1 == 0 { String::from('.') } else { c.1.to_string() });
                }
                if line.0 != print_grid.len() - 1 { print!("\n"); }
            }
            println!("\x1B[{}A", print_grid.len());
            sleep(Duration::from_millis(50));
        }
    }

    let mid_x = (WIDTH - 1) / 2;
    let mid_y = (HEIGHT - 1) / 2;

    println!("\x1B[{}B", HEIGHT + 1);

    let mut a: i32 = 0;
    let mut b: i32 = 0;
    let mut c: i32 = 0;
    let mut d: i32 = 0;

    // a b
    // c d

    for robot in &robots {
        if robot.x < mid_x && robot.y < mid_y { a += 1; continue; }
        if robot.x > mid_x && robot.y < mid_y { b += 1; continue; }
        if robot.x < mid_x && robot.y > mid_y { c += 1; continue; }
        if robot.x > mid_x && robot.y > mid_y { d += 1; continue; }
    }

    a * b * c * d
}
        
